use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::Stop;

impl Ctx<'_> {
    /// A number from `0` to `max - 1`, as rathena's `rand(max)`. `max` must be at least 1.
    pub fn rand(&self, max: i32) -> Result<i32, Stop> {
        self.call(Function::Rand, args![max])?.number()
    }

    /// A number from `min` to `max`, both included, as rathena's `rand(min, max)`.
    pub fn rand_range(&self, min: i32, max: i32) -> Result<i32, Stop> {
        self.call(Function::Rand, args![min, max])?.number()
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn rand_sends_the_range_it_was_given() {
        let transport = MockTransport::new(|_| Ok(Value::Number(4)));
        let ctx = Ctx::new(&transport);
        assert_eq!(ctx.rand(10), Ok(4));
        assert_eq!(ctx.rand_range(3, 7), Ok(4));
        assert_eq!(transport.calls(Function::Rand), vec![vec![Value::new_number(10)], vec![Value::new_number(3), Value::new_number(7)]]);
    }
}
