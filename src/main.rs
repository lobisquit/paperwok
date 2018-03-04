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
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use regex::Regex;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

mod dbutils;
use dbutils::Adapter;
mod model;
use model::{File, Format, Document, DocumentBuilder};

#[get("/<file>")]
fn web(file: String) -> Option<NamedFile> {
    if file.ends_with(".html") {
        NamedFile::open(Path::new("website/").join(file)).ok()
    }
    else {
        println!("    ====> DEBUG: file not found \"{}\"", file);
        None
    }
}

#[get("/")]
fn root() -> Option<NamedFile> {
    web("index.html".into())
}

#[get("/api/formats", format = "application/json")]
fn formats() -> Json<Vec<Format>> {
    Json(Format::variants())
}

#[get("/api/download/<file..>")]
fn download_doc(file: PathBuf) -> io::Result<NamedFile> {
    NamedFile::open(Path::new("documents/").join(file))
}

#[post("/api/add_doc", format = "application/json", data = "<doc>")]
fn add_doc(doc: Json<Document>) -> Result<(), String> {
    let adapter = match Adapter::new("documents") {
        Ok(adapt) => adapt,
        Err(msg) => panic!(msg)
    };
    adapter.add(doc.into_inner())
}

#[post("/api/del_doc", format = "application/json", data = "<doc>")]
fn del_doc(doc: Json<Document>) -> Result<(), String> {
    let adapter = match Adapter::new("documents") {
        Ok(adapt) => adapt,
        Err(msg) => panic!(msg)
    };
    adapter.del(doc.into_inner().into())
}

#[get("/api/get_docs", format="application/json")]
fn get_docs() -> Json<Vec<Document>> {
    let adapter = match Adapter::new("documents") {
        Ok(adapt) => adapt,
        Err(msg) => panic!(msg)
    };
    match adapter.get_all() {
        Ok(docs) => Json(docs),
        Err(_) => panic!("Unable to collect documents")
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![root,
                            web,
                            formats,
                            add_doc,
                            del_doc,
                            download_doc,
                            get_docs])
        .launch();
}
