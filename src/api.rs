use ntex::web::{self, HttpResponse};
use crate::model::{Info, HeadJeu, Edit, A};
use sqlx_core::sqlite::SqlitePool;
use thiserror::Error;
use ntex::web::WebResponseError;
use ntex::web::error::BlockingError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    Disconnect(#[from] sqlx_core::error::Error),
    #[error("unknown data store error")]
    Canceled,
    #[error("error request from {0}")]
    BadRequest(String),
}

impl WebResponseError for Error {}

impl From<BlockingError<Error>> for Error {
    fn from(err: BlockingError<Error>) -> Self {
        match err {
            BlockingError::Error(e) => e,
            BlockingError::Canceled => Error::Canceled,
        }
    }
}

#[web::get("/api/get")]
pub async fn list_jeu(db_pool: web::types::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let result = HeadJeu::query(db_pool.get_ref()).await.map_err(|_| Error::BadRequest(String::from("all jeus")));
    match result {
        Ok(jeu) => Ok(HttpResponse::Ok().json(&jeu)),
        _ => Ok(HttpResponse::BadRequest().body("Error trying to list jeu"))
    }
}

#[web::get("rush/b/{id}")]
pub async fn view(info: web::types::Path<Info>, db_pool: web::types::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let result = HeadJeu::details(info.id, db_pool.get_ref()).await.map_err(|_| Error::BadRequest(String::from("rush b")));
    match result {
        Ok(jeu) => Ok(HttpResponse::Ok().json(&jeu)),
        _ => Ok(HttpResponse::BadRequest().body("Error trying to details jeu"))
    }
}

#[web::get("flash/a/{id}")]
pub async fn view_(info: web::types::Path<Info>, db_pool: web::types::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let result = Edit::details_(info.id, db_pool.get_ref()).await.map_err(|_| Error::BadRequest(String::from("flash a")));
    match result {
        Ok(edit) => {
            let mut cc = edit.article.lines();
            let mut ar: Vec<String> = Vec::new();
            let mut ti = cc.next();
            let mut cle = String::new();
            while ti != None{
                cle.push_str(format!("{}{}{}", "<p>", ti.unwrap(), "</p>").as_str());
                ti = cc.next();
                if cle.len()>7000 {
                    ar.push(cle);
                    cle = String::new();
                }
            }
            if cle.len()>0 {
                ar.push(cle);
            }
            let a = A {
                chapter: edit.chapter,
                author: edit.author,
                time: edit.time,
                article: ar
            };
            Ok(HttpResponse::Ok().json(&a))
        },
        _ => Ok(HttpResponse::BadRequest().body("Error trying to details_ edit"))
    }
}