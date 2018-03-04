use bson;
use mongodb::coll::Collection;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use regex::Regex;
use std::fmt;

use model::{File, Format, Document, DocumentBuilder};

pub struct Adapter(Collection);

impl<'a> Adapter {
    pub fn new(name: &'a str) -> Result<Adapter, String> {
        match Client::connect("localhost", 27017) {
            Ok(client) => {
                let coll = client.db("db").collection(name);
                Ok(Adapter(coll))
            },
            Err(_) => Err(format!("Unable to connect to database localhost:27017"))
        }
    }

    pub fn add(&self, document: Document) -> Result<(), String> {
        match self.0.insert_one(document.into(), None) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to insert into database"))
        }
    }

    pub fn del(&self, doc: bson::Document) -> Result<(), String> {
        match self.0.delete_many(doc, None) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to delete from database"))
        }
    }

    pub fn get_all(&self) -> Result<Vec<Document>, String> {
        self.get(None)
    }

    pub fn get(&self, doc: Option<bson::Document>) -> Result<Vec<Document>, String> {
        match self.0.find(doc, None).ok() {
            Some(cursor) => {
                let mut docs: Vec<Document> = vec![];
                for item in cursor {
                    match item {
                        Ok(doc) => docs.push(doc.into()),
                        Err(_) => return Err(format!("Error in cursor iteration"))
                    }
                }
                Ok(docs)
            },
            None => Err(format!("Unable to fetch from database"))
        }
    }
}
