pub struct ExpectCTBuilder {
    max_age: u32,
    enforce: bool,
    report_uri: Option<String>,
}

impl ExpectCTBuilder {
    pub fn new() -> Self {
        Self {
            max_age: 86400, // 24 hours default
            enforce: true,
            report_uri: None,
        }
    }

    pub fn max_age(mut self, seconds: u32) -> Self {
        self.max_age = seconds;
        self
    }

    pub fn enforce(mut self, enforce: bool) -> Self {
        self.enforce = enforce;
        self
    }

    pub fn report_uri(mut self, uri: String) -> Self {
        self.report_uri = Some(uri);
        self
    }

    pub fn build(&self) -> String {
        let mut directives = vec![format!("max-age={}", self.max_age)];
        
        if self.enforce {
            directives.push("enforce".to_string());
        }
        
        if let Some(uri) = &self.report_uri {
            directives.push(format!("report-uri=\"{}\"", uri));
        }
        
        directives.join(", ")
    }
}
