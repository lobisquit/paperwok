#![allow(unused_imports)]
use std::fmt;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Format {
    PDF,
    DOC,
    DOCX,
    JPEG,
    TXT,
    ODG,
    ODT,
}

impl Default for Format {
    fn default() -> Format {
        Format::TXT
    }
}

impl<'a> Format {
    fn new(name: &'a str) -> Result<Format, String> {
        match name {
            "PDF"  | "pdf"  => Ok(Format::PDF),
            "DOC"  | "doc"  => Ok(Format::DOC),
            "DOCX" | "docx" => Ok(Format::DOCX),
            "JPEG" | "jpeg" => Ok(Format::JPEG),
            "TXT"  | "txt"  => Ok(Format::TXT),
            "ODG"  | "odg"  => Ok(Format::ODG),
            "ODT"  | "odt"  => Ok(Format::ODT),
            _ => Err(format!("Unsupported format: {}", name)),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        write!(f, "{}", name.to_lowercase())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct File {
    path: String,
    format: Format,
}

impl<'a> File {
    fn new(path: &'a str, format: Format) -> Result<File, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\\W+").unwrap();
        }
        let fixed_name: String = RE.replace_all(path, "_").into();

        match fixed_name.as_ref() {
            "" => Err(format!("Invalid file name '{}'", path)),
            _ => Ok(File {
                path: fixed_name,
                format: format,
            }),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.path, self.format)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Builder, Clone)]
#[builder(setter(into))]
struct Document {
    title: String,
    binder: String,
    folder: String,
    year: i32,
    file: File,
    tags: Vec<String>,
}

impl Into<bson::Document> for Document {
    fn into(self) -> bson::Document {
        // I decided to panic here, since conversion should always work
        let bson_self: bson::Bson = match bson::to_bson(&self) {
            Ok(bson) => bson,
            Err(_)   => panic!("Error in Document -> bson::Bson")
        };
        match bson_self {
            bson::Bson::Document(ordered_doc) => ordered_doc,
            _ => panic!("Invalid bson::Bson enum: not a bson::Document")
        }
    }
}

impl From<bson::Document> for Document {
    fn from(item: bson::Document) -> Self {
        let bson_item: bson::Bson = bson::Bson::Document(item);
        match bson::from_bson(bson_item) {
            Ok(doc) => doc,
            Err(_) => panic!("Error in generating document from bson")
        }
    }
}

struct Adapter(mongodb::coll::Collection);

impl<'a> Adapter {
    fn new(name: &'a str) -> Result<Adapter, String> {
        match Client::connect("localhost", 27017) {
            Ok(client) => {
                let coll = client.db("db").collection(name);
                Ok(Adapter(coll))
            },
            Err(_) => Err(format!("Unable to connect to database localhost:27017"))
        }
    }

    fn add(&self, document: Document) -> Result<(), String> {
        match self.0.insert_one(document.into(), None) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Unable to insert into database"))
        }
    }

    fn get_all(&self) -> Result<Vec<Document>, String> {
        self.get(None)
    }

    fn get(&self, doc: Option<bson::Document>) -> Result<Vec<Document>, String> {
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
