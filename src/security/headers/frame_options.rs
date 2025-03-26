#[derive(Debug, Clone)]
pub enum XFrameOptions {
    Deny,
    SameOrigin,
    AllowFrom(String),
}

impl ToString for XFrameOptions {
    fn to_string(&self) -> String {
        match self {
            XFrameOptions::Deny => "DENY".to_string(),
            XFrameOptions::SameOrigin => "SAMEORIGIN".to_string(),
            XFrameOptions::AllowFrom(uri) => format!("ALLOW-FROM {}", uri),
        }
    }
}

pub struct XFrameOptionsBuilder {
    option: XFrameOptions,
}

impl XFrameOptionsBuilder {
    pub fn new() -> Self {
        Self {
            option: XFrameOptions::SameOrigin,
        }
    }

    pub fn set_option(mut self, option: XFrameOptions) -> Self {
        self.option = option;
        self
    }

    pub fn build(&self) -> String {
        self.option.to_string()
    }
}
