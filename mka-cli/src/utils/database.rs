use rusqlite::{Connection, OptionalExtension};
use crate::models::configs::Config;
use anyhow::Context;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn open() -> anyhow::Result<Self> {
        // Register the extension BEFORE opening the connection
        unsafe {
            let _ = rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite_vec::sqlite3_vec_init as *const (),
            )));
        }

        let db_path = Config::get_db_path()?;
        let conn = Connection::open(db_path).context("Failed to open database")?;
        
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn init_schema(&self) -> anyhow::Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS workflow_meta (
                rowid INTEGER PRIMARY KEY,
                id TEXT UNIQUE,
                intent_hash TEXT
            )",
            [],
        ).context("Failed to create workflow_meta table")?;

        self.conn.execute(
            "CREATE VIRTUAL TABLE IF NOT EXISTS vec_workflows USING vec0(
                embedding float[384]
            )",
            [],
        ).context("Failed to create vec_workflows table")?;

        Ok(())
    }

    pub fn get_intent_hash(&self, id: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare("SELECT intent_hash FROM workflow_meta WHERE id = ?")?;
        let mut rows = stmt.query([id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn upsert_workflow(&self, id: &str, intent_hash: &str, embedding: &[f32]) -> anyhow::Result<()> {
        // Convert f32 slice to u8 slice for SQLite blob
        let embedding_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                embedding.as_ptr() as *const u8,
                embedding.len() * std::mem::size_of::<f32>(),
            )
        };

        // Find existing rowid if it exists
        let existing_rowid: Option<i64> = self.conn.query_row(
            "SELECT rowid FROM workflow_meta WHERE id = ?",
            [id],
            |row| row.get(0)
        ).optional()?;

        if let Some(rowid) = existing_rowid {
            self.conn.execute(
                "UPDATE workflow_meta SET intent_hash = ? WHERE rowid = ?",
                rusqlite::params![intent_hash, rowid],
            )?;
            
            self.conn.execute(
                "DELETE FROM vec_workflows WHERE rowid = ?",
                [rowid],
            )?;
            self.conn.execute(
                "INSERT INTO vec_workflows(rowid, embedding) VALUES (?, ?)",
                rusqlite::params![rowid, embedding_bytes],
            )?;
        } else {
            self.conn.execute(
                "INSERT INTO workflow_meta (id, intent_hash) VALUES (?, ?)",
                rusqlite::params![id, intent_hash],
            )?;
            let rowid = self.conn.last_insert_rowid();
            self.conn.execute(
                "INSERT INTO vec_workflows(rowid, embedding) VALUES (?, ?)",
                rusqlite::params![rowid, embedding_bytes],
            )?;
        }

        Ok(())
    }

    pub fn search(&self, embedding: &[f32], limit: usize) -> anyhow::Result<Vec<(String, f32)>> {
        let embedding_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(
                embedding.as_ptr() as *const u8,
                embedding.len() * std::mem::size_of::<f32>(),
            )
        };

        let mut stmt = self.conn.prepare(
            "SELECT m.id, v.distance
             FROM vec_workflows v
             JOIN workflow_meta m ON m.rowid = v.rowid
             WHERE v.embedding MATCH ? AND k = ?
             ORDER BY v.distance"
        )?;

        let rows = stmt.query_map(rusqlite::params![embedding_bytes, limit as i64], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f32>(1)?))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn cleanup_stale_workflows(&self, active_ids: &[String]) -> anyhow::Result<()> {
        if active_ids.is_empty() {
            self.conn.execute("DELETE FROM workflow_meta", [])?;
            self.conn.execute("DELETE FROM vec_workflows", [])?;
            return Ok(());
        }

        let placeholders: String = active_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let sql = format!("DELETE FROM workflow_meta WHERE id NOT IN ({})", placeholders);
        
        let mut stmt = self.conn.prepare(&sql)?;
        let params = rusqlite::params_from_iter(active_ids);
        stmt.execute(params)?;

        // Cleanup vec_workflows as well
        self.conn.execute(
            "DELETE FROM vec_workflows WHERE rowid NOT IN (SELECT rowid FROM workflow_meta)",
            [],
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod database_tests;
