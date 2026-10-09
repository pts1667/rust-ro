//! Helpers that rathena scripts call on values, texts, arrays and local variables. They follow the rules of the legacy
//! runtime, so a converted script behaves like the one it replaces.

use script_sdk::{Function, Value};

use crate::ctx::Ctx;
use crate::flow::Stop;
use crate::value::Val;

/// rathena's binary operators. Two texts order by text; a number and a text compare by their text for `==` and `!=`;
/// everything else is numeric.
pub fn op(left: &Val, operator: &str, right: &Val) -> Result<Val, Stop> {
    if let (Value::String(a), Value::String(b)) = (left.value(), right.value()) {
        let ordering = match operator {
            "<" => Some(a < b),
            ">" => Some(a > b),
            "<=" => Some(a <= b),
            ">=" => Some(a >= b),
            _ => None,
        };
        if let Some(result) = ordering {
            return Ok(Val::from(result));
        }
    }
    if matches!(operator, "==" | "!=") && left.is_number() != right.is_number() {
        return Ok(Val::from((left.text() == right.text()) == (operator == "==")));
    }
    left.clone().binary(operator, right.clone())
}

/// A default element of an array: `""` for a text array, `0` otherwise.
fn default_value(text: bool) -> Val {
    if text { Val::from("") } else { Val::from(0) }
}

/// The element `index` of a local array; anything out of range reads as the array's default.
pub fn local_get(array: &[Val], index: &Val, text: bool) -> Val {
    index.number().ok().and_then(|index| usize::try_from(index).ok()).and_then(|index| array.get(index).cloned()).unwrap_or_else(|| default_value(text))
}

/// Stores `value` at `index` of a local array, growing it with defaults. Negative and huge indexes are ignored.
pub fn local_set(array: &mut Vec<Val>, index: &Val, value: Val, text: bool) {
    let Some(index) = index.number().ok().and_then(|index| usize::try_from(index).ok()) else { return };
    if index > 65_535 {
        return;
    }
    if array.len() <= index {
        array.resize(index + 1, default_value(text));
    }
    array[index] = value;
}

/// `deletearray name[start], count`: the entries after the deleted ones move up; without `count` everything from `start` on goes.
fn remove_range(values: &mut Vec<Val>, start: &Val, count: Option<&Val>) -> Result<(), Stop> {
    let start = usize::try_from(start.number()?).unwrap_or(0).min(values.len());
    let end = match count {
        Some(count) => start.saturating_add(usize::try_from(count.number()?).unwrap_or(0)).min(values.len()),
        None => values.len(),
    };
    values.drain(start..end);
    Ok(())
}

pub fn local_delete(values: &mut Vec<Val>, start: &Val, count: Option<&Val>) -> Result<(), Stop> {
    remove_range(values, start, count)
}

/// `count` elements of `source` from `start`, missing ones read as the array's default.
pub fn slice_of(source: &[Val], start: &Val, count: &Val, text: bool) -> Result<Vec<Val>, Stop> {
    let (start, count) = (usize::try_from(start.number()?).unwrap_or(0), usize::try_from(count.number()?).unwrap_or(0).min(65_535));
    Ok((start..start + count).map(|index| source.get(index).cloned().unwrap_or_else(|| default_value(text))).collect())
}

pub fn local_splice(target: &mut Vec<Val>, start: &Val, values: Vec<Val>, text: bool) -> Result<(), Stop> {
    let start = usize::try_from(start.number()?).unwrap_or(0);
    for (offset, value) in values.into_iter().enumerate() {
        local_set(target, &Val::from((start + offset) as i32), value, text);
    }
    Ok(())
}

/// Every element of a stored array (`@x[]`, `$x[]`, `#x[]`, `.x[]`).
pub fn array_values(ctx: &Ctx, name: &str) -> Result<Vec<Val>, Stop> {
    ctx.call(Function::ArrayGet, vec![Val::from(name)])?.into_array().ok_or_else(|| Stop::from("Invalid array contents".to_string()))
}

/// Replaces a stored array: entries past the new length are reset.
pub fn array_store(ctx: &Ctx, name: &str, values: Vec<Val>) -> Result<(), Stop> {
    ctx.call(Function::ArraySet, vec![Val::from(name), Val::from(Value::Array(values.into_iter().map(Val::into_value).collect()))]).map(|_| ())
}

pub fn array_size(ctx: &Ctx, name: &str) -> Result<Val, Stop> {
    Ok(Val::from(array_values(ctx, name)?.len() as i32))
}

/// Whether a stored array name holds texts, from its `$` suffix.
fn is_text(name: &str) -> bool {
    name.trim_end_matches(']').split('[').next().is_some_and(|base| base.ends_with('$'))
}

pub fn array_delete(ctx: &Ctx, name: &str, start: &Val, count: Option<&Val>) -> Result<(), Stop> {
    let mut values = array_values(ctx, name)?;
    remove_range(&mut values, start, count)?;
    array_store(ctx, name, values)
}

pub fn array_splice(ctx: &Ctx, name: &str, start: &Val, values: Vec<Val>) -> Result<(), Stop> {
    let mut target = array_values(ctx, name)?;
    local_splice(&mut target, start, values, is_text(name))?;
    array_store(ctx, name, target)
}

pub fn array_implode(ctx: &Ctx, name: &str, delimiter: &Val) -> Result<Val, Stop> {
    implode(&array_values(ctx, name)?, delimiter)
}

/// A local variable a script names at run time through [`getd`] and [`setd`].
pub enum Local<'a> {
    Scalar(&'a Val),
    Array(&'a Vec<Val>),
}

pub enum LocalMut<'a> {
    Scalar(&'a mut Val),
    Array(&'a mut Vec<Val>),
}

fn array_index(index: i32) -> Result<u32, Stop> {
    u32::try_from(index).map_err(|_| Stop::from("Negative array index".to_string()))
}

/// `name[index]` as its lower-case base and index.
fn split_name(name: &Val) -> Result<(String, i32), Stop> {
    let text = name.text().to_lowercase();
    match text.split_once('[') {
        Some((base, rest)) => Ok((
            base.trim().to_string(),
            rest.trim_end_matches(']').trim().parse().map_err(|_| Stop::from("Invalid array index in a variable name".to_string()))?,
        )),
        None => Ok((text.trim().to_string(), 0)),
    }
}

/// Reads a variable whose name is computed: a constant, a stored variable, or one of the script's locals.
pub fn getd(ctx: &Ctx, name: &Val, locals: &[(&str, Local)]) -> Result<Val, Stop> {
    let (base, index) = split_name(name)?;
    let written = name.text();
    if written.starts_with(|first: char| first.is_ascii_uppercase()) && written.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_') {
        if let Ok(value) = ctx.constant(&written) {
            return Ok(value);
        }
    }
    if !base.starts_with(".@") {
        return ctx.var(&base).get_at(array_index(index)?);
    }
    Ok(match locals.iter().find(|(known, _)| *known == base) {
        Some((_, Local::Scalar(value))) if index == 0 => (*value).clone(),
        Some((_, Local::Array(values))) => local_get(values, &Val::from(index), base.ends_with('$')),
        _ => default_value(base.ends_with('$')),
    })
}

/// `getarraysize(getd("name"))`.
pub fn getd_size(ctx: &Ctx, name: &Val, locals: &[(&str, Local)]) -> Result<Val, Stop> {
    let (base, _) = split_name(name)?;
    if !base.starts_with(".@") {
        return array_size(ctx, &base);
    }
    Ok(Val::from(match locals.iter().find(|(known, _)| *known == base) {
        Some((_, Local::Array(values))) => values.len() as i32,
        Some((_, Local::Scalar(value))) => i32::from(value.is_true()),
        None => 0,
    }))
}

/// Writes a variable whose name is computed. A local the script does not declare is an error.
pub fn setd(ctx: &Ctx, name: &Val, value: Val, locals: &mut [(&str, LocalMut)]) -> Result<(), Stop> {
    let (base, index) = split_name(name)?;
    if !base.starts_with(".@") {
        return ctx.var(&base).set_at(array_index(index)?, value);
    }
    match locals.iter_mut().find(|(known, _)| *known == base) {
        Some((_, LocalMut::Scalar(target))) if index == 0 => **target = value,
        Some((_, LocalMut::Array(values))) => local_set(values, &Val::from(index), value, base.ends_with('$')),
        _ => return Err(Stop::from(format!("setd cannot reach the local variable {base}"))),
    }
    Ok(())
}

/// The `:` separated options of `select` and `menu`, empty ones skipped.
pub fn menu_options(options: &[&str]) -> Vec<Val> {
    options.iter().flat_map(|option| option.split(':')).filter(|option| !option.is_empty()).map(Val::from).collect()
}

/// `select` and `menu`: the answer is 1-based and is stored in `@menu`. An answer outside the options cancels the conversation.
pub fn select(ctx: &Ctx, options: &[&str]) -> Result<i32, Stop> {
    let options = menu_options(options);
    let selected = ctx.call(Function::Select, options.clone())?.number()?;
    if selected < 1 || selected as usize > options.len() {
        return Err(Stop::from("Conversation cancelled".to_string()));
    }
    Ok(selected)
}

/// [`select`] for options computed at run time.
pub fn select_values(ctx: &Ctx, options: &[Val]) -> Result<i32, Stop> {
    let texts: Vec<String> = options.iter().map(Val::text).collect();
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    select(ctx, &refs)
}

/// The array index of a value, as the stored array accessors take it.
pub fn index(value: &Val) -> Result<u32, Stop> {
    array_index(value.number()?)
}

/// `getarg(index, default)`: the script argument at `index`, or `default` when there is none.
pub fn arg(args: &[Val], index: i32, default: Val) -> Val {
    usize::try_from(index).ok().and_then(|index| args.get(index).cloned()).unwrap_or(default)
}

/// `set getarg(index), value`: changes an argument of this call only. Slots past the end read as 0 once written.
pub fn set_arg(args: &mut Vec<Val>, index: i32, value: Val) -> Result<(), Stop> {
    let index = usize::try_from(index).map_err(|_| Stop::from("Argument index is negative".to_string()))?;
    if args.len() <= index {
        args.resize(index + 1, Val::from(0));
    }
    args[index] = value;
    Ok(())
}

/// `npcskill`: the three skills the legacy runtime supports, each with its legacy formula.
pub fn npc_skill(ctx: &Ctx, skill: &Val, level: &Val, stat: &Val, npc_level: &Val) -> Result<(), Stop> {
    let (level, stat, npc_level) = (level.number()?, stat.number()?, npc_level.number()?);
    match skill.text().to_ascii_uppercase().as_str() {
        "AL_HEAL" => {
            let hp = (npc_level + stat) / 8 * (4 + 8 * level);
            ctx.call(Function::Heal, vec![Val::from(hp), Val::from(0)])?;
        }
        buff @ ("AL_BLESSING" | "AL_INCAGI") => {
            let status = if buff == "AL_BLESSING" { "SC_BLESSING" } else { "SC_INCREASEAGI" };
            ctx.call(Function::StartStatus, vec![ctx.constant(status)?, Val::from(40_000 + 20_000 * level), Val::from(level)])?;
        }
        other => return Err(Stop::from(format!("npcskill does not support {other}"))),
    }
    Ok(())
}

/// `getpartymember(party, kind)`: the members into `$@partymembercid`, `$@partymemberaid` or `$@partymembername$`, and their count.
pub fn party_members(ctx: &Ctx, party: Val, kind: Val) -> Result<(), Stop> {
    let array = match kind.number()? {
        1 => "$@partymembercid",
        2 => "$@partymemberaid",
        _ => "$@partymembername$",
    };
    let members = ctx.call(Function::GetPartyMember, vec![party, kind])?.into_array().ok_or_else(|| Stop::from("Invalid party member list".to_string()))?;
    ctx.var("$@partymembercount").set(members.len() as i32)?;
    for (position, member) in members.into_iter().enumerate() {
        ctx.var(array).set_at(position as u32, member)?;
    }
    Ok(())
}

/// `getinventorylist`: one `@inventorylist_<column>` array per column, and `@inventorylist_count` rows.
pub fn inventory_list(ctx: &Ctx) -> Result<(), Stop> {
    const COLUMNS: [&str; 10] = ["id", "amount", "equip", "refine", "identify", "attribute", "card1", "card2", "card3", "card4"];
    let rows = ctx.call(Function::GetInventoryList, vec![])?.into_array().ok_or_else(|| Stop::from("Invalid inventory list".to_string()))?;
    for (column, name) in COLUMNS.iter().enumerate() {
        array_store(ctx, &format!("@inventorylist_{name}"), rows.chunks_exact(COLUMNS.len()).map(|row| row[column].clone()).collect())?;
    }
    ctx.var("@inventorylist_count").set((rows.len() / COLUMNS.len()) as i32)
}

/// `input` of a number: the entry clamped to `[min, max]` (default `0` to `10000000`), and the status: -1 below the minimum, 1 above the maximum, 0 inside.
pub fn input_number(ctx: &Ctx, min: Option<i32>, max: Option<i32>) -> Result<(Val, i32), Stop> {
    let input = ctx.input_number(min.unwrap_or(0), max.unwrap_or(10_000_000))?;
    Ok((Val::from(input.value), input.bound.code()))
}

/// `input` of a text: its length against the bounds (default `0` to `2047`), and the status as for [`input_number`].
pub fn input_text(ctx: &Ctx, min: Option<i32>, max: Option<i32>) -> Result<(Val, i32), Stop> {
    let length = |bound: Option<i32>, default: i32| bound.unwrap_or(default).max(0) as usize;
    let input = ctx.input_text(length(min, 0), length(max, 2047))?;
    Ok((Val::from(input.value), input.bound.code()))
}

pub fn strlen(text: &Val) -> Val {
    Val::from(text.text().chars().count() as i32)
}

/// The characters from `start` to `end`, both inclusive; an empty text when the range is out of bounds.
pub fn substr(text: &Val, start: &Val, end: &Val) -> Result<Val, Stop> {
    let characters: Vec<char> = text.text().chars().collect();
    let (start, end) = (start.number()?, end.number()?);
    if start < 0 || end < start || start as usize >= characters.len() {
        return Ok(Val::from(""));
    }
    let end = (end as usize).min(characters.len() - 1);
    Ok(Val::from(characters[start as usize..=end].iter().collect::<String>()))
}

pub fn charat(text: &Val, index: &Val) -> Result<Val, Stop> {
    let index = index.number()?;
    Ok(Val::from(usize::try_from(index).ok().and_then(|index| text.text().chars().nth(index)).map(String::from).unwrap_or_default()))
}

pub fn atoi(text: &Val) -> Val {
    Val::from(text.text().trim().parse::<i32>().unwrap_or(0))
}

/// Case-insensitive "contains", as rathena's `compare`.
pub fn compare(haystack: &Val, needle: &Val) -> Val {
    Val::from(haystack.text().to_lowercase().contains(&needle.text().to_lowercase()))
}

pub fn strtolower(text: &Val) -> Val {
    Val::from(text.text().to_lowercase())
}

pub fn strtoupper(text: &Val) -> Val {
    Val::from(text.text().to_uppercase())
}

/// Index of the first match at or after `start`, -1 when there is none.
pub fn strpos(text: &Val, search: &Val, start: Option<&Val>) -> Result<Val, Stop> {
    let (text, search) = (text.text(), search.text());
    let start = start.map_or(Ok(0), Val::number)?.max(0) as usize;
    let characters: Vec<char> = text.chars().collect();
    let needle: Vec<char> = search.chars().collect();
    Ok(Val::from((start..=characters.len().saturating_sub(needle.len())).find(|position| characters.get(*position..*position + needle.len()) == Some(needle.as_slice())).map_or(-1, |position| position as i32)))
}

pub fn charisalpha(text: &Val, index: &Val) -> Result<Val, Stop> {
    Ok(Val::from(usize::try_from(index.number()?).ok().and_then(|index| text.text().chars().nth(index)).is_some_and(char::is_alphabetic)))
}

/// Inserts the first character of `character` at `index`; a negative index counts from the end.
pub fn insertchar(text: &Val, character: &Val, index: &Val) -> Result<Val, Stop> {
    let characters: Vec<char> = text.text().chars().collect();
    let index = index.number()?;
    let position = if index < 0 { characters.len().saturating_sub(index.unsigned_abs() as usize) } else { (index as usize).min(characters.len()) };
    let mut result: String = characters[..position].iter().collect();
    result.push_str(&character.text().chars().take(1).collect::<String>());
    result.extend(&characters[position..]);
    Ok(Val::from(result))
}

/// `sprintf` with `%d`, `%i`, `%s` and `%%`, honouring the `-` and `0` flags and a width.
pub fn sprintf(format: &Val, arguments: &[Val]) -> Result<Val, Stop> {
    let format = format.text();
    let mut remaining = arguments.iter();
    let mut output = String::new();
    let mut characters = format.chars().peekable();
    while let Some(character) = characters.next() {
        if character != '%' {
            output.push(character);
            continue;
        }
        let (mut left, mut zero, mut width) = (false, false, 0usize);
        while let Some(&flag) = characters.peek() {
            match flag {
                '-' => left = true,
                '0' => zero = true,
                _ => break,
            }
            characters.next();
        }
        while let Some(digit) = characters.peek().and_then(|digit| digit.to_digit(10)) {
            width = width * 10 + digit as usize;
            characters.next();
        }
        let text = match characters.next() {
            Some('%') => {
                output.push('%');
                continue;
            }
            Some('d' | 'i') => remaining.next().map(Val::number).transpose()?.unwrap_or(0).to_string(),
            Some('s') => remaining.next().map(Val::text).unwrap_or_default(),
            _ => return Err(Stop::from("Unsupported sprintf format".to_string())),
        };
        let padding = " ".repeat(width.saturating_sub(text.chars().count()));
        match (left, zero) {
            (true, _) => output.extend([text.as_str(), &padding]),
            (false, true) => output.extend([&padding.replace(' ', "0"), text.as_str()]),
            (false, false) => output.extend([&padding, text.as_str()]),
        }
    }
    Ok(Val::from(output))
}

pub fn implode(values: &[Val], delimiter: &Val) -> Result<Val, Stop> {
    Ok(Val::from(values.iter().map(Val::text).collect::<Vec<_>>().join(&delimiter.text())))
}

pub fn explode(text: &Val, delimiter: &Val) -> Vec<Val> {
    let (text, delimiter) = (text.text(), delimiter.text());
    if delimiter.is_empty() {
        return vec![Val::from(text)];
    }
    text.split(delimiter.as_str()).map(Val::from).collect()
}

pub fn countstr(text: &Val, needle: &Val) -> Val {
    let needle = needle.text();
    Val::from(if needle.is_empty() { 0 } else { text.text().matches(needle.as_str()).count() as i32 })
}

/// `replacestr(text, search, replacement{, case sensitive, limit})`: case-insensitive unless the option says otherwise.
pub fn replacestr(text: &Val, search: &Val, replacement: &Val, options: &[Val]) -> Result<Val, Stop> {
    let (text, search, replacement) = (text.text(), search.text(), replacement.text());
    let case_sensitive = options.first().map_or(Ok(1), Val::number)? != 0;
    let limit = options.get(1).map_or(Ok(-1), Val::number)?;
    if search.is_empty() {
        return Ok(Val::from(text));
    }
    let (haystack, needle) = if case_sensitive { (text.clone(), search.clone()) } else { (text.to_ascii_lowercase(), search.to_ascii_lowercase()) };
    let mut result = String::new();
    let (mut cursor, mut replaced) = (0, 0);
    while limit < 0 || replaced < limit {
        let Some(found) = haystack[cursor..].find(&needle) else { break };
        result.push_str(&text[cursor..cursor + found]);
        result.push_str(&replacement);
        cursor += found + needle.len();
        replaced += 1;
    }
    result.push_str(&text[cursor..]);
    Ok(Val::from(result))
}

pub fn delchar(text: &Val, characters: &Val) -> Val {
    let removed = characters.text();
    Val::from(text.text().chars().filter(|character| !removed.contains(*character)).collect::<String>())
}

/// `sscanf` with `%s` and `%d`: scanning stops at the first mismatch and only the values found are returned.
pub fn sscanf(input: &Val, format: &Val) -> Vec<Val> {
    let (input, format) = (input.text(), format.text());
    let (mut text, mut pattern) = (input.chars().peekable(), format.chars());
    let mut found = Vec::new();
    while let Some(expected) = pattern.next() {
        if expected == '%' {
            while text.next_if(|c| c.is_whitespace()).is_some() {}
            match pattern.next() {
                Some('s') => {
                    let mut word = String::new();
                    while let Some(c) = text.next_if(|c| !c.is_whitespace()) {
                        word.push(c);
                    }
                    if word.is_empty() {
                        break;
                    }
                    found.push(Val::from(word));
                }
                Some('d') => {
                    let mut digits = String::new();
                    if let Some(sign) = text.next_if(|c| *c == '-' || *c == '+') {
                        digits.push(sign);
                    }
                    while let Some(c) = text.next_if(char::is_ascii_digit) {
                        digits.push(c);
                    }
                    match digits.parse::<i32>() {
                        Ok(number) => found.push(Val::from(number)),
                        Err(_) => break,
                    }
                }
                _ => break,
            }
        } else if expected.is_whitespace() {
            while text.next_if(|c| c.is_whitespace()).is_some() {}
        } else if text.next() != Some(expected) {
            break;
        }
    }
    found
}
