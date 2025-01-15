mod claim;

use std::sync::Arc;

use axum::Router;

use crate::api::http::state::StateInner;

pub fn router(state: Arc<StateInner>) -> Router {
    use axum::routing::{get, post, put};

    Router::new()
        .route(get::PATH, get(get::handler))
        .route(put::PATH, put(put::handler))
        .route(post::PATH, post(post::handler))
        .route(claim::post::PATH, post(claim::post::handler))
        .with_state(state)
}

mod post {
    pub const PATH: &str = "/person";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::person::Person;

    use super::validate_value;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RequestBody {
        pub nickname: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub claim: String,
    }

    #[tracing::instrument()]
    pub async fn handler(Json(payload): Json<RequestBody>) -> ResponseResult<ResponseBody> {
        let nickname = validate_value::nickname(payload.nickname)?;
        let password = validate_value::password(payload.password)?;

        if let Some(_person) = Person::select_one_by_nickname(&nickname)? {
            return Err(Response::bad_request("nickname already exists".into()));
        }

        let person = Person::insert_one(&nickname, &password)?;

        Ok(Response::ok(ResponseBody {
            claim: Claim::new(person.unique()).issue()?,
        }))
    }
}

mod get {
    pub const PATH: &str = "/person/:unique";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::person::Person;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub unique: i64,
        pub nickname: String,
        pub created_at: u128,
        pub updated_at: u128,
    }

    #[tracing::instrument()]
    pub async fn handler(Path(unique): Path<i64>) -> ResponseResult<ResponseBody> {
        match Person::select_one_by_unique(unique)? {
            Some(person) => Ok(Response::ok(ResponseBody {
                unique: person.unique(),
                nickname: person.nickname().to_string(),
                created_at: person.created_at(),
                updated_at: person.updated_at(),
            })),
            None => Err(Response::bad_request("person does not exist".into())),
        }
    }
}

mod put {
    pub const PATH: &str = "/person/:unique";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::person::Person;

    use super::validate_value;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RequestBody {
        pub nickname: Option<String>,
        pub password: Option<String>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub nickname: Option<String>,
        pub password: Option<String>,
    }

    #[tracing::instrument()]
    pub async fn handler(
        claim: Claim,
        Path(unique): Path<i64>,
        Json(payload): Json<RequestBody>,
    ) -> ResponseResult<ResponseBody> {
        if claim.subject() != unique {
            return Err(Response::bad_request("not auth".into()));
        }

        let mut is_update = false;

        let mut result = ResponseBody {
            nickname: None,
            password: None,
        };

        let mut person = Person::select_one_by_unique(claim.subject())?
            .ok_or(Response::bad_request("person does not exist".into()))?;

        if let Some(nickname) = payload.nickname {
            let nickname = validate_value::nickname(nickname)?;

            if person.nickname() != &nickname {
                is_update = true;
                person.update_nickname(nickname.to_string());
                result.nickname = Some(nickname);
            }
        }

        if let Some(password) = payload.password {
            let password = validate_value::password(password)?;

            if person.password() != &password {
                is_update = true;
                person.update_password(password.to_string());
                result.password = Some(password);
            }
        }

        if is_update {
            Person::update_one_by_unique(person.unique(), &person)?;
        }

        Ok(Response::ok(result))
    }
}

mod validate_value {
    use crate::api::http::prelude::Response;
    use crate::common::hash::{digest_to_hex, sha256_digest};

    /// Validates and processes a nickname
    pub fn nickname(value: String) -> Result<String, Response<()>> {
        if value.is_empty() {
            return Err(Response::bad_request(
                "nickname cannot be empty".to_string(),
            ));
        }

        if value.len() > 20 {
            return Err(Response::bad_request("nickname is too long".to_string()));
        }

        Ok(value)
    }

    /// Validates and hashes a password
    pub fn password(value: String) -> Result<String, Response<()>> {
        if value.is_empty() {
            return Err(Response::bad_request(
                "password cannot be empty".to_string(),
            ));
        }

        if value.len() > 1024 {
            return Err(Response::bad_request(
                "password is too long (maximum 1024 characters)".to_string(),
            ));
        }

        let digest = sha256_digest(value.as_bytes());

        digest_to_hex(&digest).ok_or(Response::bad_request("hex conversion failed".to_string()))
    }
}
