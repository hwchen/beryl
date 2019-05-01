mod app;
mod backend;
mod clickhouse;
mod db_config;
mod dataframe;
mod error;
mod format;
mod handlers;
mod middleware;
mod schema;
mod query;
mod query_ir;

use actix;
use actix_web::server;
use failure::{Error, format_err, bail};
use pretty_env_logger;
use structopt::StructOpt;

use crate::app::create_app;
use crate::schema::Schema;

fn main() -> Result<(), Error> {
    pretty_env_logger::init();
    let opt = Opt::from_args();

    let server_addr = opt.address.unwrap_or("127.0.0.1:9999".into());

    let schema_path = std::env::var("BERYL_SCHEMA_FILEPATH")
        .unwrap_or("".into());
        //.expect("BERYL_SCHEMA_FILEPATH not found");
    let schema = Schema::from_path(&schema_path)?;

    let debug = false;

    // api key
    // bails if api key value is not unicode.
    // otherwise converts to an option.
    let api_key = match std::env::var("BERYL_API_KEY") {
        Ok(k) => Some(k),
        Err(err) => {
            match err {
                std::env::VarError::NotUnicode (_)=> bail!("For BERYL_API_KEY: {}", err),
                _ => None,
            }
        },
    };
    let with_api_key = api_key.is_some();

    // Database
    let db_url_full = std::env::var("BERYL_DATABASE_URL")
        .or(opt.database_url.ok_or(format_err!("")))
        .map_err(|_| format_err!("database url not found; either BERYL_DATABASE_URL or cli option required"))?;

    let (db, db_url, db_type) = db_config::get_db(&db_url_full)?;
    let db_type_viz = db_type.clone();

    // initialize system and server

    let sys = actix::System::new("beryl");

    server::new(
        move|| create_app(schema.clone(), db.clone(), api_key.clone(), debug)
    )
    .bind(&server_addr)
    .expect(&format!("cannot bind to {}", server_addr))
    .start();

    println!("beryl listening on : {}", server_addr);
    println!("beryl database:      {}, {}", db_url, db_type_viz);
    println!("beryl schema path:   {}", schema_path);

    if with_api_key {
        println!("beryl using api key auth");
    }

    sys.run();
    Ok(())
}

/// CLI args
#[derive(Debug, StructOpt)]
#[structopt(name="beryl")]
struct Opt {
    #[structopt(short="a", long="addr")]
    address: Option<String>,

    #[structopt(long="db-url")]
    database_url: Option<String>,
}
