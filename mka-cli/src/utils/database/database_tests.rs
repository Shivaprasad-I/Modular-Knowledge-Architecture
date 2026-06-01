#[cfg(test)]
mod tests {
    use crate::utils::database::Database;
    use rusqlite::Connection;

    fn setup_test_db() -> Database {
        unsafe {
            rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite_vec::sqlite3_vec_init as *const (),
            )));
        }
        let conn = Connection::open_in_memory().unwrap();
        let db = Database { conn };
        db.init_schema().unwrap();
        db
    }

    #[test]
    fn test_init_schema() {
        let db = setup_test_db();
        let tables: Vec<String> = db.conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |r| r.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        
        assert!(tables.contains(&"workflow_meta".to_string()));
        assert!(tables.contains(&"vec_workflows".to_string()));
    }

    #[test]
    fn test_upsert_and_get_hash() {
        let db = setup_test_db();
        let id = "test-id";
        let hash = "hash-123";
        let embedding = vec![0.1f32; 384];

        db.upsert_workflow(id, hash, &embedding).unwrap();
        
        let stored_hash = db.get_intent_hash(id).unwrap();
        assert_eq!(stored_hash, Some(hash.to_string()));

        // Update
        let new_hash = "hash-456";
        db.upsert_workflow(id, new_hash, &embedding).unwrap();
        assert_eq!(db.get_intent_hash(id).unwrap(), Some(new_hash.to_string()));
    }

    #[test]
    fn test_search() {
        let db = setup_test_db();
        let embedding1 = vec![1.0f32; 384]; // Distance 0 from itself
        let embedding2 = vec![-1.0f32; 384]; // Very far

        db.upsert_workflow("id1", "h1", &embedding1).unwrap();
        db.upsert_workflow("id2", "h2", &embedding2).unwrap();

        let results = db.search(&embedding1, 5).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "id1");
        assert!(results[0].1 < 0.0001); // Near 0 distance
    }

    #[test]
    fn test_cleanup_stale() {
        let db = setup_test_db();
        let embedding = vec![0.1f32; 384];

        db.upsert_workflow("id1", "h1", &embedding).unwrap();
        db.upsert_workflow("id2", "h2", &embedding).unwrap();

        db.cleanup_stale_workflows(&["id1".to_string()]).unwrap();

        assert_eq!(db.get_intent_hash("id1").unwrap(), Some("h1".to_string()));
        assert_eq!(db.get_intent_hash("id2").unwrap(), None);
    }
}
