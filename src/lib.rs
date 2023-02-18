use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

mod app;
pub use app::App;

#[derive(Display, EnumIter, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

impl Into<reqwest::Method> for Method {
    fn into(self) -> reqwest::Method {
        match self {
            Self::GET => reqwest::Method::GET,
            Self::POST => reqwest::Method::POST,
            Self::PUT => reqwest::Method::PUT,
            Self::DELETE => reqwest::Method::DELETE,
            Self::PATCH => reqwest::Method::PATCH,
        }
    }
}

pub struct Response {
    url: String,
    // headers: Iterator<> Fuck this its 5 am im not implemeneting that.
    body: Option<String>,
    status_code: u16,
    status_message: String,
}
