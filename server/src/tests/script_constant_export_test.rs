use script_sdk::Value;

use crate::server::script::item_script_handler::constant;

/// Writes the server's value for each constant name to JSON, for `tools/scripts-import/gen_sdk2_constants.py`.
/// Does nothing unless both `SCRIPT_CONSTANT_NAMES` and `SCRIPT_CONSTANT_VALUES` are set.
#[test]
fn export_script_constant_values() {
    let (Ok(names_path), Ok(values_path)) = (std::env::var("SCRIPT_CONSTANT_NAMES"), std::env::var("SCRIPT_CONSTANT_VALUES")) else {
        return;
    };
    let names = std::fs::read_to_string(names_path).expect("constant names file is readable");
    let values: serde_json::Map<String, serde_json::Value> = names
        .lines()
        .filter(|name| !name.is_empty())
        .map(|name| {
            let value = match constant(name) {
                Ok(Value::Number(number)) => serde_json::json!(number),
                Ok(Value::String(text)) => serde_json::json!(text),
                _ => serde_json::Value::Null,
            };
            (name.to_string(), value)
        })
        .collect();
    std::fs::write(values_path, serde_json::to_string_pretty(&values).expect("constant values serialise")).expect("constant values file is writable");
}
