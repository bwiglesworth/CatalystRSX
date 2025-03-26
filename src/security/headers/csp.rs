use std::collections::HashMap;

pub struct CSPBuilder {
    directives: HashMap<String, Vec<String>>
}

impl CSPBuilder {
    pub fn new() -> Self {
        CSPBuilder {
            directives: HashMap::new()
        }
    }

    pub fn default_src(mut self, sources: Vec<&str>) -> Self {
        self.directives.insert("default-src".to_string(), sources.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn script_src(mut self, sources: Vec<&str>) -> Self {
        self.directives.insert("script-src".to_string(), sources.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn style_src(mut self, sources: Vec<&str>) -> Self {
        self.directives.insert("style-src".to_string(), sources.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn img_src(mut self, sources: Vec<&str>) -> Self {
        self.directives.insert("img-src".to_string(), sources.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn build(&self) -> String {
        self.directives
            .iter()
            .map(|(key, values)| format!("{} {}", key, values.join(" ")))
            .collect::<Vec<String>>()
            .join("; ")
    }
}