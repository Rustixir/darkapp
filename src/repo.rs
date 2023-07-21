
use std::sync::atomic::{AtomicU64, Ordering};

use darkbird::{Database, Schema, Options, StorageType, SessionResult, dashmap::mapref::one::Ref};

use crate::movie::Movie;


pub struct Repository  {
    db: Database,
    seq: AtomicU64
}

impl Repository {
    
    pub async fn new() -> Self {
        Repository { 
            seq: AtomicU64::new(1),
            db: Schema::new()
                    .with_datastore::<u64, Movie>(default()).await.unwrap()
                    .build()
        }
    }

    pub async fn insert(&self, id: u64, movie: Movie) -> Result<(), SessionResult>{
        self.db.insert::<u64, Movie>(id, movie).await
    }

    pub fn lookup(&self, id: u64) -> Result<Option<Ref<u64, Movie>>, SessionResult> {
        self.db.lookup::<u64, Movie>(&id)
    }


    pub fn lookup_by_index(&self, key: &str) -> Result<Option<Ref<u64, Movie>>, SessionResult> {
        self.db.lookup_by_index::<u64, Movie>(&key)
    }

    pub fn lookup_by_tag(&self, tag: &str) -> Result<Vec<Ref<u64, Movie>>, SessionResult> {
        self.db.lookup_by_tag::<u64, Movie>(&tag)
    }


    pub async fn search(&self, text: String) -> Result<Vec<Ref<u64, Movie>>, SessionResult> {
        self.db.search::<u64, Movie>(text)
    }

    pub fn next_id(&self) -> u64 {
        self.seq.fetch_add(1, Ordering::Relaxed)
    }
    

}


fn default() -> Options<'static> {
    let path = "./"; 
    let storage_name = "movie"; 
    let total_page_size = 500; 
    let stype = StorageType::DiskCopies; 
    let off_reporter = true;
    return Options::new(path, storage_name, total_page_size, stype, off_reporter)
}