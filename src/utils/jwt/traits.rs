pub trait Encode<From, To> {
    type Error;
    fn encode(&self, data: From, key: &str) -> Result<To, anyhow::Error>;
}

pub trait Decode<From, To> {
    type Error;
    fn decode(&self, data: From, key: &str) -> Result<To, anyhow::Error>;
}
