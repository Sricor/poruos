use std::error::Error;

use diesel::backend::Backend;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::BigInt;

pub mod currency;

#[derive(AsExpression, FromSqlRow, PartialEq, Debug, Clone)]
#[sql_type = "BigInt"]
pub struct Amount(i64);

impl Amount {
    const PRECISION: u32 = 8;
    const PRECISION_NUM: i64 = 10_i64.pow(Self::PRECISION);

    pub fn from_f64(value: f64) -> Result<Self, Box<dyn Error>> {
        if value.is_nan() {
            return Err("NaN is not a valid value for Amount".into());
        }

        if value.is_infinite() {
            return Err("Infinity is not a valid value for Amount".into());
        }

        let scaled_value = value * (Self::PRECISION_NUM as f64);

        if scaled_value > i64::MAX as f64 || scaled_value < i64::MIN as f64 {
            return Err("Value is out of range for i64".into());
        }

        Ok(Amount(scaled_value.round() as i64))
    }

    pub fn to_f64(&self) -> f64 {
        self.0 as f64 / (Self::PRECISION_NUM as f64)
    }
}

mod sql_diesel {
    use super::*;

    impl<DB> ToSql<BigInt, DB> for Amount
    where
        DB: diesel::backend::Backend,
        i64: ToSql<BigInt, DB>,
    {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, DB>) -> diesel::serialize::Result {
            <i64 as ToSql<BigInt, DB>>::to_sql(&self.0, out)
        }
    }

    impl<DB> FromSql<BigInt, DB> for Amount
    where
        DB: diesel::backend::Backend,
        i64: FromSql<BigInt, DB>,
    {
        fn from_sql(bytes: <DB as Backend>::RawValue<'_>) -> diesel::deserialize::Result<Self> {
            let value = <i64 as FromSql<BigInt, DB>>::from_sql(bytes)?;

            Ok(Amount(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_f64_positive() {
        let amount = Amount::from_f64(123.45678901).unwrap();
        assert_eq!(amount.0, 12_345_678_901);
    }

    #[test]
    fn test_from_f64_negative() {
        let amount = Amount::from_f64(-123.45678901).unwrap();
        assert_eq!(amount.0, -12_345_678_901);
    }

    #[test]
    fn test_from_f64_zero() {
        let amount = Amount::from_f64(0.0).unwrap();
        assert_eq!(amount.0, 0);
    }

    #[test]
    fn test_from_f64_nan() {
        let result = Amount::from_f64(f64::NAN);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "NaN is not a valid value for Amount"
        );
    }

    #[test]
    fn test_from_f64_infinity() {
        let result = Amount::from_f64(f64::INFINITY);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Infinity is not a valid value for Amount"
        );
    }

    #[test]
    fn test_from_f64_negative_infinity() {
        let result = Amount::from_f64(f64::NEG_INFINITY);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Infinity is not a valid value for Amount"
        );
    }

    #[test]
    fn test_from_f64_out_of_range_positive() {
        let large_value = (i64::MAX as f64) / 1e8 + 1.0;
        let result = Amount::from_f64(large_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Value is out of range for i64"
        );
    }

    #[test]
    fn test_from_f64_out_of_range_negative() {
        let small_value = (i64::MIN as f64) / 1e8 - 1.0;
        let result = Amount::from_f64(small_value);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Value is out of range for i64"
        );
    }

    #[test]
    fn test_from_f64_precision_loss() {
        let amount = Amount::from_f64(0.00000001).unwrap();
        assert_eq!(amount.0, 1);
    }

    #[test]
    fn test_to_f64_positive() {
        let amount = Amount(12_345_678_901);
        assert_eq!(amount.to_f64(), 123.45678901);
    }

    #[test]
    fn test_to_f64_negative() {
        let amount = Amount(-12_345_678_901);
        assert_eq!(amount.to_f64(), -123.45678901);
    }

    #[test]
    fn test_to_f64_zero() {
        let amount = Amount(0);
        assert_eq!(amount.to_f64(), 0.0);
    }

    #[test]
    fn test_to_f64_max_value() {
        let amount = Amount(i64::MAX);
        assert_eq!(amount.to_f64(), i64::MAX as f64 / 1e8);
    }

    #[test]
    fn test_to_f64_min_value() {
        let amount = Amount(i64::MIN);
        assert_eq!(amount.to_f64(), i64::MIN as f64 / 1e8);
    }
}
