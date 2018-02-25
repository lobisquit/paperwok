#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use bson::Bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn main() {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");

    let coll = client.db("documents").collection("documents");

    let doc = doc!{
        "title"  => "Posta: proprietà al 31/12/2017",
        "binder" => "Documenti",
        "folder" => "ISEE 2017",
        "year"   => 2018,
        "tags"   => ["ISEE", "Posta"]
    };

    coll.delete_many(doc!{"binder" => "Altri"}, None).unwrap();

    // Insert document into 'test.movies' collection
    coll.insert_one(doc.clone(), None)
        .ok()
        .expect("Failed to insert document.");

    // Find the document and receive a cursor
    let cursor = coll.find(Some(doc!{}), None)
        .ok().expect("Failed to execute find.");

    for item in cursor {
        // cursor.next() returns an Option<Result<Document>>
        match item {
            Ok(doc) => match doc.get("binder") {
                Some(&Bson::String(ref title)) => println!("{:?}", title),
                _ => println!("whoops"),
            },
            Err(_) => panic!("Failed to get next from server!")
        }
    }
}
