use crate::types::Product;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_products(callback: FetchCallback<Vec<Product>>) -> FetchTask {
    let req = Request::get("/products/products.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_product(id: i32, callback: FetchCallback<Product>) -> FetchTask {
    let req = Request::get(format!("/products/{}.json", id))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
