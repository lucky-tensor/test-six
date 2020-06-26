// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

mod utils;

use crate::handlers::utils::{
    handle_rejection, reply_with_async_channel_writer, reply_with_lcs_bytes,
    send_size_prefixed_lcs_bytes, unwrap_or_500,
};
use libra_crypto::hash::HashValue;
use libra_types::transaction::Version;
use libradb::backup::backup_handler::BackupHandler;
use warp::{filters::BoxedFilter, reply::Reply, Filter};

pub(crate) fn get_routes(backup_handler: BackupHandler) -> BoxedFilter<(impl Reply,)> {
    // GET latest_state_root
    let bh = backup_handler.clone();
    let latest_state_root = warp::path::end()
        .map(move || reply_with_lcs_bytes(&bh.get_latest_state_root()?))
        .map(unwrap_or_500)
        .recover(handle_rejection);

    // GET state_range_proof/<version>/<end_key>
    let bh = backup_handler.clone();
    let state_range_proof = warp::path!(Version / HashValue)
        .map(move |version, end_key| {
            reply_with_lcs_bytes(&bh.get_account_state_range_proof(end_key, version)?)
        })
        .map(unwrap_or_500)
        .recover(handle_rejection);

    // GET state_snapshot/<version>
    let bh = backup_handler.clone();
    let state_snapshot = warp::path!(Version)
        .map(move |version| {
            reply_with_async_channel_writer(&bh, |bh, sender| {
                send_size_prefixed_lcs_bytes(bh.get_account_iter(version), sender)
            })
        })
        .map(unwrap_or_500)
        .recover(handle_rejection);

    // GET state_root_proof/<version>
    let bh = backup_handler;
    let state_root_proof = warp::path!(Version)
        .map(move |version| reply_with_lcs_bytes(&bh.get_state_root_proof(version)?))
        .map(unwrap_or_500)
        .recover(handle_rejection);

    // Route by endpoint name.
    let routes = warp::any()
        .and(warp::path("latest_state_root").and(latest_state_root))
        .or(warp::path("state_range_proof").and(state_range_proof))
        .or(warp::path("state_snapshot").and(state_snapshot))
        .or(warp::path("state_root_proof").and(state_root_proof));

    // Serve all routes for GET only.
    warp::get().and(routes).boxed()
}
