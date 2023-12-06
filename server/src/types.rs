use std::net::SocketAddr;
use actix_web::{ResponseError, HttpResponse};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection, AsChangeset};
use serde::Deserialize;

pub struct ServerState {
    pub server_address: SocketAddr,
    pub connection_pool: Pool<ConnectionManager<PgConnection>>
}

pub type ServerResult = Result<HttpResponse, ServerError>;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {

    #[error("Database error")]
    DieselError(#[from] diesel::result::Error),

    #[error("Connection pool error")]
    PoolConnectionError(#[from] r2d2::Error),

    // #[error("Unknown internal error")]
    // Unknown
}
impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServerError::PoolConnectionError(_) | ServerError::DieselError(_) => HttpResponse::InternalServerError().finish()
            // ServerError::DieselError(_) | ServerError::ConnectionError => HttpResponse::InternalServerError().finish(),
            // ServerError::NotFound(_) => HttpResponse::NotFound().finish(),
            // ServerError::InvalidData { .. } => HttpResponse::BadRequest().finish(),
            // ServerError::Unknown => HttpResponse::InternalServerError().finish(),
        }
    }
}