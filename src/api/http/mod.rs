pub mod request;
pub mod response;

pub mod prelude {
    use super::*;

    pub use request::body::Json;
    pub use request::headers::{Claim, Path, Query};

    pub use response::{Response, ResponseResult};

    pub use state::State;
}

pub mod state {
    use std::sync::Arc;

    use crate::time::timestamp;

    pub type State = axum::extract::State<Arc<StateInner>>;

    pub struct StateInner {}

    impl StateInner {
        pub fn new() -> Self {
            Self {}
        }

        pub fn timestamp_millis(&self) -> u128 {
            timestamp().as_millis()
        }
    }
}
