
use darkbird::document::{Document, FullText, Indexer, MaterializedView, Range, Tags, RangeField};
use serde::{Serialize, Deserialize};



#[derive(Clone, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub time: i32,
    pub imdb_rate: i32,
    pub publish_year: i32,
    pub description: String,
    pub genre: String, 
    pub hashtags: Vec<String>,
    pub summary: String
}

impl Document for Movie {}


impl Indexer for Movie {
    fn extract(&self) -> Vec<String> {
        vec![self.title.clone()]
    }
}

// indexing group of movies by tags and genre
impl Tags for Movie {
    fn get_tags(&self) -> Vec<String> {
        let mut cloned = self.hashtags.clone();
        cloned.push(self.genre.clone());
        return cloned
    }
}

// indexing for range operation over rate
impl Range for Movie {
    fn get_fields(&self) -> Vec<darkbird::document::RangeField> {
        vec![
            RangeField {
                name: "imdb_rate".to_string(),
                value: self.imdb_rate.to_string()
            }
        ]
    }
}


impl MaterializedView for Movie {
    fn filter(&self) -> Option<String> {
        if self.imdb_rate > 6 {
            return Some("great-movies".to_owned())
        }
        if self.imdb_rate > 4 {
            return Some("good-movies".to_owned())
        }
        return None;
    }
}


// used for indexing movie summary for FullText search
impl FullText for Movie {
    fn get_content(&self) -> Option<String> {
        Some(self.summary.clone())
    }
}
