use crate::db::types::Transfer;
use crate::db::{types::CreateTransfer, Database};
use crate::errors::AppError;
use std::sync::Arc;

#[derive(Clone)]
pub struct TransferManager {
    db: Arc<Database>,
}

impl TransferManager {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }

    pub async fn create_transfer(
        &self,
        hash: String,
        transfer: CreateTransfer,
    ) -> Result<Transfer, AppError> {
        let progress = transfer.progress.unwrap_or(0);

        sqlx::query(
            "INSERT INTO transfers (hash, transfer_id, progress, status, resolver, filename) 
             VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&hash)
        .bind(transfer.transfer_id)
        .bind(progress)
        .bind(&transfer.status)
        .bind(&transfer.resolver)
        .bind(&transfer.filename)
        .execute(&self.db.pool)
        .await?;

        Ok(Transfer {
            hash,
            transfer_id: transfer.transfer_id,
            progress,
            status: transfer.status,
            resolver: transfer.resolver,
            filename: transfer.filename,
        })
    }

    pub async fn list_transfers(&self) -> Result<Vec<Transfer>, AppError> {
        let rows = sqlx::query_as::<_, Transfer>(
            "SELECT hash, transfer_id, progress, status, resolver, filename 
             FROM transfers ORDER BY transfer_id DESC",
        )
        .fetch_all(&self.db.pool)
        .await?;

        Ok(rows)
    }

    pub async fn delete_transfer(&self, hash: &str) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM transfers WHERE hash = ?")
            .bind(hash)
            .execute(&self.db.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn upsert_transfer(
        &self,
        hash: String,
        transfer: CreateTransfer,
    ) -> Result<Transfer, AppError> {
        let progress = transfer.progress.unwrap_or(0);

        sqlx::query(
            "INSERT INTO transfers (hash, transfer_id, progress, status, resolver, filename) 
             VALUES (?, ?, ?, ?, ?, ?)
             ON CONFLICT(hash) DO UPDATE SET
             transfer_id = excluded.transfer_id,
             progress = excluded.progress,
             status = excluded.status,
             resolver = excluded.resolver,
             filename = excluded.filename",
        )
        .bind(&hash)
        .bind(transfer.transfer_id)
        .bind(progress)
        .bind(&transfer.status)
        .bind(&transfer.resolver)
        .bind(&transfer.filename)
        .execute(&self.db.pool)
        .await?;

        Ok(Transfer {
            hash,
            transfer_id: transfer.transfer_id,
            progress,
            status: transfer.status,
            resolver: transfer.resolver,
            filename: transfer.filename,
        })
    }
}
