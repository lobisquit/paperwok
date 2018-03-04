use std::fmt;
use std::slice::Iter;
use bson;
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Format {
    PDF,
    DOC,
    DOCX,
    JPEG,
    TXT,
    ODG,
    ODT,
}

impl Format {
    pub fn variants() -> Vec<Format> {
        vec![Format::PDF,
             Format::DOC,
             Format::DOCX,
             Format::JPEG,
             Format::TXT,
             Format::ODG,
             Format::ODT]
    }
}

impl Default for Format {
    fn default() -> Format {
        Format::TXT
    }
}

impl<'a> Format {
    pub fn new<T: Into<String>>(name: T) -> Result<Format, String> {
        let name_string: String = name.into();
        match name_string.as_ref() {
            "PDF"  | "pdf"  => Ok(Format::PDF),
            "DOC"  | "doc"  => Ok(Format::DOC),
            "DOCX" | "docx" => Ok(Format::DOCX),
            "JPEG" | "jpeg" => Ok(Format::JPEG),
            "TXT"  | "txt"  => Ok(Format::TXT),
            "ODG"  | "odg"  => Ok(Format::ODG),
            "ODT"  | "odt"  => Ok(Format::ODT),
            _ => Err(format!("Unsupported format: {}", name_string)),
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
pub struct File {
    path: String,
    format: Format,
}

impl<'a> File {
    pub fn new(path: &'a str, format: Format) -> Result<File, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\\W+").unwrap();
        }
        let fixed_name: String = RE.replace_all(path, "_").into();

        match fixed_name.as_ref() {
            "" => Err(format!("Invalid file name '{}'", path)),
            _  => Ok(File { path: fixed_name, format: format })
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
pub struct Document {
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
