use std::error::Error;

use actix_web::http::StatusCode;
use actix_web::ResponseError;

#[derive(thiserror::Error)]
pub enum PokedexError {
    #[error("Unable to process request: {0}")]
    InvalidRequest(#[source] anyhow::Error),
    #[error("Unexpected internal error: {0}")]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for PokedexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}\n", self))?;
        let mut error = self.source();
        while let Some(content) = error {
            f.write_fmt(format_args!("Caused by:\n\t{}", content))?;
            error = content.source();
        }
        Ok(())
    }
}

impl ResponseError for PokedexError {
    fn status_code(&self) -> StatusCode {
        match self {
            PokedexError::InvalidRequest(_) => StatusCode::NOT_FOUND,
            PokedexError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
