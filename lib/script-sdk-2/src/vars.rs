use script_sdk::{Request, Value, Variable, VariableScope};

use crate::ctx::Ctx;
use crate::flow::{Script, Stop};
use crate::value::Val;

impl<'a> Ctx<'a> {
    /// A variable by its rathena name: `Zeny`, `#gold`, `$@event_state`, `@menu`, `.npc_flag` or `'instance_flag`.
    ///
    /// The prefix picks the scope, and a trailing `$` makes it a text variable.
    pub fn var<'c>(&'c self, name: &'c str) -> Var<'c, 'a> {
        Var { ctx: self, name }
    }
}

/// A script variable, read and written through the host.
pub struct Var<'c, 'a> {
    ctx: &'c Ctx<'a>,
    name: &'c str,
}

impl Var<'_, '_> {
    pub fn get(&self) -> Result<Val, Stop> {
        self.get_at(0)
    }

    /// Reads element `index` of an array variable, such as `$list[index]`.
    pub fn get_at(&self, index: u32) -> Result<Val, Stop> {
        let (scope, name) = split_scope(self.name);
        if index == 0 && scope == VariableScope::Character {
            return self.ctx.request(Request::Read(self.name.into()));
        }
        self.ctx.request(Request::VariableRead {
            scope,
            name: name.into(),
            index,
        })
    }

    pub fn set(&self, value: impl Into<Val>) -> Script {
        self.set_at(0, value)
    }

    /// Writes element `index` of an array variable.
    pub fn set_at(&self, index: u32, value: impl Into<Val>) -> Script {
        let (scope, name) = split_scope(self.name);
        let value: Val = value.into();
        let value = coerce(self.name, value.into_value());
        if index == 0 && is_single_value_scope(scope) {
            return self
                .ctx
                .request(Request::Write {
                    name: self.name.into(),
                    value,
                })
                .map(|_| ());
        }
        self.ctx
            .request(Request::VariablesWrite(vec![Variable {
                scope,
                name: name.into(),
                index,
                value,
            }]))
            .map(|_| ())
    }
}

fn split_scope(name: &str) -> (VariableScope, &str) {
    if let Some(rest) = name.strip_prefix("##").or_else(|| name.strip_prefix('#')) {
        (VariableScope::Account, rest)
    } else if let Some(rest) = name.strip_prefix("$@") {
        (VariableScope::ServerTemporary, rest)
    } else if let Some(rest) = name.strip_prefix('$') {
        (VariableScope::Server, rest)
    } else if let Some(rest) = name.strip_prefix('@') {
        (VariableScope::CharacterTemporary, rest)
    } else if let Some(rest) = name.strip_prefix("''").or_else(|| name.strip_prefix('\'')) {
        (VariableScope::Instance, rest)
    } else if let Some(rest) = name.strip_prefix('.') {
        (VariableScope::NpcInstance, rest)
    } else {
        (VariableScope::Character, name)
    }
}

fn is_single_value_scope(scope: VariableScope) -> bool {
    matches!(
        scope,
        VariableScope::Character
            | VariableScope::CharacterTemporary
            | VariableScope::Account
            | VariableScope::Server
            | VariableScope::ServerTemporary
    )
}

/// Text variables, marked by a trailing `$`, always hold text, and every other variable holds a number.
fn coerce(name: &str, value: Value) -> Value {
    match (name.ends_with('$'), value) {
        (true, Value::Number(number)) => Value::String(number.to_string()),
        (false, Value::String(text)) => Value::Number(text.trim().parse().unwrap_or(0)),
        (_, value) => value,
    }
}

#[cfg(test)]
mod tests {
    use script_sdk::{Request, Value, Variable, VariableScope};

    use crate::ctx::Ctx;
    use crate::transport::MockTransport;

    #[test]
    fn prefix_picks_the_scope() {
        assert_eq!(super::split_scope("#gold"), (VariableScope::Account, "gold"));
        assert_eq!(super::split_scope("##gold"), (VariableScope::Account, "gold"));
        assert_eq!(super::split_scope("$@tmp"), (VariableScope::ServerTemporary, "tmp"));
        assert_eq!(super::split_scope("$world"), (VariableScope::Server, "world"));
        assert_eq!(super::split_scope("@menu"), (VariableScope::CharacterTemporary, "menu"));
        assert_eq!(super::split_scope("'flag"), (VariableScope::Instance, "flag"));
        assert_eq!(super::split_scope(".flag"), (VariableScope::NpcInstance, "flag"));
        assert_eq!(super::split_scope("Zeny"), (VariableScope::Character, "Zeny"));
    }

    #[test]
    fn text_variables_store_text_and_numbers_store_numbers() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.var("name$").set(7).unwrap();
        ctx.var("level").set("12").unwrap();
        assert_eq!(
            transport.requests(),
            vec![
                Request::Write {
                    name: "name$".into(),
                    value: Value::String("7".into())
                },
                Request::Write {
                    name: "level".into(),
                    value: Value::Number(12)
                },
            ]
        );
    }

    #[test]
    fn array_elements_use_the_stripped_name() {
        let transport = MockTransport::silent();
        let ctx = Ctx::new(&transport);
        ctx.var("#visits").set_at(3, 1).unwrap();
        ctx.var("#visits").get_at(3).unwrap();
        assert_eq!(
            transport.requests(),
            vec![
                Request::VariablesWrite(vec![Variable {
                    scope: VariableScope::Account,
                    name: "visits".into(),
                    index: 3,
                    value: Value::Number(1)
                }]),
                Request::VariableRead {
                    scope: VariableScope::Account,
                    name: "visits".into(),
                    index: 3
                },
            ]
        );
    }
}
