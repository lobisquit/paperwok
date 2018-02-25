#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main() {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll: mongodb::coll::Collection = client.db("documents").collection("documents");

    let doc: bson::Document = doc!{
        "title"  => "Posta: proprietà al 31/12/2017",
        "binder" => "Documenti",
        "folder" => "ISEE 2017",
        "year"   => 2018,
        "tags"   => ["ISEE", "Posta"]
    };

    // remove all entries (empty matches all)
    coll.delete_many(doc!{}, None).unwrap();

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document.");

    // Find the document and receive a cursor
    let cursor = coll.find(None, None)
        .ok().expect("Failed to execute find.");

    for item in cursor {
        for (key, value) in item.unwrap() {
            println!("{:?} -> {:?}", key, value);
        }
    }
}
