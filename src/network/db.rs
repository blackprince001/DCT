use sqlx_sqlite::SqlitePool;
use std::time::{Duration, SystemTime};

use crate::network::types::HourlySample;

#[async_trait::async_trait]
pub trait MetricsStorage {
    async fn store_hourly_sample(
        &self,
        sample: &HourlySample,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_hourly_samples(&self) -> Result<Vec<HourlySample>, Box<dyn std::error::Error>>;
}

#[derive(Debug, Clone)]

pub struct SqliteStorage {
    pub pool: SqlitePool,
}

impl SqliteStorage {
    pub async fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = SqlitePool::connect(db_path).await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS hourly_samples (
                timestamp INTEGER NOT NULL,
                avg_bytes_per_second_in REAL NOT NULL,
                avg_bytes_per_second_out REAL NOT NULL,
                total_bytes_received INTEGER NOT NULL,
                total_bytes_sent INTEGER NOT NULL
            );
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }
}

#[async_trait::async_trait]
impl MetricsStorage for SqliteStorage {
    async fn store_hourly_sample(
        &self,
        sample: &HourlySample,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = sample
            .timestamp
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        sqlx::query(
            r#"
            INSERT INTO hourly_samples 
            (timestamp, avg_bytes_per_second_in, avg_bytes_per_second_out, 
             total_bytes_received, total_bytes_sent)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(timestamp as i64)
        .bind(sample.avg_bytes_per_second_in)
        .bind(sample.avg_bytes_per_second_out)
        .bind(sample.total_bytes_received as i64)
        .bind(sample.total_bytes_sent as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_hourly_samples(&self) -> Result<Vec<HourlySample>, Box<dyn std::error::Error>> {
        let cutoff = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs()
            - (24_u64 * 3600);

        let ic = cutoff as i64;

        let rows = sqlx::query!(
            r#"
            SELECT timestamp as "timestamp!: i64",
                   avg_bytes_per_second_in,
                   avg_bytes_per_second_out,
                   total_bytes_received,
                   total_bytes_sent
            FROM hourly_samples 
            WHERE timestamp > ?
            ORDER BY timestamp DESC
            "#,
            ic
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| HourlySample {
                timestamp: std::time::UNIX_EPOCH + Duration::from_secs(row.timestamp as u64),
                avg_bytes_per_second_in: row.avg_bytes_per_second_in,
                avg_bytes_per_second_out: row.avg_bytes_per_second_out,
                total_bytes_received: row.total_bytes_received as u64,
                total_bytes_sent: row.total_bytes_sent as u64,
            })
            .collect())
    }
}
