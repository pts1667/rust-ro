use std::collections::HashSet;

use crate::{ImageRef, MotionKind, Node, Overhead, Window};

/// Bounds the server holds plugin UI to, so a plugin cannot make a client allocate or draw without limit.
pub struct Limits {
    pub window_json_bytes: usize,
    pub nodes: usize,
    pub depth: usize,
    pub table_rows: usize,
    pub table_columns: usize,
    pub text_chars: usize,
    pub label_chars: usize,
    pub id_chars: usize,
    pub frames: usize,
    pub path_chars: usize,
    pub image_side: u16,
    pub overhead_ms: u32,
    pub motions: usize,
}

pub const LIMITS: Limits = Limits {
    window_json_bytes: 32 * 1024,
    nodes: 512,
    depth: 16,
    table_rows: 200,
    table_columns: 12,
    text_chars: 1024,
    label_chars: 64,
    id_chars: 64,
    frames: 64,
    path_chars: 128,
    image_side: 512,
    overhead_ms: 10_000,
    motions: 16,
};

/// Checks `window` against [`LIMITS`] and turns its `@/` image paths into `plugin`'s asset paths.
pub fn validate_window(window: &mut Window, plugin: &str) -> Result<(), String> {
    window.resolve_assets(plugin);
    check_id("window id", &window.id)?;
    check_chars("title", &window.title, LIMITS.label_chars)?;
    if !(80..=1024).contains(&window.width) {
        return Err(format!("window width {} is outside 80..=1024", window.width));
    }
    if window.height.is_some_and(|height| height > 768) {
        return Err("window height is over 768".into());
    }
    let mut checker = Checker { plugin, nodes: 0, widgets: HashSet::new() };
    checker.node(&window.node, 1)?;
    let bytes = serde_json::to_vec(window).map_err(|error| error.to_string())?.len();
    if bytes > LIMITS.window_json_bytes {
        return Err(format!("window is {bytes} bytes of JSON, over {}", LIMITS.window_json_bytes));
    }
    Ok(())
}

/// Checks `overhead` against [`LIMITS`] and turns its `@/` image paths into `plugin`'s asset paths.
pub fn validate_overhead(overhead: &mut Overhead, plugin: &str) -> Result<(), String> {
    if overhead.image.is_none() && overhead.text.is_none() {
        return Err("an overhead needs an image or a text".into());
    }
    if overhead.duration_ms == 0 || overhead.duration_ms > LIMITS.overhead_ms {
        return Err(format!("overhead duration must be within 1..={} ms", LIMITS.overhead_ms));
    }
    if let Some(image) = &mut overhead.image {
        image.resolve(plugin);
        check_image(image, plugin)?;
        check_side(overhead.width)?;
        check_side(overhead.height)?;
    }
    if let Some(text) = &overhead.text {
        check_chars("overhead text", &text.text, LIMITS.label_chars)?;
        check_color(text.color.as_deref())?;
    }
    if overhead.motions.len() > LIMITS.motions {
        return Err(format!("more than {} motions", LIMITS.motions));
    }
    for motion in &overhead.motions {
        if motion.ms > LIMITS.overhead_ms || motion.start_ms.is_some_and(|start| start > LIMITS.overhead_ms) {
            return Err(format!("motion timing over {} ms", LIMITS.overhead_ms));
        }
        let sane = |value: f32, bound: f32| value.is_finite() && value.abs() <= bound;
        let ok = match motion.kind {
            MotionKind::Rise { px } | MotionKind::Shake { px } | MotionKind::Bounce { px } => sane(px, 256.0),
            MotionKind::Scale { from, to } => sane(from, 8.0) && sane(to, 8.0),
            MotionKind::FadeIn | MotionKind::FadeOut | MotionKind::Pop => true,
        };
        if !ok {
            return Err(format!("motion {:?} is out of bounds", motion.kind));
        }
    }
    Ok(())
}

struct Checker<'a> {
    plugin: &'a str,
    nodes: usize,
    widgets: HashSet<String>,
}

impl Checker<'_> {
    fn node(&mut self, node: &Node, depth: usize) -> Result<(), String> {
        self.nodes += 1;
        if self.nodes > LIMITS.nodes {
            return Err(format!("more than {} nodes", LIMITS.nodes));
        }
        if depth > LIMITS.depth {
            return Err(format!("nodes nested deeper than {}", LIMITS.depth));
        }
        if let Some(id) = node.widget_id() {
            check_id("widget id", id)?;
            if !self.widgets.insert(id.to_string()) {
                return Err(format!("widget id {id} is used twice"));
            }
        }
        match node {
            Node::Text { text, color, .. } => {
                check_chars("text", text, LIMITS.text_chars)?;
                check_color(color.as_deref())?;
            }
            Node::Image { image, width, height } => {
                check_image(image, self.plugin)?;
                check_side(*width)?;
                check_side(*height)?;
            }
            Node::Button { label, .. } => check_chars("label", label, LIMITS.label_chars)?,
            Node::NumberInput { value, min, max, .. } => {
                if min > max || value < min || value > max {
                    return Err(format!("number input needs min <= value <= max, got {min} <= {value} <= {max}"));
                }
            }
            Node::TextInput { value, max_len, .. } => {
                if *max_len == 0 || *max_len as usize > LIMITS.text_chars {
                    return Err(format!("text input max_len must be within 1..={}", LIMITS.text_chars));
                }
                check_chars("text input value", value, *max_len as usize)?;
            }
            Node::Gauge { color, .. } => check_color(color.as_deref())?,
            Node::Table(table) => {
                if table.columns.is_empty() || table.columns.len() > LIMITS.table_columns {
                    return Err(format!("table {} needs 1..={} columns", table.id, LIMITS.table_columns));
                }
                if table.rows.len() > LIMITS.table_rows {
                    return Err(format!("table {} has more than {} rows", table.id, LIMITS.table_rows));
                }
                if table.visible_rows == 0 {
                    return Err(format!("table {} shows no rows", table.id));
                }
                for column in &table.columns {
                    check_chars("column title", &column.title, LIMITS.label_chars)?;
                }
                let mut rows = HashSet::new();
                for row in &table.rows {
                    check_id("row id", &row.id)?;
                    if !rows.insert(&row.id) {
                        return Err(format!("table {} has row {} twice", table.id, row.id));
                    }
                    if row.cells.len() > table.columns.len() {
                        return Err(format!("row {} of table {} has more cells than columns", row.id, table.id));
                    }
                }
            }
            Node::Column(_) | Node::Row(_) | Node::Spacer { .. } | Node::Separator | Node::ItemIcon { .. } => {}
        }
        for child in node.children() {
            self.node(child, depth + 1)?;
        }
        Ok(())
    }
}

fn check_chars(what: &str, text: &str, limit: usize) -> Result<(), String> {
    if text.chars().count() > limit {
        return Err(format!("{what} is longer than {limit} characters"));
    }
    if text.chars().any(|c| c.is_control() && c != '\n') {
        return Err(format!("{what} contains control characters"));
    }
    Ok(())
}

fn check_id(what: &str, id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err(format!("{what} is empty"));
    }
    check_chars(what, id, LIMITS.id_chars)
}

fn check_color(color: Option<&str>) -> Result<(), String> {
    match color {
        Some(color) if color.len() != 6 || !color.bytes().all(|b| b.is_ascii_hexdigit()) => Err(format!("colour {color} is not RRGGBB")),
        _ => Ok(()),
    }
}

fn check_side(side: u16) -> Result<(), String> {
    if side > LIMITS.image_side {
        return Err(format!("image side {side} is over {}", LIMITS.image_side));
    }
    Ok(())
}

fn check_image(image: &ImageRef, plugin: &str) -> Result<(), String> {
    if image.frames.is_empty() || image.frames.len() > LIMITS.frames {
        return Err(format!("an image needs 1..={} frames", LIMITS.frames));
    }
    if !image.fps.is_finite() || !(0.0..=60.0).contains(&image.fps) {
        return Err("image fps must be within 0..=60".into());
    }
    let own = format!("plugin/{plugin}/");
    for frame in &image.frames {
        check_chars("image path", frame, LIMITS.path_chars)?;
        let escapes = frame.split(['/', '\\']).any(|part| part == "..") || frame.starts_with(['/', '\\']) || frame.contains(':');
        if frame.is_empty() || escapes {
            return Err(format!("image path {frame} is not a client path"));
        }
        if frame.starts_with("plugin/") && !frame.starts_with(&own) {
            return Err(format!("image path {frame} belongs to another plugin"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Align, Table};

    fn window(node: Node) -> Window {
        Window::new("w", "W", 200, node)
    }

    #[test]
    fn a_shop_window_passes() {
        let mut shop = window(Node::column([
            Node::image(ImageRef::plugin_animated(["c0.png", "c1.png"], 8.0), 24, 24),
            Table::new("stock", 6).column("Item", 120, Align::Start).column("", 60, Align::End).row("501", [Node::item(501), Node::button("buy:501", "Buy")]).into(),
            Node::number_input("amount", 1, 1, 30_000),
        ]));
        assert_eq!(validate_window(&mut shop, "ui_shop"), Ok(()));
    }

    #[test]
    fn duplicate_widget_ids_are_refused() {
        let mut twice = window(Node::column([Node::button("a", "A"), Node::button("a", "B")]));
        assert!(validate_window(&mut twice, "p").unwrap_err().contains("twice"));
    }

    #[test]
    fn paths_stay_inside_the_client_and_the_plugin() {
        for path in ["../x.png", "/etc/x", "C:x", "plugin/other/x.png", "a\\..\\b"] {
            let mut bad = window(Node::image(ImageRef::texture(path), 8, 8));
            assert!(validate_window(&mut bad, "mine").is_err(), "{path}");
        }
        let mut own = window(Node::image(ImageRef::texture("plugin/mine/x.png"), 8, 8));
        assert!(validate_window(&mut own, "mine").is_ok());
    }

    #[test]
    fn deep_or_large_trees_are_refused() {
        let mut deep = Node::text("x");
        for _ in 0..LIMITS.depth {
            deep = Node::column([deep]);
        }
        assert!(validate_window(&mut window(deep), "p").unwrap_err().contains("deeper"));
        let mut wide = window(Node::column((0..LIMITS.nodes).map(|_| Node::Separator)));
        assert!(validate_window(&mut wide, "p").unwrap_err().contains("nodes"));
    }

    #[test]
    fn bad_inputs_and_colours_are_refused() {
        assert!(validate_window(&mut window(Node::number_input("n", 5, 6, 9)), "p").is_err());
        assert!(validate_window(&mut window(Node::text("x").color("red")), "p").is_err());
        assert!(validate_window(&mut window(Node::text("bell\u{7}")), "p").is_err());
    }

    #[test]
    fn overheads_need_content_and_sane_motions() {
        assert!(validate_overhead(&mut Overhead::text("", None, 1000).rise(10.0, 100), "p").is_ok());
        let mut empty = Overhead::text("x", None, 1000);
        empty.text = None;
        assert!(validate_overhead(&mut empty, "p").is_err());
        assert!(validate_overhead(&mut Overhead::text("x", None, 60_000), "p").is_err());
        assert!(validate_overhead(&mut Overhead::text("x", None, 1000).rise(f32::NAN, 10), "p").is_err());
        let mut image = Overhead::image(ImageRef::plugin("up.png"), 64, 32, 1000);
        validate_overhead(&mut image, "shop").unwrap();
        assert_eq!(image.image.unwrap().frames, vec!["plugin/shop/up.png"]);
    }
}
