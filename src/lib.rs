pub mod app;
pub mod domain;
pub mod infra;
pub mod model;
pub mod services;
pub mod ui;

pub type AppResult<T> = Result<T, error::AppError>;

pub mod error {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum AppError {
        #[error("io error: {0}")]
        Io(#[from] std::io::Error),
        #[error("json error: {0}")]
        Json(#[from] serde_json::Error),
        #[error("yaml error: {0}")]
        Yaml(#[from] serde_yaml::Error),
        #[error("zip error: {0}")]
        Zip(#[from] zip::result::ZipError),
        #[error("validation error: {0}")]
        Validation(String),
        #[error("blocked: {0}")]
        Blocked(String),
    }
}
