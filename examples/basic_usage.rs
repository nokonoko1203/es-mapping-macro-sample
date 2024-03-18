use es_mapping::traits::{document::EsDocument, mapping::EsMap};
use macros::EsMapping;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, EsMapping)]
struct User {
    #[serde(rename = "userId")]
    #[es(type = "keyword")]
    user_id: i32,
    #[es(type = "text")]
    name: String,
    #[es(type = "text")]
    age: i32,
    year: i32,
}

impl EsDocument for User {}

fn main() {
    let user = User {
        user_id: 1,
        name: "John Doe".to_string(),
        age: 25,
        year: 2021,
    };

    let mapping = User::generate_mapping();
    println!("Mapping: {}", json!(mapping));

    let document = user.to_document();
    println!("Document: {}", json!(document));
}
