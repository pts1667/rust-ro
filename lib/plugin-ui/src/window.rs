use serde::{Deserialize, Serialize};

/// A window a plugin opens for one player. Its `id` names it among the windows of that plugin for that player: opening
/// an id that is already open replaces the content and keeps where the player moved it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Window {
    pub id: String,
    pub title: String,
    pub width: u16,
    /// Grows with the content when absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u16>,
    #[serde(default = "yes")]
    pub closable: bool,
    pub node: Node,
}

fn yes() -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Column(Column),
    Row(Row),
    Spacer {
        size: u16,
    },
    Separator,
    /// `^RRGGBB` colour codes work inside `text`, as in dialogue.
    Text {
        text: String,
        /// `RRGGBB`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color: Option<String>,
        #[serde(default, skip_serializing_if = "is_false")]
        bold: bool,
        #[serde(default)]
        align: Align,
        /// Wraps at the width of its parent instead of growing it.
        #[serde(default, skip_serializing_if = "is_false")]
        wrap: bool,
    },
    Image {
        image: ImageRef,
        width: u16,
        height: u16,
    },
    /// The item's icon, with its name as tooltip.
    ItemIcon {
        item_id: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        amount: Option<u32>,
    },
    Button {
        id: String,
        label: String,
        #[serde(default = "yes")]
        enabled: bool,
    },
    NumberInput {
        id: String,
        value: i32,
        min: i32,
        max: i32,
        #[serde(default = "default_input_width")]
        width: u16,
    },
    TextInput {
        id: String,
        #[serde(default)]
        value: String,
        max_len: u16,
        #[serde(default = "default_input_width")]
        width: u16,
    },
    Table(Table),
    Gauge {
        value: u32,
        max: u32,
        width: u16,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        color: Option<String>,
    },
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn default_input_width() -> u16 {
    60
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Column {
    #[serde(default)]
    pub gap: u16,
    #[serde(default)]
    pub align: Align,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Row {
    #[serde(default)]
    pub gap: u16,
    /// Vertical alignment of the children.
    #[serde(default)]
    pub align: Align,
    pub children: Vec<Node>,
}

/// Rows of cells under titled columns. More rows than `visible_rows` scroll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub id: String,
    pub columns: Vec<TableColumn>,
    pub rows: Vec<TableRow>,
    pub visible_rows: u16,
    /// Clicking a row sends a `select` event with its id.
    #[serde(default)]
    pub selectable: bool,
    #[serde(default = "default_row_height")]
    pub row_height: u16,
}

fn default_row_height() -> u16 {
    24
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableColumn {
    pub title: String,
    pub width: u16,
    #[serde(default)]
    pub align: Align,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableRow {
    pub id: String,
    pub cells: Vec<Node>,
}

/// An image, animated when it has several frames. Frames are client texture paths; a path starting with `@/` is a
/// file of the plugin's own `ui/` folder, which the server turns into `plugin/<name>/...`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageRef {
    pub frames: Vec<String>,
    #[serde(default)]
    pub fps: f32,
    /// Stays on the last frame when false.
    #[serde(default = "yes")]
    pub looped: bool,
}

impl ImageRef {
    /// A file of the plugin's `ui/` folder.
    pub fn plugin(file: &str) -> Self {
        Self::texture(&format!("@/{file}"))
    }

    pub fn texture(path: &str) -> Self {
        Self { frames: vec![path.to_string()], fps: 0.0, looped: true }
    }

    pub fn animated<S: AsRef<str>>(frames: impl IntoIterator<Item = S>, fps: f32) -> Self {
        Self { frames: frames.into_iter().map(|frame| frame.as_ref().to_string()).collect(), fps, looped: true }
    }

    /// Frames of the plugin's `ui/` folder.
    pub fn plugin_animated<S: AsRef<str>>(files: impl IntoIterator<Item = S>, fps: f32) -> Self {
        Self::animated(files.into_iter().map(|file| format!("@/{}", file.as_ref())), fps)
    }

    pub fn once(mut self) -> Self {
        self.looped = false;
        self
    }

    /// The frame shown `seconds` after the image appeared.
    pub fn frame_at(&self, seconds: f32) -> Option<&str> {
        let count = self.frames.len();
        if count == 0 {
            return None;
        }
        let index = if count == 1 || self.fps <= 0.0 {
            0
        } else {
            let step = (seconds.max(0.0) * self.fps) as usize;
            if self.looped {
                step % count
            } else {
                step.min(count - 1)
            }
        };
        Some(&self.frames[index])
    }

    pub(crate) fn resolve(&mut self, plugin: &str) {
        for frame in &mut self.frames {
            if let Some(file) = frame.strip_prefix("@/") {
                *frame = crate::plugin_asset_path(plugin, file);
            }
        }
    }
}

impl Window {
    pub fn new(id: &str, title: &str, width: u16, node: Node) -> Self {
        Self { id: id.to_string(), title: title.to_string(), width, height: None, closable: true, node }
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    pub fn fixed(mut self) -> Self {
        self.closable = false;
        self
    }

    /// Every node of the window, depth first, cells of tables included.
    pub fn nodes(&self) -> Vec<&Node> {
        let mut nodes = Vec::new();
        self.node.collect(&mut nodes);
        nodes
    }

    pub fn widget(&self, id: &str) -> Option<&Node> {
        self.nodes().into_iter().find(|node| node.widget_id() == Some(id))
    }

    /// Turns `@/` image paths into the plugin's asset paths.
    pub fn resolve_assets(&mut self, plugin: &str) {
        self.node.for_each_image(&mut |image| image.resolve(plugin));
    }
}

impl Node {
    pub fn column(children: impl IntoIterator<Item = Node>) -> Self {
        Node::Column(Column { gap: 4, align: Align::Start, children: children.into_iter().collect() })
    }

    pub fn row(children: impl IntoIterator<Item = Node>) -> Self {
        Node::Row(Row { gap: 4, align: Align::Center, children: children.into_iter().collect() })
    }

    pub fn text(text: impl Into<String>) -> Self {
        Node::Text { text: text.into(), color: None, bold: false, align: Align::Start, wrap: false }
    }

    pub fn wrapped(text: impl Into<String>) -> Self {
        Node::Text { text: text.into(), color: None, bold: false, align: Align::Start, wrap: true }
    }

    pub fn title(text: impl Into<String>) -> Self {
        Node::Text { text: text.into(), color: None, bold: true, align: Align::Start, wrap: false }
    }

    pub fn image(image: ImageRef, width: u16, height: u16) -> Self {
        Node::Image { image, width, height }
    }

    pub fn item(item_id: u32) -> Self {
        Node::ItemIcon { item_id, amount: None }
    }

    pub fn button(id: impl Into<String>, label: impl Into<String>) -> Self {
        Node::Button { id: id.into(), label: label.into(), enabled: true }
    }

    pub fn number_input(id: impl Into<String>, value: i32, min: i32, max: i32) -> Self {
        Node::NumberInput { id: id.into(), value, min, max, width: default_input_width() }
    }

    pub fn text_input(id: impl Into<String>, value: impl Into<String>, max_len: u16) -> Self {
        Node::TextInput { id: id.into(), value: value.into(), max_len, width: default_input_width() }
    }

    pub fn gauge(value: u32, max: u32, width: u16) -> Self {
        Node::Gauge { value, max, width, color: None }
    }

    pub fn spacer(size: u16) -> Self {
        Node::Spacer { size }
    }

    /// Sets the colour of a `Text` (`RRGGBB`) or a `Gauge`; other nodes are unchanged.
    pub fn color(mut self, rgb: &str) -> Self {
        if let Node::Text { color, .. } | Node::Gauge { color, .. } = &mut self {
            *color = Some(rgb.to_string());
        }
        self
    }

    /// Aligns a `Text`, or the children of a `Column`/`Row`.
    pub fn align(mut self, to: Align) -> Self {
        match &mut self {
            Node::Text { align, .. } => *align = to,
            Node::Column(column) => column.align = to,
            Node::Row(row) => row.align = to,
            _ => {}
        }
        self
    }

    pub fn gap(mut self, size: u16) -> Self {
        match &mut self {
            Node::Column(column) => column.gap = size,
            Node::Row(row) => row.gap = size,
            _ => {}
        }
        self
    }

    pub fn disabled(mut self) -> Self {
        if let Node::Button { enabled, .. } = &mut self {
            *enabled = false;
        }
        self
    }

    pub fn widget_id(&self) -> Option<&str> {
        match self {
            Node::Button { id, .. } | Node::NumberInput { id, .. } | Node::TextInput { id, .. } | Node::Table(Table { id, .. }) => Some(id),
            _ => None,
        }
    }

    pub fn children(&self) -> Vec<&Node> {
        match self {
            Node::Column(Column { children, .. }) | Node::Row(Row { children, .. }) => children.iter().collect(),
            Node::Table(table) => table.rows.iter().flat_map(|row| row.cells.iter()).collect(),
            _ => Vec::new(),
        }
    }

    fn collect<'a>(&'a self, into: &mut Vec<&'a Node>) {
        into.push(self);
        for child in self.children() {
            child.collect(into);
        }
    }

    fn for_each_image(&mut self, apply: &mut impl FnMut(&mut ImageRef)) {
        match self {
            Node::Image { image, .. } => apply(image),
            Node::Column(Column { children, .. }) | Node::Row(Row { children, .. }) => children.iter_mut().for_each(|child| child.for_each_image(apply)),
            Node::Table(table) => table.rows.iter_mut().flat_map(|row| row.cells.iter_mut()).for_each(|cell| cell.for_each_image(apply)),
            _ => {}
        }
    }
}

impl Table {
    pub fn new(id: impl Into<String>, visible_rows: u16) -> Self {
        Self { id: id.into(), columns: Vec::new(), rows: Vec::new(), visible_rows, selectable: false, row_height: default_row_height() }
    }

    pub fn column(mut self, title: impl Into<String>, width: u16, align: Align) -> Self {
        self.columns.push(TableColumn { title: title.into(), width, align });
        self
    }

    pub fn row(mut self, id: impl Into<String>, cells: impl IntoIterator<Item = Node>) -> Self {
        self.rows.push(TableRow { id: id.into(), cells: cells.into_iter().collect() });
        self
    }

    pub fn selectable(mut self) -> Self {
        self.selectable = true;
        self
    }

    pub fn row_height(mut self, height: u16) -> Self {
        self.row_height = height;
        self
    }
}

impl From<Table> for Node {
    fn from(table: Table) -> Self {
        Node::Table(table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frames_loop_or_hold_on_the_last_one() {
        let image = ImageRef::animated(["a", "b", "c"], 2.0);
        assert_eq!(image.frame_at(0.0), Some("a"));
        assert_eq!(image.frame_at(1.0), Some("c"));
        assert_eq!(image.frame_at(1.5), Some("a"));
        assert_eq!(image.once().frame_at(10.0), Some("c"));
    }

    #[test]
    fn plugin_paths_resolve_to_the_plugin_folder() {
        let mut window = Window::new("w", "W", 200, Node::column([Node::image(ImageRef::plugin("coin.png"), 16, 16)]));
        window.resolve_assets("shop");
        let Node::Image { image, .. } = window.nodes()[1] else { panic!() };
        assert_eq!(image.frames, vec!["plugin/shop/coin.png"]);
    }

    #[test]
    fn a_window_round_trips_through_json() {
        let window = Window::new(
            "shop",
            "Shop",
            300,
            Node::column([
                Node::title("Stock").color("ffcc00"),
                Table::new("stock", 5).column("Item", 120, Align::Start).row("501", [Node::item(501), Node::button("buy:501", "Buy")]).into(),
                Node::number_input("amount", 1, 1, 99),
            ]),
        );
        let json = serde_json::to_string(&window).unwrap();
        assert_eq!(serde_json::from_str::<Window>(&json).unwrap(), window);
        assert!(window.widget("buy:501").is_some());
        assert!(window.widget("amount").is_some());
    }
}
