use es_mapping::traits::mapping::EsMap;
use macros::EsMapping;
use serde_json::json;

#[test]
fn test_mapping() {
    #[derive(EsMapping)]
    struct User {
        #[es(type = "keyword")]
        user_id: i32,
        #[es(type = "text")]
        name: String,
        #[es(type = "text")]
        age: i32,
        year: i32,
    }

    let mapping = User::generate_mapping();
    assert_eq!(
        json!({
            "mappings": {
                "properties": {
                    "user_id": {
                        "type": "keyword"
                    },
                    "name": {
                        "type": "text"
                    },
                    "age": {
                        "type": "text"
                    },
                    "year": {
                        "type": "long"
                    }
                }
            }
        }),
        mapping
    );
}
