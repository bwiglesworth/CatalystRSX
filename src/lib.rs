pub mod validation;
pub mod error;
pub mod middleware;
pub mod config;
pub mod logging;
pub mod routing {
    pub mod handlers;
}
pub mod models {
    pub mod user;
}

pub mod db;

pub mod auth {
    pub mod guard;
    pub mod password;
    pub mod admin;
    pub mod session;
}

pub mod templates {
    pub mod pages {
        pub mod index;
        pub mod admin {
            pub mod login;
        }
    }
}
mod tests;
pub mod security;

