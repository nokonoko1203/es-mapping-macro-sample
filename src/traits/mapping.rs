pub trait EsMap {
    fn generate_mapping() -> serde_json::Value;
}
