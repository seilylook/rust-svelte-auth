use slqx::Row;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct NewUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateUser {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name="Adding a new User", 
skip(pool, new_user, redis_pool),
fields(
    new_user_email = %new_user_email,
    new_user_first_name = %new_user_first_name,
    new_user_last_name = %new_user_last_name
))]

#[actix_web::post("/register/")]
pub async fn register_user(
    pool: actix_web::web::Data<sqlx::postgres::Pgpool>,
    new_user: actix_web::web::Json<NewUser>,
    redis_pool: actix_web::web::Data<deadpool_redis::Pool>,
) -> actix_web::HttpResponse {
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(e) => {
            tracing::event!(target: "backend", transaction::Level::Error, "Unalbe to begin DB transaction: {:#?}", e);
            return actix_web::HttpResponse::InternalServerError().json(
                crate::types::ErrorResponse {
                    error: "Something unexpecteed happend. Please Type again"
                }
            )
        }
    }

}