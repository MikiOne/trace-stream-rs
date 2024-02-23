use futures::future::{self, Either, Ready};
use log::{error, info};
use ntex::http::header::{HeaderName, HeaderValue};
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::ServiceCall;
use ntex::web::{Error, ErrorRenderer, HttpResponse, WebRequest, WebResponse};
use common::biz_code::BizCode;

use common::biz_resp::RespData;
use crate::auth::auth_consts::JWT_USER;

use crate::auth::jwt_handler;


pub struct JwtFilter;

impl<S> Middleware<S> for JwtFilter {
    type Service = JwtFilterMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        JwtFilterMiddleware { service }
    }
}

pub struct JwtFilterMiddleware<S> {
    service: S,
}

impl<S, Err> Service<WebRequest<Err>> for JwtFilterMiddleware<S>
where
    S: Service<WebRequest<Err>, Response = WebResponse, Error = Error> + 'static,
    Err: ErrorRenderer + 'static,
{
    type Response = WebResponse;
    type Error = Error;
    type Future<'f> = Either<ServiceCall<'f, S, WebRequest<Err>>,
        Ready<Result<Self::Response, Self::Error>>> where Self: 'f, Err: 'f;

    ntex::forward_poll_ready!(service);
    ntex::forward_poll_shutdown!(service);

    fn call<'a>(&'a self, mut req: WebRequest<Err>, ctx: ServiceCtx<'a, Self>) -> Self::Future<'a> {
        if req.path() == "/api/auth/token" {
            return Either::Left(ctx.call(&self.service, req));
        }

        let headers = req.headers();
        return match jwt_handler::get_jwt_user(headers) {
            Ok(user) => {
                let user_str = serde_json::to_string(&user).unwrap();
                info!("jwt user: {}", &user_str);
                req.headers_mut().insert(
                    HeaderName::from_static(JWT_USER),
                    HeaderValue::from_str(user_str.as_str()).unwrap(),
                );
                Either::Left(ctx.call(&self.service, req))
            }
            Err(err) => {
                error!("auth2 filter, error: {:?}", &err);
                let resp = RespData::with_biz_code(err.biz_code);
                Either::Right(future::ok(req.into_response(HttpResponse::Ok().json(&resp))))
            }
        };
    }
}
