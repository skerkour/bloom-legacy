use actix_web::{Result, HttpResponse, HttpRequest};
use actix_web::middleware::{Started, Finished, Middleware};
use chrono::{Utc, DateTime};
use crate::server::middlewares::request_id::GetRequestID;


pub struct Logger;

struct StartTime(DateTime<Utc>);

impl<S> Middleware<S> for Logger {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.extensions_mut().insert(StartTime(Utc::now()));
        return Ok(Started::Done);
    }

    fn finish(&self, req: &HttpRequest<S>, _resp: &HttpResponse) -> Finished {
        let start_time: StartTime = req.extensions_mut().remove().unwrap();
        let id = req.request_id().0.clone();
        println!("request {} started at: {:?} ended", id, start_time.0);
        return Finished::Done;
    }
}
