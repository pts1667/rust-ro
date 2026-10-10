use script_sdk::Function;

use crate::args;
use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::Val;

impl Ctx<'_> {
    /// Shows `text` in one dialogue box. Use `\n` for line breaks inside it.
    pub fn mes(&self, text: &str) -> Result<(), Stop> {
        self.call(Function::Mes, args![text]).map(|_| ())
    }

    /// Shows `text` under a `[speaker]` title line.
    pub fn mes_as(&self, speaker: &str, text: &str) -> Result<(), Stop> {
        self.mes(&format!("[{speaker}]\n{text}"))
    }

    /// Shows each of `lines` in one dialogue box, one line per entry, so line breaks need no escapes.
    pub fn lines(&self, lines: Vec<Val>) -> Result<(), Stop> {
        self.call(Function::Mes, lines).map(|_| ())
    }

    /// [`lines`](Self::lines) under a `[speaker]` title line.
    pub fn lines_as(&self, speaker: impl Into<Val>, lines: Vec<Val>) -> Result<(), Stop> {
        let speaker: Val = speaker.into();
        let mut all = vec![Val::from("[") + speaker + "]"];
        all.extend(lines);
        self.lines(all)
    }

    /// Waits for the player to press next.
    pub fn next(&self) -> Result<(), Stop> {
        self.call(Function::Next, args![]).map(|_| ())
    }

    /// Closes the dialogue window and ends the script.
    pub fn close(&self) -> Script {
        self.close_window()?;
        Err(Stop::End)
    }

    /// Closes the dialogue window and keeps the script running.
    pub fn close_window(&self) -> Result<(), Stop> {
        self.call(Function::Close, args![]).map(|_| ())
    }

    /// Ends the script without touching the window.
    pub fn end(&self) -> Script {
        Err(Stop::End)
    }

    /// Offers `options` and returns the index of the one the player picked. Cancelling the menu is an error.
    pub fn menu(&self, options: &[impl AsRef<str>]) -> Result<usize, Stop> {
        let selected = self
            .call(Function::Select, options.iter().map(|option| option.as_ref().into()).collect())?
            .number()?;
        usize::try_from(selected)
            .ok()
            .filter(|selected| (1..=options.len()).contains(selected))
            .map(|selected| selected - 1)
            .ok_or_else(|| Stop::Error("Conversation cancelled".into()))
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Function, Reply, Request, Value};

    use crate::flow::Stop;
    use crate::transport::MockTransport;
    use crate::{Ctx, Script, Val, args};

    fn choosing(choice: i32) -> MockTransport {
        MockTransport::new(move |request| match request {
            Request::Call {
                function: Function::Select,
                ..
            } => Ok(Value::Number(choice)),
            _ => Ok(Value::default()),
        })
    }

    #[test]
    fn mes_as_puts_the_speaker_on_its_own_line() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).mes_as("Prontera Guard", "Welcome.").unwrap();
        assert_eq!(
            transport.calls(Function::Mes),
            vec![vec![Value::String("[Prontera Guard]\nWelcome.".into())]]
        );
    }

    #[test]
    fn lines_as_sends_the_speaker_then_one_argument_per_line() {
        let transport = MockTransport::silent();
        Ctx::new(&transport).lines_as(Val::from("Chulsoo"), args!["Madeleine?", "Oh, hello!"]).unwrap();
        assert_eq!(
            transport.calls(Function::Mes),
            vec![vec![
                Value::String("[Chulsoo]".into()),
                Value::String("Madeleine?".into()),
                Value::String("Oh, hello!".into()),
            ]]
        );
    }

    #[test]
    fn menu_returns_a_zero_based_index() {
        let transport = choosing(2);
        assert_eq!(Ctx::new(&transport).menu(&["Yes", "No"]), Ok(1));
        assert_eq!(
            transport.calls(Function::Select),
            vec![vec![Value::String("Yes".into()), Value::String("No".into())]]
        );
    }

    #[test]
    fn cancelled_menu_is_an_error() {
        let transport = choosing(0);
        assert_eq!(
            Ctx::new(&transport).menu(&["Yes", "No"]),
            Err(Stop::Error("Conversation cancelled".into()))
        );
        let transport = choosing(3);
        assert!(Ctx::new(&transport).menu(&["Yes", "No"]).is_err());
    }

    #[test]
    fn close_closes_the_window_and_ends_the_script() {
        let transport = MockTransport::silent();
        let script: Script = Ctx::new(&transport).close();
        assert_eq!(script, Err(Stop::End));
        assert_eq!(transport.calls(Function::Close).len(), 1);
    }

    #[test]
    fn close_window_keeps_the_script_running() {
        let transport = MockTransport::silent();
        assert_eq!(Ctx::new(&transport).close_window(), Ok(()));
        assert_eq!(transport.calls(Function::Close).len(), 1);
    }

    #[test]
    fn host_failure_propagates_as_an_error_stop() {
        let transport = MockTransport::new(|_| -> Reply { Err("boom".into()) });
        assert_eq!(Ctx::new(&transport).next(), Err(Stop::Error("boom".into())));
    }
}
