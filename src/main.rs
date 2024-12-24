use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;

mod db;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    // Teste de conexão com o banco
    match db::establish_connection().await {
        Ok(pool) => {
            println!("Conexão com o banco de dados estabelecida com sucesso!");
            run_server(pool).await
        }
        Err(e) => {
            eprintln!("Erro ao conectar ao banco de dados: {:?}", e);
            std::process::exit(1);
        }
    }
}

async fn run_server(pool: PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // Agora clonando o Pool diretamente
            .service(
                web::scope("/users")
                    .route("", web::get().to(handlers::h_user::get_users))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
