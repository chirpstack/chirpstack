use crate::config;
use tonic::{Request, Status};
use uuid::Uuid;

pub mod claims;
pub mod error;
pub mod validator;

#[derive(PartialEq, Eq, Debug)]
pub enum AuthID {
    None,
    User(Uuid),
    Key(Uuid),
}

pub fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    let conf = config::get();

    let auth_str = match req.metadata().get("authorization") {
        Some(v) => match v.to_str() {
            Ok(vv) => vv,
            Err(e) => {
                return Err(Status::unauthenticated(format!("{}", e)));
            }
        },
        _ => {
            // some API methods do not require the authorization metadata. When it is not available
            // we do not error. Each will perform its own authorization.
            req.extensions_mut().insert(AuthID::None);
            return Ok(req);
        }
    };

    let auth_str = match auth_str.strip_prefix("Bearer ") {
        Some(v) => v,
        None => {
            return Err(Status::unauthenticated(
                "authorization metadata must in format 'Bearer <TOKEN>",
            ));
        }
    };

    let token = match claims::AuthClaim::decode(auth_str, conf.api.secret.as_ref()) {
        Ok(v) => v,
        Err(e) => {
            return Err(Status::unauthenticated(format!("{}", e)));
        }
    };

    let id = match Uuid::parse_str(&token.sub) {
        Ok(v) => v,
        Err(e) => {
            return Err(Status::unauthenticated(format!("{}", e)));
        }
    };

    match token.typ.as_ref() {
        "user" => {
            req.extensions_mut().insert(AuthID::User(id));
        }
        "key" => {
            req.extensions_mut().insert(AuthID::Key(id));
        }
        _ => {
            return Err(Status::unauthenticated(format!(
                "invalid token typ: {}",
                token.typ
            )));
        }
    };

    Ok(req)
}
