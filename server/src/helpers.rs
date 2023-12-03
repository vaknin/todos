use std::{env, net::ToSocketAddrs};
use anyhow::{Result, Context};
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use crate::types::ServerState;

pub fn initiate_server() -> Result<ServerState> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let connection_pool = create_connection_pool()?;
    let server_address_str = env::var("SERVER_ADDRESS")?;
    let server_port_str = env::var("SERVER_PORT")?;
    
    let server_address = format!("{}:{}", server_address_str, server_port_str)
        .to_socket_addrs()?
        .next()
        .context("Invalid server address or port")?;

    Ok(ServerState {
        server_address,
        connection_pool
    })
}

pub fn create_connection_pool() -> Result<Pool<ConnectionManager<PgConnection>>> {

    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .context("Failed to create database connection pool.")
}