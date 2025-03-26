use time::{Duration, OffsetDateTime};

pub struct PasswordExpiration {
    max_age_days: u64,
}

impl PasswordExpiration {
    pub fn new(max_age_days: u64) -> Self {
        PasswordExpiration { max_age_days }
    }

    pub fn is_expired(&self, password_timestamp: OffsetDateTime) -> bool {
        let age = OffsetDateTime::now_utc() - password_timestamp;
        age >= Duration::days(self.max_age_days as i64 + 1)    }
}
