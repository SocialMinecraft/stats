mod proto;
mod util;
mod handlers;
mod store;

use anyhow::Result;
use tokio::task::JoinSet;
//use tracing::info;
use crate::handlers::update::update;
use crate::store::Store;

#[tokio::main]
async fn main() -> Result<()> {

    // get the app name, used for group and such
    let app_name = match util::get_app_name() {
        Some(name) => name,
        None => { return Err(anyhow::anyhow!("Could not  determine application name.")); },
    };

    // Setup logging
    util::setup_logging(app_name.as_str());

    // connect to db
    let db = util::connect_to_database().await?;
    let store = Store::new(db.clone());

    // connect to nats
    let nc = util::connect_to_nats().await?;

    let mut set = JoinSet::new();

    let _nc = nc.clone();
    let _store = store.clone();
    set.spawn(async move {
        util::handle_requests(_nc, "stats.update", move|_nc, msg| {
            update(_store.clone(), _nc, msg)
        }).await.expect("stats.update");
    });

    set.join_all().await;
    Ok(())
}
