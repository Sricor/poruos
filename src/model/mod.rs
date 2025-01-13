pub mod finance;
pub mod person;

trait Model {
    fn initialize() -> &'static str;
}

pub(crate) fn initialize() -> String {
    [
        person::Person::initialize(),
        finance::currency::numeric_code::NumericCode::initialize(),
        finance::currency::transaction::Transaction::initialize(),
    ]
    .concat()
}
