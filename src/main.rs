#![allow(unused_imports)]
#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

#[macro_use(doc, bson)]
extern crate bson;
extern crate mongodb;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use regex::Regex;
use std::fmt;

mod model;
mod dbutils;
use model::{File, Format, Document, DocumentBuilder};
use dbutils::Adapter;

fn main() {
    let file = File::new("ciao)$%pollo", Format::new("pdf").unwrap()).unwrap();

    let my_doc = DocumentBuilder::default()
        .title("c")
        .binder("rosso")
        .folder("quella")
        .year(2010)
        .file(file)
        .tags(vec!["uno".into(), "due".into()])
        .build()
        .unwrap();

    let adapt = Adapter::new("prova").unwrap();
    adapt.add(my_doc.clone()).unwrap();

    for doc in &adapt.get(Some(doc!["title" => "c"])).unwrap() {
        println!("doc: {:?}", doc);
    }
}
