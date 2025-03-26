use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum FeaturePolicy {
    None,
    Self_,
    All,
    Origins(Vec<String>)
}
impl ToString for FeaturePolicy {
    fn to_string(&self) -> String {
        match self {
            FeaturePolicy::None => "'none'".to_string(),
            FeaturePolicy::Self_ => "'self'".to_string(),
            FeaturePolicy::All => "*".to_string(),
            FeaturePolicy::Origins(origins) => origins.join(" "),
        }
    }
}
pub struct FeaturePolicyBuilder {
    directives: HashMap<String, FeaturePolicy>,
}

impl FeaturePolicyBuilder {
    pub fn new() -> Self {
        Self {
            directives: HashMap::new(),
        }
    }

    pub fn add_feature(&mut self, feature: &str, policy: FeaturePolicy) -> &mut Self {
        self.directives.insert(feature.to_string(), policy);
        self
    }

    pub fn build(&self) -> String {
        self.directives
            .iter()
            .map(|(feature, policy)| format!("{} {}", feature, policy.to_string()))
            .collect::<Vec<String>>()
            .join("; ")
    }
}
