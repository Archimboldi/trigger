use ntex::web::{self, middleware, App, HttpServer, Error};
use ntex_files as fs;
use sqlx::SqlitePool;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use flexi_logger::{detailed_format, Cleanup, Criterion, Logger, Naming, LogTarget, Age};
use anyhow::Result;
use dotenv::dotenv;
use ntex_middleware_redirect_https::RedirectHTTPS; 

mod api;
mod model;

async fn edit() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/cuipeng.html")?)
}
async fn pinky() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/images/doc2/pinky.html")?)
}
async fn asteroids() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/images/doc14/asteroids.html")?)
}

#[ntex::main]
async fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    Logger::with_env()
        .format(detailed_format)
        .log_target(LogTarget::File)
        .directory("./static/logs/")
        .rotate(Criterion::Age(Age::Day), Naming::Timestamps, Cleanup::Never)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));
    dotenv().ok();
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let db_pool = SqlitePool::new(&database_url).await?;

    let serv = HttpServer::new(move|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(RedirectHTTPS::default())
            .data(db_pool.clone())
            .route("/writing/", web::get().to(edit))
            .route("/pinky", web::get().to(pinky))
            .route("/asteroids", web::get().to(asteroids))
            .service(api::list_jeu)
            .service(api::view)
            .service(api::view_)
            .service(
                fs::Files::new("/", "./static/").index_file("jeu.html")
            )
    })
    .workers(4)
    .bind_openssl("0.0.0.0:443", builder)?
    .bind("0.0.0.0:80")?;
    serv.run().await?;
    Ok(())
}