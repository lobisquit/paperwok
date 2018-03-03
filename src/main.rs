#![allow(unused_imports)]
#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate derive_builder;

#[macro_use]
extern crate lazy_static;

#[macro_use(doc, bson)]
extern crate bson;
extern crate mongodb;
extern crate regex;
extern crate rocket;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use regex::Regex;
use rocket::response::NamedFile;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

mod model;
mod dbutils;
use model::{File, Format, Document, DocumentBuilder};
use dbutils::Adapter;

#[get("/all_docs")]
fn all_docs() -> String {
    let adapter = match Adapter::new("documents") {
        Ok(adapt) => adapt,
        Err(msg) => panic!(msg)
    };
    let documents = match adapter.get_all() {
        Ok(docs) => docs,
        Err(_) => panic!("Unable to collect documents")
    };
    match serde_json::to_string(&documents) {
        Ok(string) => string,
        Err(_) => panic!("Unable to jsonify vector of documents")
    }
}

#[get("/formats")]
fn formats() -> String {
    match serde_json::to_string(&Format::variants()) {
        Ok(string) => string,
        Err(_) => panic!("Unable to jsonify vector of documents")
    }
}

#[get("/documents/<file..>")]
fn documents(file: PathBuf) -> Result<NamedFile, io::Error> {
    NamedFile::open(Path::new("documents/").join(file))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![all_docs,
                            formats,
                            documents])
        .launch();
}
