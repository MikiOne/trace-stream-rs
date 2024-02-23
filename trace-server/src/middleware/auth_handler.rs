use futures::future::ready;
use futures::future::Ready;
use log::error;
use ntex::http::Payload;
use ntex::web::{FromRequest, HttpRequest};

use common::biz_code::BizCode;
use common::biz_error::BizError;
use crate::auth::auth_consts::JWT_USER;

use crate::auth::models::SlimUser;

pub type LoggedUser = SlimUser;
impl<Err> FromRequest<Err> for LoggedUser {
    type Error = BizError;
    type Future = Ready<Result<LoggedUser, BizError>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.headers().get(JWT_USER) {
            None => {
                error!("Not logged in or Token expired");
                Err(BizError::new(BizCode::LOGIN_TIMEOUT))
            }
            Some(user_str) => {
                let user_str = user_str.to_str().unwrap();
                serde_json::from_str::<LoggedUser>(&user_str).map_err(|err| {
                    error!("Get LoggedUser from_request error: {:?}", err);
                    BizError::new(BizCode::LOGIN_TIMEOUT)
                })
            }
        })
    }

    // fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
    //     ready(match req.headers().get(JWT_USER) {
    //         None => {
    //             error!("未登录或Token过期");
    //             Err(AuErr(BizCode::LOGIN_TIMEOUT).into())
    //         }
    //         Some(user_str) => {
    //             let user_str = user_str.to_str().unwrap();
    //             serde_json::from_str::<LoggedUser>(&user_str)
    //                 .map_err(|err| AuErr(BizCode::LOGIN_TIMEOUT).into())
    //         }
    //     })
    // }
}
