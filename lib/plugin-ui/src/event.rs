use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{Node, Window};

/// What the player did in a plugin window. Every event carries the current value of each input of the window, so a
/// button press reads the amount typed next to it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UiEvent {
    pub window: String,
    /// The button, input or table; empty for `closed`.
    #[serde(default)]
    pub widget: String,
    pub kind: EventKind,
    /// The row of a `select`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row: Option<String>,
    #[serde(default)]
    pub inputs: BTreeMap<String, InputValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// A button was pressed.
    Click,
    /// Enter in an input.
    Submit,
    /// A row of a selectable table was clicked.
    Select,
    /// The player closed the window.
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputValue {
    Number(i32),
    Text(String),
}

impl UiEvent {
    pub fn click(window: &str, widget: &str) -> Self {
        Self { window: window.into(), widget: widget.into(), kind: EventKind::Click, row: None, inputs: BTreeMap::new() }
    }

    pub fn closed(window: &str) -> Self {
        Self { window: window.into(), widget: String::new(), kind: EventKind::Closed, row: None, inputs: BTreeMap::new() }
    }

    pub fn with_input(mut self, id: &str, value: InputValue) -> Self {
        self.inputs.insert(id.into(), value);
        self
    }

    /// The number typed in input `id`, if it is a number input of the window.
    pub fn number(&self, id: &str) -> Option<i32> {
        match self.inputs.get(id)? {
            InputValue::Number(value) => Some(*value),
            InputValue::Text(text) => text.trim().parse().ok(),
        }
    }

    pub fn text(&self, id: &str) -> Option<&str> {
        match self.inputs.get(id)? {
            InputValue::Text(text) => Some(text),
            InputValue::Number(_) => None,
        }
    }

    /// Checks the event against the window the player was shown, since a client can send anything: the widget must
    /// exist and take that kind of event, and inputs are kept only when they name an input of the window, clamped
    /// to its bounds.
    pub fn accepted_by(mut self, window: &Window) -> Result<Self, String> {
        if self.window != window.id {
            return Err(format!("event for window {} sent to {}", self.window, window.id));
        }
        match self.kind {
            EventKind::Closed => {
                if !window.closable {
                    return Err(format!("window {} cannot be closed", window.id));
                }
                self.widget.clear();
                self.row = None;
            }
            kind => {
                let widget = window.widget(&self.widget).ok_or_else(|| format!("window {} has no widget {}", window.id, self.widget))?;
                let fits = match (kind, widget) {
                    (EventKind::Click, Node::Button { enabled, .. }) => *enabled,
                    (EventKind::Submit, Node::NumberInput { .. } | Node::TextInput { .. }) => true,
                    (EventKind::Select, Node::Table(table)) => table.selectable && self.row.as_ref().is_some_and(|row| table.rows.iter().any(|r| &r.id == row)),
                    _ => false,
                };
                if !fits {
                    return Err(format!("widget {} of window {} does not take {kind:?}", self.widget, window.id));
                }
                if kind != EventKind::Select {
                    self.row = None;
                }
            }
        }
        let inputs = std::mem::take(&mut self.inputs);
        self.inputs = inputs
            .into_iter()
            .filter_map(|(id, value)| {
                let value = match (window.widget(&id)?, value) {
                    (Node::NumberInput { min, max, .. }, InputValue::Number(n)) => InputValue::Number(n.clamp(*min, *max)),
                    (Node::NumberInput { min, max, .. }, InputValue::Text(text)) => InputValue::Number(text.trim().parse::<i32>().ok()?.clamp(*min, *max)),
                    (Node::TextInput { max_len, .. }, InputValue::Text(text)) => InputValue::Text(text.chars().take(*max_len as usize).collect()),
                    (Node::TextInput { .. }, InputValue::Number(n)) => InputValue::Text(n.to_string()),
                    _ => return None,
                };
                Some((id, value))
            })
            .collect();
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Align, Table};

    fn window() -> Window {
        Window::new(
            "shop",
            "Shop",
            300,
            Node::column([
                Table::new("stock", 4).column("Item", 100, Align::Start).row("501", [Node::button("buy", "Buy")]).selectable().into(),
                Node::button("off", "Off").disabled(),
                Node::number_input("amount", 1, 1, 10),
                Node::text_input("note", "", 4),
            ]),
        )
    }

    #[test]
    fn clicks_must_name_an_enabled_button() {
        assert!(UiEvent::click("shop", "buy").accepted_by(&window()).is_ok());
        assert!(UiEvent::click("shop", "off").accepted_by(&window()).is_err());
        assert!(UiEvent::click("shop", "missing").accepted_by(&window()).is_err());
        assert!(UiEvent::click("other", "buy").accepted_by(&window()).is_err());
        assert!(UiEvent::click("shop", "amount").accepted_by(&window()).is_err());
    }

    #[test]
    fn selections_must_name_a_row() {
        let mut event = UiEvent::click("shop", "stock");
        event.kind = EventKind::Select;
        assert!(event.clone().accepted_by(&window()).is_err());
        event.row = Some("501".into());
        assert!(event.clone().accepted_by(&window()).is_ok());
        event.row = Some("502".into());
        assert!(event.accepted_by(&window()).is_err());
    }

    #[test]
    fn inputs_are_clamped_and_unknown_ones_dropped() {
        let event = UiEvent::click("shop", "buy")
            .with_input("amount", InputValue::Number(50))
            .with_input("note", InputValue::Text("abcdef".into()))
            .with_input("buy", InputValue::Number(3))
            .with_input("ghost", InputValue::Number(3))
            .accepted_by(&window())
            .unwrap();
        assert_eq!(event.number("amount"), Some(10));
        assert_eq!(event.text("note"), Some("abcd"));
        assert_eq!(event.inputs.len(), 2);
    }

    #[test]
    fn a_fixed_window_cannot_be_closed() {
        assert!(UiEvent::closed("shop").accepted_by(&window()).is_ok());
        assert!(UiEvent::closed("shop").accepted_by(&window().fixed()).is_err());
    }
}
