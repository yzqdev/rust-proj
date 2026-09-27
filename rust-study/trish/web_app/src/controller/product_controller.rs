use axum::{Json, Router, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    id: u64,
    name: String,
    price: f64,
    category: String,
    in_stock: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    name: String,
    price: f64,
    category: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    name: Option<String>,
    price: Option<f64>,
    category: Option<String>,
    in_stock: Option<bool>,
}

// In-memory product store
lazy_static::lazy_static! {
    static ref PRODUCTS: Mutex<Vec<Product>> = Mutex::new(vec![
        Product {
            id: 0,
            name: "Rust Book".to_string(),
            price: 29.99,
            category: "Books".to_string(),
            in_stock: true,
        },
        Product {
            id: 1,
            name: "Laptop".to_string(),
            price: 999.99,
            category: "Electronics".to_string(),
            in_stock: true,
        },
    ]);
}

pub fn product_route() -> Router {
    Router::new()
        .route("/api/products", get(list_products).post(create_product))
        .route(
            "/api/products/{id}",
            get(get_product).put(update_product).delete(delete_product),
        )
        .route("/api/products/search", get(search_products))
}

async fn list_products() -> Json<Vec<Product>> {
    let products = PRODUCTS.lock().unwrap();
    Json(products.clone())
}

async fn create_product(Json(payload): Json<CreateProduct>) -> (StatusCode, Json<Product>) {
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let product = Product {
        id,
        name: payload.name,
        price: payload.price,
        category: payload.category,
        in_stock: true,
    };
    PRODUCTS.lock().unwrap().push(product.clone());
    (StatusCode::CREATED, Json(product))
}

async fn get_product(
    axum::extract::Path(id): axum::extract::Path<u64>,
) -> Result<Json<Product>, StatusCode> {
    let products = PRODUCTS.lock().unwrap();
    products
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_product(
    axum::extract::Path(id): axum::extract::Path<u64>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    let mut products = PRODUCTS.lock().unwrap();
    let product = products
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = payload.name {
        product.name = name;
    }
    if let Some(price) = payload.price {
        product.price = price;
    }
    if let Some(category) = payload.category {
        product.category = category;
    }
    if let Some(in_stock) = payload.in_stock {
        product.in_stock = in_stock;
    }

    Ok(Json(product.clone()))
}

async fn delete_product(axum::extract::Path(id): axum::extract::Path<u64>) -> StatusCode {
    let mut products = PRODUCTS.lock().unwrap();
    let len = products.len();
    products.retain(|p| p.id != id);
    if products.len() < len {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
    category: Option<String>,
}

async fn search_products(
    axum::extract::Query(query): axum::extract::Query<SearchQuery>,
) -> Json<Vec<Product>> {
    let products = PRODUCTS.lock().unwrap();
    let results: Vec<Product> = products
        .iter()
        .filter(|p| {
            let name_match = p.name.to_lowercase().contains(&query.q.to_lowercase());
            let cat_match = match &query.category {
                Some(cat) => p.category.to_lowercase() == cat.to_lowercase(),
                None => true,
            };
            name_match && cat_match
        })
        .cloned()
        .collect();
    Json(results)
}
