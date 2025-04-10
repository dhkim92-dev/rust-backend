use std::collections::HashMap;
// use sea_orm::sqlx::query;
use serde::Serialize;

use crate::common::CursorList;

pub struct CursorListBuilder<T: Serialize> {
    data: Vec<T>,
    query: HashMap<String, String>,
    target: Option<String>,
    size: usize
}

impl<T: Serialize + Clone> CursorListBuilder<T> {

    pub fn new(data: Vec<T>, size: usize) -> Self {
        Self {
            data: data,
            query: HashMap::new(),
            target: None,
            size: size
        }
    }

    pub fn set_target(mut self, target: String) -> Self{
        if self.target.is_some() {
            panic!("Target already set");
        }

        self.target = Some(target);
        self
    }

    pub fn register_query(mut self, key: String, value: String) -> Self{
        self.query.insert(key, value);
        self
    }

    pub fn build(self, uri: String) -> CursorList<T> {
        if self.target.is_none() {
            panic!("Target not set");
        }

        let size = self.size;

        if size >= self.data.len() {
            return CursorList::new(self.data, None);
        }
        
        let target_query = self.extract_target();
        let fixed_queries = self.build_queries();

        let mut queries = String::new();

        queries.push_str(&fixed_queries);
        if let Some(target_query) = target_query {
            queries.push_str(&target_query);
        }

        CursorList::new(
            self.data[0..size].to_vec(),
            Some(format!("{}?{}", uri, queries)),
        )
    }

    fn build_queries(&self) -> String {
        let mut queries = String::new();

        queries.push_str(&format!("{}={}", "size", self.size));

        for (key, value) in &self.query {
            queries.push_str(&format!("&{}={}", key, value));
        }

        queries
    }

    fn extract_target(&self) -> Option<String> {
        let target = self.target.as_ref().expect("Target not set");
        let element = self.data.last().cloned();

        if let Some(element) = element {
            let json = serde_json::to_value(element).expect("Failed to convert element to JSON");
            let value = json.get(target).unwrap_or_else(|| {
                panic!("Target not found in element");
            }).to_string();
            let value = value.trim_matches('"').to_string();

            return Some(format!("&{}={}", target, value));
        } else {
            None
        }
    }
}


