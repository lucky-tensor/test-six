// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::transport::{ConnectionOrigin, Transport};
use futures::{future::Future, stream::Stream};
use parity_multiaddr::Multiaddr;
use pin_project::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// An [`AndThen`] is a transport which applies a closure (F) to all connections created by the
/// underlying transport.
pub struct AndThen<T, F> {
    transport: T,
    function: F,
}

impl<T, F> AndThen<T, F> {
    pub(crate) fn new(transport: T, function: F) -> Self {
        Self {
            transport,
            function,
        }
    }
}

impl<T, F, Fut, O> Transport for AndThen<T, F>
where
    T: Transport,
    F: FnOnce(T::Output, ConnectionOrigin) -> Fut + Send + Unpin + Clone,
    // Pin the error types to be the same for now
    // TODO don't require the error types to be the same
    Fut: Future<Output = Result<O, T::Error>> + Send,
{
    type Output = O;
    type Error = T::Error;
    type Listener = AndThenStream<T::Listener, F>;
    type Inbound = AndThenFuture<T::Inbound, Fut, F>;
    type Outbound = AndThenFuture<T::Outbound, Fut, F>;

    fn listen_on(&self, addr: Multiaddr) -> Result<(Self::Listener, Multiaddr), Self::Error> {
        let (listener, addr) = self.transport.listen_on(addr)?;
        let listener = AndThenStream::new(listener, self.function.clone());

        Ok((listener, addr))
    }

    fn dial(&self, addr: Multiaddr) -> Result<Self::Outbound, Self::Error> {
        let fut = self.transport.dial(addr)?;
        let origin = ConnectionOrigin::Outbound;
        let f = self.function.clone();

        Ok(AndThenFuture::new(fut, f, origin))
    }
}

/// Listener stream returned by [listen_on](Transport::listen_on) on an AndThen transport.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct AndThenStream<St, F> {
    #[pin]
    stream: St,
    f: F,
}

impl<St, Fut1, O1, Fut2, O2, E, F> AndThenStream<St, F>
where
    St: Stream<Item = Result<(Fut1, Multiaddr), E>>,
    Fut1: Future<Output = Result<O1, E>>,
    Fut2: Future<Output = Result<O2, E>>,
    F: FnOnce(O1, ConnectionOrigin) -> Fut2 + Clone,
    E: ::std::error::Error,
{
    fn new(stream: St, f: F) -> Self {
        Self { stream, f }
    }
}

impl<St, Fut1, O1, Fut2, O2, E, F> Stream for AndThenStream<St, F>
where
    St: Stream<Item = Result<(Fut1, Multiaddr), E>>,
    Fut1: Future<Output = Result<O1, E>>,
    Fut2: Future<Output = Result<O2, E>>,
    F: FnOnce(O1, ConnectionOrigin) -> Fut2 + Clone,
    E: ::std::error::Error,
{
    type Item = Result<(AndThenFuture<Fut1, Fut2, F>, Multiaddr), E>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context) -> Poll<Option<Self::Item>> {
        match self.as_mut().project().stream.poll_next(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e))),
            Poll::Ready(Some(Ok((fut1, addr)))) => Poll::Ready(Some(Ok((
                AndThenFuture::new(fut1, self.f.clone(), ConnectionOrigin::Inbound),
                addr,
            )))),
        }
    }
}

#[derive(Debug)]
enum AndThenChain<Fut1, Fut2, F> {
    First(Fut1, Option<(F, ConnectionOrigin)>),
    Second(Fut2),
    Empty,
}

/// Future generated by the [`AndThen`] transport.
///
/// Takes a future (Fut1) generated from an underlying transport, runs it to completion and applies
/// a closure (F) to the result to create another future (Fut2) which is then run to completion.
// Ideally we'd want to use `pin` to get a pinned version of the `AndThenChain`, unfortunately
// a Pin<&mut AndThenChain> doesn't let us construct Pin<&mut Fut> pins for the interior
// futures stored in the enum variants; as such we leave it unpinned and instead proceed  with
// great caution:
//
//   1. We take care to never move `chain` or its interior Futures
//   2. When transitioning from First to Second state we first ensure that the `drop` method is
//      called on the future stored in First prior to advancing to Second.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct AndThenFuture<Fut1, Fut2, F> {
    chain: AndThenChain<Fut1, Fut2, F>,
}

impl<Fut1, O1, Fut2, O2, E, F> AndThenFuture<Fut1, Fut2, F>
where
    Fut1: Future<Output = Result<O1, E>>,
    Fut2: Future<Output = Result<O2, E>>,
    F: FnOnce(O1, ConnectionOrigin) -> Fut2,
    E: ::std::error::Error,
{
    fn new(fut1: Fut1, f: F, origin: ConnectionOrigin) -> Self {
        Self {
            chain: AndThenChain::First(fut1, Some((f, origin))),
        }
    }
}

// Inspired by: https://github.com/rust-lang-nursery/futures-rs/blob/master/futures-util/src/future/chain.rs
impl<Fut1, O1, Fut2, O2, E, F> Future for AndThenFuture<Fut1, Fut2, F>
where
    Fut1: Future<Output = Result<O1, E>>,
    Fut2: Future<Output = Result<O2, E>>,
    F: FnOnce(O1, ConnectionOrigin) -> Fut2,
    E: ::std::error::Error,
{
    type Output = Result<O2, E>;

    fn poll(mut self: Pin<&mut Self>, mut context: &mut Context) -> Poll<Self::Output> {
        loop {
            let (output, (f, origin)) = match self.as_mut().project().chain {
                // Step 1: Drive Fut1 to completion
                AndThenChain::First(fut1, data) => {
                    // Safe to construct a Pin of the interior future because
                    // `self` is pinned (and therefor `chain` is pinned).
                    match unsafe { Pin::new_unchecked(fut1) }.poll(&mut context) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                        Poll::Ready(Ok(output)) => {
                            (output, data.take().expect("must be initialized"))
                        }
                    }
                }
                // Step 4: Drive Fut2 to completion
                AndThenChain::Second(fut2) => {
                    // Safe to construct a Pin of the interior future because
                    // `self` is pinned (and therefor `chain` is pinned).
                    return unsafe { Pin::new_unchecked(fut2) }.poll(&mut context);
                }
                AndThenChain::Empty => unreachable!(),
            };

            // Step 2: Ensure that Fut1 is dropped
            *self.as_mut().project().chain = AndThenChain::Empty;
            // Step 3: Run F on the output of Fut1 to create Fut2
            let fut2 = f(output, origin);
            *self.as_mut().project().chain = AndThenChain::Second(fut2)
        }
    }
}
