impl Encode<PublicData, JwtToken> for JsonWebToken {
    type Error = anyhow::Error;
    fn encode(&self, encode_data: PublicData) -> Result<JwtToken, anyhow::Error> {
        let token = JsonWebTokenTool::encode_token(encode_data, self)
            .map_err(|err| {
                error!("encode_token: {}", err);
                err
            })?;
        Ok(token)
    }
}