use protobuf::{Message};
use async_nats::Client;
use crate::proto::stats_update::{UpdateStats, UpdateStatsResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn update(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let request = UpdateStats::parse_from_bytes(&msg.payload).unwrap();

    if let Some(reply) = msg.reply {

        // Update database
        db.update_stats(&request.stats).await?;

        // Build and Send Response
        let mut resp = UpdateStatsResponse::new();
        resp.success = true;
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;
    }

    return Ok(());
}