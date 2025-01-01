use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::BigInt;

// pub mod wallet;
pub mod currency;

// pub struct Amount(i64);

// impl Amount {
//     const PRECISION: u32 = 12;
//     const PRECISION_NUM: i64 = 10_i64.wrapping_pow(Self::PRECISION);

//     fn from_f64(value: f64) -> Self

//     fn to_i64(self) -> i64
// }
