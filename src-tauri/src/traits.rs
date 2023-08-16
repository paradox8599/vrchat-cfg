use crate::err::AppError;

pub trait Json {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Result<Self, AppError>
    where
        Self: std::marker::Sized;
}
