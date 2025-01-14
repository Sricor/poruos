pub mod get {
    pub const PATH: &str = "/finance/currency/transaction";

    use serde::{Deserialize, Serialize};

    use crate::api::http::paginate;
    use crate::api::http::prelude::*;
    use crate::model::finance::currency::transaction::Transaction;

    #[derive(Debug, Deserialize)]
    pub struct Params {
        pub page: Option<u32>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub unique: i64,
        pub amount: String,
        pub numeric_code: i64,
        pub remarks: Option<String>,
        pub occurrence_at: u128,
        pub created_at: u128,
        pub updated_at: u128,
    }

    #[tracing::instrument()]
    pub async fn handler(
        claim: Claim,
        Query(params): Query<Params>,
    ) -> ResponseResult<Vec<ResponseBody>> {
        let (limit, offset) = paginate(params.page.unwrap_or(0), 64);

        let query = Transaction::select_by_owner(claim.subject(), limit.into(), offset.into())?;

        let result = query
            .into_iter()
            .map(|e| ResponseBody {
                unique: e.unique(),
                amount: e.amount.to_string(),
                numeric_code: e.numeric_code,
                remarks: e.remarks.clone(),
                occurrence_at: e.occurrence_at(),
                created_at: e.created_at(),
                updated_at: e.updated_at(),
            })
            .collect();

        Ok(Response::ok(result))
    }
}

pub mod post {
    pub const PATH: &str = "/finance/currency/transaction";

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::finance::currency::transaction::Transaction;
    use crate::model::finance::Amount;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct RequestBody {
        pub amount: Amount,
        pub numeric_code: i64,
        pub remarks: Option<String>,
        pub occurrence_at: Option<i64>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub unique: i64,
        pub amount: Amount,
        pub numeric_code: i64,
        pub remarks: Option<String>,
        pub occurrence_at: u128,
        pub created_at: u128,
        pub updated_at: u128,
    }

    #[tracing::instrument()]
    pub async fn handler(
        claim: Claim,
        Json(payload): Json<RequestBody>,
    ) -> ResponseResult<ResponseBody> {
        let occurrence_at = match payload.occurrence_at {
            Some(value) => {
                let time = DateTime::from_timestamp_millis(value).ok_or(Response::bad_request(
                    format!("invalid timestamp {}", value),
                ))?;
                time.naive_utc()
            }
            None => Utc::now().naive_utc(),
        };

        let item = Transaction::insert_one(
            claim.subject(),
            payload.amount,
            payload.numeric_code,
            payload.remarks.as_ref(),
            occurrence_at,
        )?;

        Ok(Response::ok(ResponseBody {
            unique: item.unique(),
            amount: item.amount.clone(),
            numeric_code: item.numeric_code,
            remarks: item.remarks.clone(),
            occurrence_at: item.occurrence_at(),
            created_at: item.created_at(),
            updated_at: item.updated_at(),
        }))
    }
}
