mod app;
mod handlers;
mod schema;

use actix;
use actix_web::server;
use failure::Error;
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

    // initialize system and server

    let sys = actix::System::new("beryl");

    server::new(
        move|| create_app(schema.clone(), debug)
    )
    .bind(&server_addr)
    .expect(&format!("cannot bind to {}", server_addr))
    .start();

    println!("beryl listening on : {}", server_addr);

    sys.run();
    Ok(())
}

/// CLI args
#[derive(Debug, StructOpt)]
#[structopt(name="tesseract")]
struct Opt {
    #[structopt(short="a", long="addr")]
    address: Option<String>,
}
