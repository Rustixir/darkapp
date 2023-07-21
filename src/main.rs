

pub mod movie;
pub mod repo;
pub mod handler;
pub mod dto;

use darkbird::{Options, Storage, StorageType};
use movie::Movie;
use repo::Repository;



#[tokio::main]
async fn main() {

    let repo = Repository::new().await;

    
}
