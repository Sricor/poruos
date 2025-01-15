pub mod get {
    pub const PATH: &str = "/finance/currency/numeric_code";

    use serde::{Deserialize, Serialize};

    use crate::api::http::prelude::*;
    use crate::model::finance::currency::numeric_code::NumericCode;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub code: i64,
        pub symbol: String,
    }

    #[tracing::instrument()]
    pub async fn handler() -> ResponseResult<Vec<ResponseBody>> {
        let query = NumericCode::select_all()?;

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
    use crate::model::finance::currency::numeric_code::NumericCode;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ResponseBody {
        pub code: i64,
        pub symbol: String,
    }

    #[tracing::instrument()]
    pub async fn handler(Path(code): Path<i64>) -> ResponseResult<ResponseBody> {
        let query = NumericCode::select_one_by_code(code)?.ok_or(Response::bad_request(
            format!("numeric code {} not supported", code),
        ))?;

        Ok(Response::ok(ResponseBody {
            code: query.code(),
            symbol: query.symbol().to_string(),
        }))
    }
}
