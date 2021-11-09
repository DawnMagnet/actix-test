#[macro_use]
extern crate log;
use actix_test::prelude::*;

#[actix_web::main]
async fn main() -> Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .data(establish_connection(
                "mysql://root:hxy011204@39.105.160.181:3306/ByteDanceCamp"
            ))
            .service(to_user::check_update)
            .service(to_user::download_count)
            .service(to_admin::get_version_by_condition)
            .service(to_admin::get_versions)
    })
    .bind(("127.0.0.1", 8080))?;
    info!("Starting Server");
    server.run().await?;
    Ok(())
}
