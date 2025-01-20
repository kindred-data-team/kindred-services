use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, Error};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;
use std::rc::Rc;

pub struct Logger;

impl<S, B> Transform<S, ServiceRequest> for Logger
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggerMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct LoggerMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let start_time = Instant::now();
        let service = self.service.clone();

        Box::pin(async move {
            let response = service.call(req).await;
            match &response {
                Ok(res) => {
                    let status = res.status();
                    let duration = start_time.elapsed();
                    println!(
                        "{} {} -> {} (took {:?})",
                        method, path, status, duration
                    );
                }
                Err(err) => {
                    println!(
                        "{} {} -> INTERNAL SERVER ERROR: {}",
                        method, path, err
                    );
                }
            }
            response
        })
    }
}