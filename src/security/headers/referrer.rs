use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    Origin,
    OriginWhenCrossOrigin,
    SameOrigin,
    StrictOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let policy = match self {
            ReferrerPolicy::NoReferrer => "no-referrer",
            ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicy::Origin => "origin",
            ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicy::SameOrigin => "same-origin",
            ReferrerPolicy::StrictOrigin => "strict-origin",
            ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            ReferrerPolicy::UnsafeUrl => "unsafe-url",
        };
        write!(f, "{}", policy)
    }
}

pub struct ReferrerPolicyBuilder {
    policy: ReferrerPolicy,
}

impl ReferrerPolicyBuilder {
    pub fn new() -> Self {
        Self {
            policy: ReferrerPolicy::StrictOriginWhenCrossOrigin, // Safe default
        }
    }

    pub fn set_policy(mut self, policy: ReferrerPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn build(&self) -> String {
        self.policy.to_string()
    }
}
