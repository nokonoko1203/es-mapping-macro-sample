#[derive(Debug)]
pub enum EsType {
    Text,
    Keyword,
    Number,
    Boolean,
    UnsignedNumber,
    Double,
    Object,
    Nested,
    GeoPoint,
    GeoShape,
}

impl EsType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EsType::Text => "text",
            EsType::Keyword => "keyword",
            EsType::Number => "long",
            EsType::Boolean => "boolean",
            EsType::UnsignedNumber => "unsigned_long",
            EsType::Double => "double",
            EsType::Object => "object",
            EsType::Nested => "nested",
            EsType::GeoPoint => "geo_point",
            EsType::GeoShape => "geo_shape",
        }
    }
}

pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

pub struct GeoShape {
    pub points: Vec<GeoPoint>,
}
