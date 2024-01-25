use std::fmt;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, Path, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;

use sqlx::{Pool, Postgres};

pub async fn app(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route(
            "/",
            get(using_connection_pool_extractor).post(using_connection_extractor),
        )
        .route("/create_table", get(create_table))
        .route("/products", post(insert_one_product))
        .route("/products/:id", get(get_product).patch(update_product))
        .with_state(pool)
}

// we can extract the connection pool with `State`
async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

async fn using_connection_extractor(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&mut *conn)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

async fn create_table(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    let _ = sqlx::query_file!("migrations/create_products_table.sql")
        .fetch_one(&pool)
        .await
        .map_err(|err: sqlx::Error| (StatusCode::IM_A_TEAPOT, err.to_string()));

    Ok(format!("{:?}", "success"))
}

#[derive(Serialize, Deserialize, Default, sqlx::FromRow)]
struct Product {
    product_id: Option<i32>,
    product_name: String,
    description: Option<String>,
    price: f64,
    stock_quantity: i32,
    category_id: Option<i32>,
}

impl fmt::Display for Product {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(
            f,
            "product_id => {}\nproduct_name => {}\ndescription => {}\nprice => {}\nstock_quantity => {}\ncategory_id => {}",
            self.product_id.unwrap_or_default(),
            self.product_name,
            self.description.as_deref().unwrap_or_default(),
            self.price,
            self.stock_quantity,
            self.category_id.unwrap_or_default(),
        )
    }
}

async fn insert_one_product(
    State(pool): State<PgPool>,
    Json(product): Json<Product>,
) -> Result<String, (StatusCode, String)> {
    let row =
        sqlx::query_as::<_, Product>("INSERT INTO products (product_name, description, price, stock_quantity, category_id) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(product.product_name)
            .bind(product.description)
            .bind(product.price)
            .bind(product.stock_quantity)
            .bind(product.category_id)
            .fetch_one(&pool)
            .await
            .map_err(internal_error)?;

    Ok(format!("{}", row))
}

async fn get_product(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let query = "select * from products where product_id = $1";
    let result = sqlx::query_as::<_, Product>(query)
        .bind(product_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()))?;

    Ok(Json(result))
}

#[derive(Deserialize)]
struct UpdateProduct {
    product_name: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    stock_quantity: Option<i32>,
    category_id: Option<i32>,
}

async fn update_product(
    State(pool): State<PgPool>,
    Path(product_id): Path<i32>,
    Json(update_product): Json<UpdateProduct>,
) -> Result<Json<Product>, (StatusCode, String)> {
    let query = "select * from products where product_id = $1";
    let mut original_product = sqlx::query_as::<_, Product>(query)
        .bind(product_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()))?;

    // 將有 input 的值把舊值替換掉
    if let Some(product_name) = &update_product.product_name {
        original_product.product_name = product_name.to_string();
    }

    if let Some(description) = &update_product.description {
        original_product.description = Some(description.to_string());
    }

    if let Some(price) = &update_product.price {
        original_product.price = price.to_owned();
    }

    if let Some(stock_quantity) = &update_product.stock_quantity {
        original_product.stock_quantity = stock_quantity.to_owned();
    }

    if let Some(category_id) = &update_product.category_id {
        original_product.category_id = Some(category_id.to_owned());
    }

    let update_query = r#"
        UPDATE
            products
        SET
            product_name = $1,
            description = $2,
            price = $3,
            stock_quantity = $4,
            category_id = $5
        WHERE
            product_id = $6
        RETURNING
            *;
    "#;

    let result = sqlx::query_as::<_, Product>(update_query)
        .bind(original_product.product_name)
        .bind(original_product.description)
        .bind(original_product.price)
        .bind(original_product.stock_quantity)
        .bind(original_product.category_id)
        .bind(product_id)
        .fetch_one(&pool)
        .await
        .map_err(|err| (StatusCode::UNPROCESSABLE_ENTITY, err.to_string()))?;

    Ok(Json(result))
}
