pub(super) mod post {
    pub const PATH: &str = "/person/claim";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::person::Person;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RequestBody {
        pub nickname: String,
        pub password: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub claim: String,
        pub expire: u128,
    }

    #[tracing::instrument()]
    pub async fn handler(Json(payload): Json<RequestBody>) -> ResponseResult<ResponseBody> {
        let person = Person::select_one_by_nickname_password(&payload.nickname, &payload.password)
            .ok_or(Response::bad_request(
                "incorrect nickname or password".into(),
            ))?;

        let claim = Claim::new(person.unique());
        let expire = claim.expire();

        Ok(Response::ok(ResponseBody {
            claim: claim.issue()?,
            expire,
        }))
    }
}