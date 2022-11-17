mod api;
pub mod db;
pub mod schema;

use std::fs;

use axum::{routing::get, Router, Server};
use clap::{Parser, Subcommand};
use miette::{miette, Diagnostic, IntoDiagnostic, Result};
use schema::RegistryConfig;
use thiserror::Error;
use tokio::signal;
use tracing::{info, metadata::LevelFilter};

use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("Failed to parse registry.toml config file")]
    #[diagnostic(code(registry_host::parse_config_fail))]
    ParseConfigFileFail(
        #[from]
        #[source]
        toml::de::Error,
    ),
    #[error("Config file does not exist")]
    #[diagnostic(code(registry_host::config_file_doesnt_exist))]
    #[help("Try creating a registry.toml file")]
    ConfigDoesntExist(
        #[from]
        #[source]
        std::io::Error,
    ),
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long, default_value = "info")]
    log: LevelFilter,
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Default)]
pub enum Command {
    #[default]
    Run,
    Init,
}

pub async fn run(cli: Cli) -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(cli.log)
        .try_init()
        .map_err(|e| miette!("Failed to initialise tracing subscriber: {}", e))?;

    let config: RegistryConfig = toml::from_str(
        fs::read_to_string("registry.toml")
            .map_err(ConfigDoesntExist)?
            .as_str(),
    )
    .map_err(ParseConfigFileFail)?;

    match cli.command.unwrap_or_default() {
        Command::Run => {
            Server::bind(&(config.server.bind, config.server.port).into())
                .serve(app().await.into_make_service())
                .with_graceful_shutdown(shutdown_signal())
                .await
                .into_diagnostic()?;
        }
        Command::Init => {
            let (db, sess) = db::start_for_kv(&config.db.connection).await?;
            let ns = &config.db.ns;
            let dbname = &config.db.db;
            let ast = format!(
                "
BEGIN TRANSACTION;

DEFINE NAMESPACE {ns};
USE NAMESPACE {ns};
DEFINE DATABASE {dbname};
USE DB {dbname};

DEFINE TABLE user SCHEMAFULL
    PERMISSIONS
        FOR select, update WHERE id = $auth.id,
        FOR create, delete NONE;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD password ON user TYPE string;
DEFINE INDEX idx_user ON user COLUMNS user UNIQUE;

DEFINE SCOPE allusers
  -- the JWT session will be valid for 14 days
  SESSION 14d
  -- The optional SIGNUP clause will be run when calling the signup method for this scope
  -- It is designed to create or add a new record to the database.
  -- If set, it needs to return a record or a record id
  -- The variables can be passed in to the signin method
  SIGNUP ( CREATE user SET settings.marketing = $marketing, user = $user, pass = crypto::argon2::generate($pass), tags = $tags )
  -- The optional SIGNIN clause will be run when calling the signin method for this scope
  -- It is designed to check if a record exists in the database.
  -- If set, it needs to return a record or a record id
  -- The variables can be passed in to the signin method
  SIGNIN ( SELECT * FROM user WHERE user = $user AND crypto::argon2::compare(pass, $pass) );
  -- this optional clause will be run when calling the signup method for this scope

DEFINE TABLE packages SCHEMAFULL
    PERMISSIONS
        FOR create, update
            WHERE author = $auth.id
        FOR delete
            WHERE author = $auth.id
            OR $auth.admin = true;
DEFINE FIELD name ON TABLE packages TYPE string;

COMMIT TRANSACTION;
            "
            );
            let response = db
                .execute(&ast, &sess, None, true)
                .await
                .map_err(db::Error::DBError)?;

            info!("Response from database: {:#?}", response);
        }
    }

    Ok(())
}

pub async fn app() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest(
            "/api",
            Router::new().route("/handshake", get(api::handshake)),
        )
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
