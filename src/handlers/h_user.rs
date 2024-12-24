use actix_web::{web, HttpResponse};
use sqlx::{PgPool, Row};


pub async fn get_users(pool: web::Data<PgPool>) -> HttpResponse {
    let query = "SELECT codigo, usuario, email FROM tb_cad_usuario";
    let result = sqlx::query(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(rows) => {
            let users: Vec<_> = rows
                .into_iter()
                .map(|row| {
                    format!(
                        "ID: {}, Name: {}, Email: {}",
                        row.get::<i32, _>("codigo"),
                        row.get::<String, _>("usuario"),
                        row.get::<String, _>("email"),
                    )
                })
                .collect();

            HttpResponse::Ok().json(users)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
