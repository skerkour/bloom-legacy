
use crate::{
    api,
    services::account::api::models::Register
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, State,
};


pub fn post_register(register_data: Json<Register>, _state: State<api::State>) -> FutureResponse<HttpResponse> {
    tokio_timer::sleep(Duration::from_millis(400))
    .from_err()
    .and_then(move |_| {
        let res = api::Response::data(register_data.0);
        Ok(HttpResponse::Ok().json(&res))
    })
    .responder()
    // Ok(HttpResponse::Ok().json(&res))
    // Ok(api::Response::data(register_data.0))
}
