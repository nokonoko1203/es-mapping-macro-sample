pub trait EsDocument: serde::Serialize {
    fn to_document(&self) -> serde_json::Value {
        serde_json::to_value(self).expect("Serialization failed")
    }
}
