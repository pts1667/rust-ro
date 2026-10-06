use std::path::Path;
use std::sync::RwLock;

pub const MOTD_LINE_LIMIT: usize = 128;

/// `conf/motd.txt`: one line per chat message shown to a player when they enter the game.
#[derive(Default)]
pub struct Motd {
    lines: RwLock<Vec<String>>,
}

pub fn parse_motd(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| !line.starts_with("//"))
        .map(|line| if line.is_empty() { " ".to_string() } else { line.to_string() })
        .take(MOTD_LINE_LIMIT)
        .collect()
}

impl Motd {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let motd = Self::default();
        motd.reload(path);
        motd
    }

    /// Re-reads the file; a missing file clears the message.
    pub fn reload(&self, path: impl AsRef<Path>) -> usize {
        let path = path.as_ref();
        let lines = match std::fs::read_to_string(path) {
            Ok(text) => parse_motd(&text),
            Err(error) => {
                warn!("Message of the day {} cannot be read: {error}", path.display());
                Vec::new()
            }
        };
        let count = lines.len();
        *self.lines.write().unwrap_or_else(|poisoned| poisoned.into_inner()) = lines;
        count
    }

    pub fn lines(&self) -> Vec<String> {
        self.lines.read().unwrap_or_else(|poisoned| poisoned.into_inner()).clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comments_are_skipped_and_empty_lines_become_a_blank_message() {
        assert_eq!(parse_motd("// comment\nWelcome!\n\nSecond line\r\n"), vec!["Welcome!", " ", "Second line"]);
        assert!(parse_motd("// only a comment").is_empty());
    }

    #[test]
    fn at_most_128_lines_are_kept() {
        let text = (0..200).map(|index| format!("line {index}\n")).collect::<String>();
        assert_eq!(parse_motd(&text).len(), MOTD_LINE_LIMIT);
    }

    #[test]
    fn reload_replaces_the_lines_and_a_missing_file_clears_them() {
        let dir = std::env::temp_dir().join(format!("motd-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("motd.txt");
        std::fs::write(&path, "one\ntwo\n").unwrap();
        let motd = Motd::load(&path);
        assert_eq!(motd.lines(), vec!["one", "two"]);
        std::fs::write(&path, "three\n").unwrap();
        assert_eq!(motd.reload(&path), 1);
        assert_eq!(motd.lines(), vec!["three"]);
        std::fs::remove_file(&path).unwrap();
        assert_eq!(motd.reload(&path), 0);
        assert!(motd.lines().is_empty());
        let _ = std::fs::remove_dir(&dir);
    }
}
