#[derive(Deserialize)]
pub struct FacebookWebLoginToken {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "expiresIn")]
    pub expires_in: u64,
    #[serde(rename = "signedRequest")]
    pub signed_request: String,
}
