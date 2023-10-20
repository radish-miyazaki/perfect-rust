mod sample_handler;
mod tera_handler;

use actix_web::web::{resource, ServiceConfig};
use actix_web::{middleware, web, App, HttpServer};
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .configure(set_configure)
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}

fn set_configure(cfg: &mut ServiceConfig) -> () {
    cfg.service(
        web::scope("/sample")
            .route(
                "/calc1/{value1}/{value2}",
                web::get().to(sample_handler::calc_1),
            )
            .route("/calc2", web::get().to(sample_handler::calc_2))
            .route("/calc3", web::post().to(sample_handler::calc_3))
            .route("/calc4", web::post().to(sample_handler::calc_4))
            .route("/calc5", web::post().to(sample_handler::calc_5)),
    )
    .service(
        resource("/tera")
            .route(web::get().to(tera_handler::get_calc))
            .route(web::post().to(tera_handler::post_calc)),
    );
}
