use darkbird::dashmap::mapref::one::Ref;
use serde::Serialize;

use crate::movie;


#[derive(Serialize)]
pub struct Movie {
    id: u64,
    title: String,
    time: i32,
    imdb_rate: i32,
    publish_year: i32,
    description: String,
    genre: String, 
    hashtags: Vec<String>,
    summary: String
}


impl Movie {
    pub fn new(rf: Ref<u64, movie::Movie>) -> Movie {
        let id = rf.key().to_owned();
        let movie = rf.value();
        
        Movie { 
            id, 
            title: movie.title.clone(),
            time: movie.time.clone(),
            imdb_rate: movie.imdb_rate.clone(),
            publish_year: movie.publish_year.clone(),
            description: movie.description.clone(),
            genre: movie.genre.clone(),
            hashtags: movie.hashtags.clone(),
            summary: movie.summary.clone(),
        }
    }

    pub fn new_list(list: Vec<Ref<u64, movie::Movie>>) -> Vec<Movie> {
        let mut vec = Vec::with_capacity(list.len());
        list.into_iter()
            .for_each(|rf| 
                vec.push(Movie::new(rf))
            );
        
        vec
    }
}
