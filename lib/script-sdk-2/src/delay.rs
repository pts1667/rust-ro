use crate::flow::{Script, Stop};
use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;

impl Ctx<'_> {
    /// Waits `milliseconds` before the script continues. The game keeps running meanwhile.
    pub fn sleep(&self, milliseconds: i32) -> Script {
        self.call(Function::Sleep, args![milliseconds]).map(|_| ())
    }

    /// Shows a progress bar of `color` for `seconds`, then runs the rest of the script. Moving cancels it.
    pub fn progress_bar(&self, color: i32, seconds: i32) -> Script {
        self.call(Function::ProgressBar, args![color, seconds]).map(|_| ())
    }
}

impl Ctx<'_> {
    /// A time counter, as rathena's `gettimetick`: `0` the server tick, `1` seconds since midnight, anything else
    /// the unix timestamp.
    pub fn time_tick(&self, kind: i32) -> Result<i32, Stop> {
        self.call(Function::GetTimeTick, args![kind])?.number()
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Value};

    use crate::transport::MockTransport;
    use crate::Ctx;

    #[test]
    fn sleep_sends_the_milliseconds() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).sleep(500).unwrap();
        assert_eq!(transport.calls(Function::Sleep)[0], vec![Value::new_number(500)]);
    }

    #[test]
    fn time_tick_sends_the_kind() {
        let transport = MockTransport::new(|_| Ok(Value::Number(42)));
        assert_eq!(Ctx::new(&transport).time_tick(1), Ok(42));
        assert_eq!(transport.calls(Function::GetTimeTick)[0], vec![Value::new_number(1)]);
    }
}
