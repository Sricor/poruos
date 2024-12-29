pub mod get {
    pub const PATH: &str = "/finance/currency/numeric_code";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::finance::currency::NumericCode;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub code: i32,
        pub symbol: String,
    }

    #[tracing::instrument()]
    pub async fn handler() -> ResponseResult<Vec<ResponseBody>> {
        let query = NumericCode::select_all()
            .ok_or(Response::bad_request("numeric code not found".into()))?;

        let result = query
            .into_iter()
            .map(|e| ResponseBody {
                code: e.code(),
                symbol: e.symbol().to_string(),
            })
            .collect();

        Ok(Response::ok(result))
    }
}

pub mod get_code {
    pub const PATH: &str = "/finance/currency/:code";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::finance::currency::NumericCode;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub code: i32,
        pub symbol: String,
    }

    #[tracing::instrument()]
    pub async fn handler(Path(code): Path<i32>) -> ResponseResult<ResponseBody> {
        let query = NumericCode::select_one_by_code(code)
            .ok_or(Response::bad_request("numeric code not found".into()))?;

        Ok(Response::ok(ResponseBody {
            code: query.code(),
            symbol: query.symbol().to_string(),
        }))
    }
}
