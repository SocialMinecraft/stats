use protobuf::{Message};
use async_nats::Client;
use crate::proto::stats_get::{GetStats, GetStatsResponse};
use crate::store::Store;

#[tracing::instrument]
pub async fn get(db: Store, nc: Client, msg: async_nats::Message) -> anyhow::Result<()> {
    let request = GetStats::parse_from_bytes(&msg.payload).unwrap();

    if let Some(reply) = msg.reply {
        // Get Results
        let mut result = Vec::new();
        for id in request.minecraft_ids {
            for stats in db.get_stats(id.as_str()).await? {
                result.push(stats);
            }
        }

        // Build and Send Response
        let mut resp = GetStatsResponse::new();
        resp.stats = result;
        let encoded: Vec<u8> = resp.write_to_bytes()?;
        nc.publish(reply, encoded.into()).await?;
    }

    return Ok(());
}