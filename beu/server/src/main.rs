mod api;
mod app_config;
mod app_error;
mod db;
#[macro_use]
mod macros;
mod utils;
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use app_config::AppConfig;
use db::db::{ConnectionOptions, DB};
use leptos::*;
use leptos_actix::generate_route_list;
use log;
use once_cell::sync::Lazy;
use surrealdb::opt::auth::Root;
use std::env;
use crate::utils::session;
use leptos_actix::LeptosRoutes;

static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::init().await);

fn leptos_app() -> impl IntoView {
    use app::*;
    view! { <App /> }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    log::info!("Starting server...");

    let namespace = &APP_CONFIG.database_namespace;
    let database = &APP_CONFIG.database_name;
    let username = &APP_CONFIG.database_username;
    let password = &APP_CONFIG.database_password;

    let conn_opts = ConnectionOptions {
        namespace,
        database,
        credentials: Root { username, password },
    };
    let db = DB::connect("127.0.0.1:8000", &conn_opts)
        .await
        .unwrap_or_else(|err| {
            log::error!("Error Connecting To SurrealDB: {}", err);
            std::process::exit(1);
        });
    log::info!("Connected to SurrealDB...");
    let db_ctx = Data::new(db);

    let leptos_routes = generate_route_list(leptos_app);

    let addr = &APP_CONFIG.leptos_options.site_addr;
    log::info!("Starting Server at http://{}", addr);

    HttpServer::new(move || {
        let logger = Logger::default();
        let leptos_options = &APP_CONFIG.leptos_options;
        let site_root = leptos_options.site_root.clone();

        App::new()
            .app_data(db_ctx.clone())
            .wrap(IdentityMiddleware::default())
            .wrap(session::make_session())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5500")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(logger)
            .configure(app_config::configure)
            .leptos_routes(leptos_options.to_owned(), leptos_routes.to_owned(), leptos_app)
            .service(actix_files::Files::new("/", site_root.to_owned()))
            .wrap(actix_web::middleware::Compress::default())
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}
