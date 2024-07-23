use surrealdb::sql::{Array, Object, Value};
use crate::app_error::AppError as Error;

pub struct W<T>(pub T);

impl TryFrom<W<Value>> for Object {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Object, Error> {
        match val.0 {
            Value::Object(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Object".to_string())),
        }
    }
}

impl TryFrom<W<Value>> for Array {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<Array, Error> {
        match val.0 {
            Value::Array(obj) => Ok(obj),
            _ => Err(Error::XValueNotOfType("Array".to_string())),
        }
    }
}

impl TryFrom<W<Value>> for i64 {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<i64, Error> {
        match val.0 {
            Value::Number(obj) => Ok(obj.as_int()),
            _ => Err(Error::XValueNotOfType("i64".to_string())),
        }
    }
}

impl TryFrom<W<Value>> for bool {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<bool, Error> {
        match val.0 {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::XValueNotOfType("bool".to_string())),
        }
    }
}

impl TryFrom<W<Value>> for String {
    type Error = Error;
    fn try_from(val: W<Value>) -> Result<String, Error> {
        match val.0 {
            Value::Strand(strand) => Ok(strand.as_string()),
            Value::Thing(thing) => Ok(thing.to_string()),
            _ => Err(Error::XValueNotOfType("String".to_string())),
        }
    }
}
