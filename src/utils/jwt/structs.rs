use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Token {
    token: String,
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        token.token
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        self.token.as_ref()
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Token{token: s}
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct JwtToken {
    access_token: Token,
    refresh_token: Token
}