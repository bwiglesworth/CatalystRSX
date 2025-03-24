use rand::Rng;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct CsrfToken(String);

impl CsrfToken {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let random_bytes: [u8; 32] = rng.r#gen();
        let token = BASE64.encode(random_bytes);
        CsrfToken(token)
    }

    pub fn verify(&self, token: &str) -> bool {
        self.0 == token
    }

    pub fn get_token(&self) -> String {
        self.0.clone()
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
