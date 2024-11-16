use anyhow::Result;
use sqlx::types::Uuid;
use sqlx::PgPool;
use crate::proto::stats::Stats;

#[derive(Clone, Debug)]
pub struct Store {
    db: PgPool
}

impl Store {
    pub fn new(db: PgPool) -> Self {
        Store { db }
    }

    pub async fn update_stats(&self, stats: &Stats) -> Result<bool> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO stats (
                minecraft_uuid, server,
                playtime, blocks_broken, blocks_placed, deaths,
                last_updated
            ) VALUES ($1, $2, $3, $4, $5, $6, CURRENT_TIMESTAMP)
            ON CONFLICT (minecraft_uuid, server) DO UPDATE SET
                playtime = $3,
                blocks_broken = $4,
                blocks_placed = $5,
                deaths = $6,
                last_updated = CURRENT_TIMESTAMP
            ;"#,
            Uuid::parse_str(&stats.minecraft_uuid)?,
            stats.server,
            stats.playtime,
            stats.blocks_broken,
            stats.blocks_placed,
            stats.deaths,
        )
            .fetch_optional(&self.db)
            .await;

        Ok(true)
    }
}