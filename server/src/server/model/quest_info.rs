//! NPC quest icons (`questinfo`): the icon an NPC shows while a condition holds for the player.
//! Conditions are the C-like integer expressions of the stock scripts, e.g. `!isbegin_quest(7712) && BaseLevel >= 6`.

use std::collections::HashMap;

/// Functions a condition may call.
const FUNCTIONS: [&str; 3] = ["isbegin_quest", "checkquest", "countitem"];

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Number(i64),
    Name(String),
    Call(String, Vec<Condition>),
    Not(Box<Condition>),
    Negate(Box<Condition>),
    Binary(&'static str, Box<Condition>, Box<Condition>),
}

pub trait ConditionEnvironment {
    fn name(&self, name: &str) -> Result<i64, String>;
    fn call(&self, function: &str, arguments: &[i64]) -> Result<i64, String>;
}

impl Condition {
    pub fn parse(text: &str) -> Result<Self, String> {
        let tokens = tokenize(text)?;
        let mut parser = Parser { tokens, position: 0 };
        let condition = parser.or()?;
        match parser.tokens.get(parser.position) {
            None => Ok(condition),
            Some(token) => Err(format!("unexpected '{token}' in condition '{text}'")),
        }
    }

    pub fn evaluate(&self, environment: &dyn ConditionEnvironment) -> Result<i64, String> {
        Ok(match self {
            Self::Number(value) => *value,
            Self::Name(name) => environment.name(name)?,
            Self::Call(function, arguments) => {
                let values = arguments.iter().map(|argument| argument.evaluate(environment)).collect::<Result<Vec<_>, _>>()?;
                environment.call(function, &values)?
            }
            Self::Not(inner) => i64::from(inner.evaluate(environment)? == 0),
            Self::Negate(inner) => inner.evaluate(environment)?.wrapping_neg(),
            Self::Binary("&&", left, right) => i64::from(left.evaluate(environment)? != 0 && right.evaluate(environment)? != 0),
            Self::Binary("||", left, right) => i64::from(left.evaluate(environment)? != 0 || right.evaluate(environment)? != 0),
            Self::Binary(operator, left, right) => {
                let (a, b) = (left.evaluate(environment)?, right.evaluate(environment)?);
                match *operator {
                    "==" => i64::from(a == b),
                    "!=" => i64::from(a != b),
                    "<" => i64::from(a < b),
                    "<=" => i64::from(a <= b),
                    ">" => i64::from(a > b),
                    ">=" => i64::from(a >= b),
                    "+" => a.wrapping_add(b),
                    "-" => a.wrapping_sub(b),
                    "*" => a.wrapping_mul(b),
                    "/" => a.checked_div(b).ok_or("division by zero in condition")?,
                    "%" => a.checked_rem(b).ok_or("division by zero in condition")?,
                    other => return Err(format!("unknown operator {other}")),
                }
            }
        })
    }
}

fn tokenize(text: &str) -> Result<Vec<String>, String> {
    let characters: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if character.is_whitespace() {
            index += 1;
        } else if character.is_ascii_alphanumeric() || character == '_' {
            let start = index;
            while index < characters.len() && (characters[index].is_ascii_alphanumeric() || characters[index] == '_') {
                index += 1;
            }
            tokens.push(characters[start..index].iter().collect());
        } else if let Some(operator) = ["&&", "||", "==", "!=", "<=", ">="].into_iter().find(|operator| text_at(&characters, index, operator)) {
            tokens.push(operator.to_string());
            index += 2;
        } else if "!<>+-*/%(),".contains(character) {
            tokens.push(character.to_string());
            index += 1;
        } else {
            return Err(format!("unsupported character '{character}' in condition '{text}'"));
        }
    }
    Ok(tokens)
}

fn text_at(characters: &[char], index: usize, pattern: &str) -> bool {
    pattern.chars().enumerate().all(|(offset, expected)| characters.get(index + offset) == Some(&expected))
}

struct Parser {
    tokens: Vec<String>,
    position: usize,
}

impl Parser {
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.position).map(String::as_str)
    }

    fn take(&mut self, expected: &str) -> bool {
        if self.peek() == Some(expected) {
            self.position += 1;
            return true;
        }
        false
    }

    fn operator(&mut self, operators: &[&'static str]) -> Option<&'static str> {
        let found = operators.iter().copied().find(|operator| self.peek() == Some(operator))?;
        self.position += 1;
        Some(found)
    }

    fn binary(&mut self, operators: &[&'static str], next: fn(&mut Self) -> Result<Condition, String>) -> Result<Condition, String> {
        let mut left = next(self)?;
        while let Some(operator) = self.operator(operators) {
            left = Condition::Binary(operator, Box::new(left), Box::new(next(self)?));
        }
        Ok(left)
    }

    fn or(&mut self) -> Result<Condition, String> {
        self.binary(&["||"], Self::and)
    }

    fn and(&mut self) -> Result<Condition, String> {
        self.binary(&["&&"], Self::equality)
    }

    fn equality(&mut self) -> Result<Condition, String> {
        self.binary(&["==", "!="], Self::relation)
    }

    fn relation(&mut self) -> Result<Condition, String> {
        self.binary(&["<=", ">=", "<", ">"], Self::sum)
    }

    fn sum(&mut self) -> Result<Condition, String> {
        self.binary(&["+", "-"], Self::product)
    }

    fn product(&mut self) -> Result<Condition, String> {
        self.binary(&["*", "/", "%"], Self::unary)
    }

    fn unary(&mut self) -> Result<Condition, String> {
        if self.take("!") {
            return Ok(Condition::Not(Box::new(self.unary()?)));
        }
        if self.take("-") {
            return Ok(Condition::Negate(Box::new(self.unary()?)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Condition, String> {
        let token = self.peek().ok_or("condition ends unexpectedly")?.to_string();
        self.position += 1;
        if token == "(" {
            let inner = self.or()?;
            return if self.take(")") { Ok(inner) } else { Err("missing ')' in condition".into()) };
        }
        let first = token.chars().next().unwrap_or(' ');
        if first.is_ascii_digit() {
            let number = match token.strip_prefix("0x") {
                Some(hex) => i64::from_str_radix(hex, 16),
                None => token.parse(),
            };
            return number.map(Condition::Number).map_err(|_| format!("invalid number '{token}' in condition"));
        }
        if !(first.is_ascii_alphabetic() || first == '_') {
            return Err(format!("unexpected '{token}' in condition"));
        }
        if !self.take("(") {
            return Ok(Condition::Name(token));
        }
        if !FUNCTIONS.contains(&token.to_lowercase().as_str()) {
            return Err(format!("function '{token}' is not supported in conditions"));
        }
        let mut arguments = Vec::new();
        if !self.take(")") {
            loop {
                arguments.push(self.or()?);
                if self.take(")") {
                    break;
                }
                if !self.take(",") {
                    return Err(format!("missing ')' after the arguments of '{token}'"));
                }
            }
        }
        Ok(Condition::Call(token.to_lowercase(), arguments))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuestInfoEntry {
    pub icon: u16,
    pub color: u8,
    pub condition: Option<Condition>,
}

/// Where a NPC lives: map name, instance and NPC id.
pub type NpcKey = (String, u8, u32);

/// Icons registered by NPC scripts, evaluated in the order they were registered.
#[derive(Debug, Default)]
pub struct QuestInfos {
    npcs: HashMap<NpcKey, Vec<QuestInfoEntry>>,
}

impl QuestInfos {
    pub fn register(&mut self, key: NpcKey, entry: QuestInfoEntry) {
        self.npcs.entry(key).or_default().push(entry);
    }

    pub fn on_map<'a>(&'a self, map: &'a str, instance: u8) -> impl Iterator<Item = (u32, &'a [QuestInfoEntry])> + 'a {
        self.npcs.iter().filter(move |((npc_map, npc_instance, _), _)| npc_map == map && *npc_instance == instance).map(|((_, _, npc_id), entries)| (*npc_id, entries.as_slice()))
    }

    pub fn remove_npc(&mut self, npc_id: u32) {
        self.npcs.retain(|(_, _, id), _| *id != npc_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixed;

    impl ConditionEnvironment for Fixed {
        fn name(&self, name: &str) -> Result<i64, String> {
            Ok(match name {
                "BaseLevel" => 12,
                "HUNTING" => 2,
                _ => 0,
            })
        }

        fn call(&self, function: &str, arguments: &[i64]) -> Result<i64, String> {
            Ok(match (function, arguments) {
                ("isbegin_quest", [7712]) => 1,
                ("isbegin_quest", _) => 0,
                ("checkquest", [7713, 2]) => 2,
                _ => -1,
            })
        }
    }

    fn evaluate(text: &str) -> i64 {
        Condition::parse(text).unwrap().evaluate(&Fixed).unwrap()
    }

    #[test]
    fn stock_conditions_evaluate() {
        assert_eq!(evaluate("!isbegin_quest(7719)"), 1);
        assert_eq!(evaluate("isbegin_quest(7712) > 0 && (checkquest(7713,HUNTING) == -1 || checkquest(7713,HUNTING) == 2)"), 1);
        assert_eq!(evaluate("!isbegin_quest(7712) && BaseLevel >= 6 && BaseLevel <= 9"), 0);
        assert_eq!(evaluate(" BaseLevel == 10 + 2 "), 1);
        assert!(Condition::parse("getarraysize($@a)").is_err());
        assert!(Condition::parse("BaseLevel >=").is_err());
    }
}
