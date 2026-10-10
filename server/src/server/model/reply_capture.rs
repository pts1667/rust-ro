//! The chat lines a command sends to its caller, collected so that a bot gets them as the answer of its call.
use std::cell::RefCell;

struct Capture {
    char_id: u32,
    lines: Vec<String>,
}

thread_local! {
    static CAPTURE: RefCell<Option<Capture>> = const { RefCell::new(None) };
}

struct Release;

impl Drop for Release {
    fn drop(&mut self) {
        CAPTURE.with(|capture| *capture.borrow_mut() = None);
    }
}

/// Runs `run` and returns the lines sent to `char_id` meanwhile on this thread. The lines are still delivered to the client.
pub fn capture_replies<R>(char_id: u32, run: impl FnOnce() -> R) -> (R, Vec<String>) {
    CAPTURE.with(|capture| *capture.borrow_mut() = Some(Capture { char_id, lines: Vec::new() }));
    let release = Release;
    let result = run();
    let lines = CAPTURE.with(|capture| capture.borrow_mut().take().map(|capture| capture.lines)).unwrap_or_default();
    drop(release);
    (result, lines)
}

pub fn record_reply(char_id: u32, text: &str) {
    CAPTURE.with(|capture| {
        if let Some(capture) = capture.borrow_mut().as_mut().filter(|capture| capture.char_id == char_id) {
            capture.lines.push(text.trim_end_matches('\0').to_string());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_lines_of_the_captured_character_are_kept_while_capturing() {
        record_reply(1, "before");
        let ((), lines) = capture_replies(1, || {
            record_reply(1, "first");
            record_reply(2, "someone else");
            record_reply(1, "second\0");
        });
        assert_eq!(lines, ["first", "second"]);
        record_reply(1, "after");
        assert!(capture_replies(1, || ()).1.is_empty());
    }
}
