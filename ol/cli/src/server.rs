//! `server`  web monitor http server
use futures::StreamExt;
use serde_json::json;
use std::{convert::Infallible, fs, path::PathBuf, thread, time::Duration};
use tokio::time::interval;
use warp::{sse::ServerSentEvent, Filter};
use ol_types::config::IS_PROD;

use crate::{cache::Vitals, check::runner, node::node::Node};

fn sse_vitals(data: Vitals) -> Result<impl ServerSentEvent, Infallible> {
    Ok(warp::sse::json(data))
}

#[tokio::main]
/// starts the web server
pub async fn start_server(node: Node) {
    let cfg = &node.conf;

    let node_home = cfg.clone().workspace.node_home.clone();
    //GET check/ (json api for check data)
    let vitals_route = warp::path("vitals").and(warp::get()).map(move || {
        let path = node_home.clone();
        // create server event source from Check object
        let event_stream = interval(Duration::from_secs(10)).map(move |_| {
            let vitals = Vitals::read_json(&path);
            // let items = health.refresh_checks();
            sse_vitals(vitals)
        });
        // reply using server-sent events
        warp::sse::reply(event_stream)
    });

    let account_template = warp::path("account.json").and(warp::get().map(|| {
        fs::read_to_string("/root/.0L/account.json").unwrap()
        // let obj: Value = serde_json::from_str(&string);
    }));

    let node_home = cfg.clone().workspace.node_home.clone();
    let epoch_route = warp::path("epoch.json").and(warp::get().map(move || {
        // let node_home = node_home_two.clone();
        let vitals = Vitals::read_json(&node_home).chain_view.unwrap();
        let json = json!({
          "epoch": vitals.epoch,
          "waypoint": vitals.waypoint.unwrap().to_string()
        });
        json.to_string()
    }));


    let node_home = cfg.clone().workspace.node_home.clone();
    let web_files = if !*IS_PROD {
        // for using `npm run dev`
      PathBuf::from("/root/libra/ol/cli/web-monitor/public/")
    } else {
      node_home.join("web-monitor/public/")
    };

    //GET /
    let landing = warp::fs::dir(web_files);

    // TODO: Perhaps a better way to keep the check cache fresh?
    thread::spawn(move || {
        runner::run_checks(node, true, false);
    });

    warp::serve(landing.or(account_template).or(vitals_route).or(epoch_route))
        .run(([0, 0, 0, 0], 3030))
        .await;
}
