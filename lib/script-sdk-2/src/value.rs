use std::cmp::Ordering;
use std::ops::Add;

use script_sdk::Value;

use crate::flow::Stop;

/// Builds the argument list of [`Ctx::call`](crate::Ctx::call) from anything that converts into [`Val`].
#[macro_export]
macro_rules! args {
    ($($value:expr),* $(,)?) => {
        vec![$($crate::Val::from($value)),*]
    };
}

/// A script value: a number or a text. rathena scripts treat both interchangeably, so comparisons follow its rules.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Val(Value);

impl Val {
    pub fn number(&self) -> Result<i32, Stop> {
        self.0.number_value().map_err(Stop::from)
    }

    pub fn text(&self) -> String {
        self.0.text()
    }

    pub fn is_true(&self) -> bool {
        self.0.truthy()
    }

    pub fn into_value(self) -> Value {
        self.0
    }

    pub(crate) fn value(&self) -> &Value {
        &self.0
    }

    pub fn is_number(&self) -> bool {
        self.0.is_number()
    }

    /// The elements of an array value, or `None` for a number or a text.
    pub fn into_array(self) -> Option<Vec<Val>> {
        match self.0 {
            Value::Array(values) => Some(values.into_iter().map(Val).collect()),
            _ => None,
        }
    }

    /// Any binary operator of rathena that `script_sdk::Value` evaluates, as a value.
    pub(crate) fn binary(self, operation: &str, rhs: Val) -> Result<Val, Stop> {
        self.0.binary(operation, rhs.0).map(Val).map_err(Stop::from)
    }

    /// Equality that compares a number and a text by their text, as rathena does.
    pub fn loosely_equals(&self, other: &Val) -> bool {
        self == other || (self.0.is_number() != other.0.is_number() && self.text() == other.text())
    }

    /// Subtraction, failing on text operands. Named `try_*` because division by zero is an error too.
    pub fn try_sub(self, rhs: impl Into<Val>) -> Result<Val, Stop> {
        self.numeric("-", rhs)
    }

    pub fn try_mul(self, rhs: impl Into<Val>) -> Result<Val, Stop> {
        self.numeric("*", rhs)
    }

    pub fn try_div(self, rhs: impl Into<Val>) -> Result<Val, Stop> {
        self.numeric("/", rhs)
    }

    pub fn try_rem(self, rhs: impl Into<Val>) -> Result<Val, Stop> {
        self.numeric("%", rhs)
    }

    fn numeric(self, operation: &str, rhs: impl Into<Val>) -> Result<Val, Stop> {
        self.binary(operation, rhs.into())
    }
}

impl<T: Into<Val>> Add<T> for Val {
    type Output = Val;

    /// Adds numbers, and joins anything else as text, as rathena's `+` does.
    fn add(self, rhs: T) -> Val {
        Val(self.0.add(rhs.into().0))
    }
}

impl PartialEq<i32> for Val {
    fn eq(&self, other: &i32) -> bool {
        self.loosely_equals(&Val::from(*other))
    }
}

impl PartialEq<&str> for Val {
    fn eq(&self, other: &&str) -> bool {
        self.loosely_equals(&Val::from(*other))
    }
}

/// Only numbers are ordered. A text compares as unordered, so `<`, `>` and friends are false for it.
impl PartialOrd<i32> for Val {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        match self.0 {
            Value::Number(number) => Some(number.cmp(other)),
            _ => None,
        }
    }
}

impl From<i32> for Val {
    fn from(number: i32) -> Self {
        Self(Value::Number(number))
    }
}

impl From<bool> for Val {
    fn from(flag: bool) -> Self {
        Self(Value::Number(i32::from(flag)))
    }
}

impl From<&str> for Val {
    fn from(text: &str) -> Self {
        Self(Value::String(text.into()))
    }
}

impl From<String> for Val {
    fn from(text: String) -> Self {
        Self(Value::String(text))
    }
}

impl From<Value> for Val {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

impl From<Val> for Value {
    fn from(val: Val) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_and_text_compare_by_text() {
        assert!(Val::from("5") == 5);
        assert!(Val::from(5).loosely_equals(&Val::from("5")));
        assert!(Val::from("five") != 5);
    }

    #[test]
    fn numbers_order_and_texts_do_not() {
        let three = Val::from(3);
        let text = Val::from("3");
        assert!(three < 5);
        let seven = Val::from(7);
        assert!(seven >= 7);
        assert!(!(text.clone() < 5));
        assert!(!(text >= 5));
    }

    #[test]
    fn plus_adds_numbers_and_joins_texts() {
        assert_eq!(Val::from(1) + 2, Val::from(3));
        assert_eq!(Val::from("a") + "b", Val::from("ab"));
    }

    #[test]
    fn division_by_zero_is_an_error() {
        assert!(Val::from(1).try_div(0).is_err());
        assert_eq!(Val::from(9).try_div(3).unwrap(), Val::from(3));
    }
}
