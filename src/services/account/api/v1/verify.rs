use crate::{
    api,
    services::account::api::v1::models
};
use std::time::Duration;
use futures::future::Future;
use actix_web::{
    FutureResponse, AsyncResponder, HttpResponse, Json, State,
};
use crate::services::account::domain::{
    pending_account,
};
use futures::future::IntoFuture;
use rand::Rng;



pub fn verify_post(verify_data: Json<models::VerifyBody>, state: State<api::State>) -> FutureResponse<HttpResponse> {
    let mut rng = rand::thread_rng();

    tokio_timer::sleep(Duration::from_millis(rng.gen_range(700, 900))).into_future()
    .from_err()
    .and_then(move |_|
        state.db
        .send(pending_account::Verify{
            code: verify_data.code.clone(),
        }).flatten()
    )
    .and_then(move |is_valid| {
        let res = models::VerifyResponse{
            is_valid,
        };
        let res = api::Response::data(res);
        Ok(HttpResponse::Created().json(&res))
    })
    .map_err(|err| api::Error::from(err))
    .from_err()
    .responder()
}
