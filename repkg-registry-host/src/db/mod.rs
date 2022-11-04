use miette::{Diagnostic, Result};
use surrealdb::{Datastore, Session};
use thiserror::Error;

use Error::*;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(registry_host::db::data_store_error))]
    DBError(#[from] surrealdb::Error),
}

pub async fn start_for_db(path: &str, ns: &str, db: &str) -> Result<(Datastore, Session)> {
    let datastore = Datastore::new(path).await.map_err(DBError)?;
    let session = Session::for_db(ns, db);

    Ok((datastore, session))
}

pub async fn start_for_sc(
    path: &str,
    ns: &str,
    db: &str,
    sc: &str,
) -> Result<(Datastore, Session)> {
    let datastore = Datastore::new(path).await.map_err(DBError)?;
    let session = Session::for_sc(ns, db, sc);

    Ok((datastore, session))
}

pub async fn start_for_kv(path: &str) -> Result<(Datastore, Session)> {
    let datastore = Datastore::new(path).await.map_err(DBError)?;
    let session = Session::for_kv();

    Ok((datastore, session))
}
