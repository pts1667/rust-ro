use script_sdk::Function;

use crate::ctx::Ctx;
use crate::flow::Stop;

/// Where a typed input landed against its bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bound {
    Below,
    Within,
    Above,
}

impl Bound {
    /// The status number rathena scripts test after `input`: `-1`, `0` or `1`.
    pub fn code(self) -> i32 {
        match self {
            Bound::Below => -1,
            Bound::Within => 0,
            Bound::Above => 1,
        }
    }
}

/// What the player typed, kept inside its bounds, and which bound (if any) it crossed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input<T> {
    pub value: T,
    pub bound: Bound,
}

impl Ctx<'_> {
    /// Asks for a number. Values outside `min..=max` are clamped to the nearest bound.
    pub fn input_number(&self, min: i32, max: i32) -> Result<Input<i32>, Stop> {
        let entered = self.call(Function::InputNumber, vec![])?.number()?;
        Ok(if entered > max {
            Input { value: max, bound: Bound::Above }
        } else if entered < min {
            Input { value: min, bound: Bound::Below }
        } else {
            Input { value: entered, bound: Bound::Within }
        })
    }

    /// Asks for text. Text longer than `max` characters is cut to `max`; shorter than `min` is kept as typed.
    pub fn input_text(&self, min: usize, max: usize) -> Result<Input<String>, Stop> {
        let entered = self.call(Function::InputString, vec![])?.text();
        let length = entered.chars().count();
        Ok(if length > max {
            Input { value: entered.chars().take(max).collect(), bound: Bound::Above }
        } else if length < min {
            Input { value: entered, bound: Bound::Below }
        } else {
            Input { value: entered, bound: Bound::Within }
        })
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Request, Value};

    use crate::input::Bound;
    use crate::transport::MockTransport;
    use crate::Ctx;

    fn typing(entered: Value) -> MockTransport {
        MockTransport::new(move |request| match request {
            Request::Call { function: Function::InputNumber | Function::InputString, .. } => Ok(entered.clone()),
            _ => Ok(Value::default()),
        })
    }

    #[test]
    fn number_inside_bounds_is_kept() {
        let transport = typing(Value::Number(7));
        let input = Ctx::new(&transport).input_number(1, 10).unwrap();
        assert_eq!((input.value, input.bound), (7, Bound::Within));
    }

    #[test]
    fn number_outside_bounds_is_clamped() {
        let high = Ctx::new(&typing(Value::Number(99))).input_number(1, 10).unwrap();
        assert_eq!((high.value, high.bound), (10, Bound::Above));
        let low = Ctx::new(&typing(Value::Number(-4))).input_number(1, 10).unwrap();
        assert_eq!((low.value, low.bound), (1, Bound::Below));
    }

    #[test]
    fn long_text_is_cut_to_the_maximum() {
        let transport = typing(Value::new_string("abcdef".into()));
        let input = Ctx::new(&transport).input_text(0, 4).unwrap();
        assert_eq!((input.value.as_str(), input.bound), ("abcd", Bound::Above));
    }

    #[test]
    fn short_text_is_kept_as_typed() {
        let transport = typing(Value::new_string("ab".into()));
        let input = Ctx::new(&transport).input_text(3, 8).unwrap();
        assert_eq!((input.value.as_str(), input.bound), ("ab", Bound::Below));
    }
}
