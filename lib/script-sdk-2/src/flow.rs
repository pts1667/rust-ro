/// How a script stops early. Every `ctx` call returns `Result<_, Stop>`, so `?` unwinds the whole script.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    /// The conversation is over, normally. Returned by [`Ctx::close`](crate::Ctx::close) and [`Ctx::end`](crate::Ctx::end).
    End,
    /// The script failed. The host logs the message.
    Error(String),
}

impl From<String> for Stop {
    fn from(error: String) -> Self {
        Self::Error(error)
    }
}

/// The result of a script. `Ok(())` and [`Stop::End`] both finish the script.
pub type Script = Result<(), Stop>;

/// Converts a script result into the status the host reads back, `Err` only for real failures.
pub fn finish(result: Script) -> Result<(), String> {
    match result {
        Ok(()) | Err(Stop::End) => Ok(()),
        Err(Stop::Error(error)) => Err(error),
    }
}
