use anyhow::Result;
use chrono::NaiveDateTime;
use protobuf::SpecialFields;
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

    pub async fn get_stats(&self, minecraft_uuid: &str) -> Result<Vec<Stats>> {
        struct T {
            pub minecraft_uuid: String,
            pub server: String,

            pub playtime: Option<i32>,
            pub blocks_broken: Option<i32>,
            pub blocks_placed: Option<i32>,
            pub deaths: Option<i32>,

            pub last_updated: NaiveDateTime,
        }
        let re : sqlx::Result<Vec<T>> = sqlx::query_as!(
            T,
            r#"
            SELECT
                minecraft_uuid, server,
                playtime, blocks_broken, blocks_placed, deaths,
                last_updated
            FROM stats
            WHERE minecraft_uuid = $1
            ;"#,
            Uuid::parse_str(minecraft_uuid)?,
        )
            .fetch_all(&self.db)
            .await;

        let mut vec = Vec::new();
        if re.is_ok() {
            vec = re?.into_iter().map(|t| {
                Stats {
                    minecraft_uuid: t.minecraft_uuid,
                    server: t.server,
                    playtime: t.playtime,
                    blocks_broken: t.blocks_broken,
                    blocks_placed: t.blocks_placed,
                    deaths: t.deaths,
                    last_updated: Some(t.last_updated.and_utc().timestamp()),
                    special_fields: SpecialFields::default(),
                }
            } ).collect();
        }

        Ok(vec)
    }
}