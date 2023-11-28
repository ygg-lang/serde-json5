use super::*;

pub(super) fn parse_cst(input: &str, rule: Json5Rule) -> OutputResult<Json5Rule> {
    state(input, |state| match rule {
        Json5Rule::Value => parse_value(state),
        Json5Rule::Object => parse_object(state),
        Json5Rule::ObjectPair => parse_object_pair(state),
        Json5Rule::ObjectKey => parse_object_key(state),
        Json5Rule::Array => parse_array(state),
        Json5Rule::String => parse_string(state),
        Json5Rule::StringEscaped => parse_string_escaped(state),
        Json5Rule::Number => parse_number(state),
        Json5Rule::Boolean => parse_boolean(state),
        Json5Rule::Null => parse_null(state),
        Json5Rule::Identifier => parse_identifier(state),
        Json5Rule::WhiteSpace => parse_white_space(state),
        Json5Rule::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5Rule::Value, |s| {
        Err(s).or_else(|s|parse_object(s).and_then(|s| s.tag_node("object"))).or_else(|s|parse_array(s).and_then(|s| s.tag_node("array"))).or_else(|s|parse_string(s).and_then(|s| s.tag_node("string"))).or_else(|s|parse_number(s).and_then(|s| s.tag_node("number"))).or_else(|s|parse_boolean(s).and_then(|s| s.tag_node("boolean"))).or_else(|s|parse_null(s).and_then(|s| s.tag_node("null")))
    })
}
#[inline]
fn parse_object(state: Input) -> Output {
    state.rule(Json5Rule::Object, |s| {
        s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,"{",false)).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|s.sequence(|s|Ok(s).and_then(|s|parse_object_pair(s).and_then(|s| s.tag_node("object_pair"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.repeat(0..4294967295,|s|s.sequence(|s|Ok(s).and_then(|s|builtin_ignore(s)).and_then(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,",",false)).and_then(|s|builtin_ignore(s)).and_then(|s|parse_object_pair(s).and_then(|s| s.tag_node("object_pair")))))))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s,",",false))))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s,"}",false)))
    })
}
#[inline]
fn parse_object_pair(state: Input) -> Output {
    state.rule(Json5Rule::ObjectPair, |s| {
        s.sequence(|s|Ok(s).and_then(|s|parse_object_key(s).and_then(|s| s.tag_node("object_key"))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s,":",false)).and_then(|s|builtin_ignore(s)).and_then(|s|parse_value(s).and_then(|s| s.tag_node("value"))))
    })
}
#[inline]
fn parse_object_key(state: Input) -> Output {
    state.rule(Json5Rule::ObjectKey, |s| {
        Err(s).or_else(|s|parse_identifier(s).and_then(|s| s.tag_node("identifier"))).or_else(|s|parse_string(s).and_then(|s| s.tag_node("string")))
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5Rule::Array, |s| {
        s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,"[",false)).and_then(|s|builtin_ignore(s)).and_then(|s|s.optional(|s|s.sequence(|s|Ok(s).and_then(|s|parse_value(s).and_then(|s| s.tag_node("value"))).and_then(|s|builtin_ignore(s)).and_then(|s|s.repeat(0..4294967295,|s|s.sequence(|s|Ok(s).and_then(|s|builtin_ignore(s)).and_then(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,",",false)).and_then(|s|builtin_ignore(s)).and_then(|s|parse_value(s).and_then(|s| s.tag_node("value")))))))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s,",",false))))).and_then(|s|builtin_ignore(s)).and_then(|s|builtin_text(s,"]",false)))
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Json5Rule::String, |s| {
        Err(s).or_else(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,"\"",false)).and_then(|s|Err(s).or_else(|s|parse_string_escaped(s).and_then(|s| s.tag_node("string_escaped"))).or_else(|s|builtin_any(s))).and_then(|s|builtin_text(s,"\"",false)))).or_else(|s|s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,"'",false)).and_then(|s|Err(s).or_else(|s|parse_string_escaped(s).and_then(|s| s.tag_node("string_escaped"))).or_else(|s|builtin_any(s))).and_then(|s|builtin_text(s,"'",false))))
    })
}
#[inline]
fn parse_string_escaped(state: Input) -> Output {
    state.rule(Json5Rule::StringEscaped, |s| {
        s.sequence(|s|Ok(s).and_then(|s|builtin_text(s,"\\",false)).and_then(|s|builtin_any(s)))
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5Rule::Number, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new("^(?x)([+-]?(0|[1-9][0-9]*))").unwrap())})
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5Rule::Boolean, |s| {
        Err(s).or_else(|s|builtin_text(s,"true",false).and_then(|s| s.tag_node("true"))).or_else(|s|builtin_text(s,"false",false).and_then(|s| s.tag_node("false")))
    })
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5Rule::Null, |s| {
        s.match_string("null", false)
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(Json5Rule::Identifier, |s| {
        s.match_regex({static REGEX:OnceLock<Regex>=OnceLock::new();REGEX.get_or_init(||Regex::new("^(?x)([_\\p{XID_start}][\\p{XID_continue}]*)").unwrap())})
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5Rule::WhiteSpace, |s| {
        Err(s).or_else(|s|builtin_text(s," ",false)).or_else(|s|builtin_text(s,"\\n",false)).or_else(|s|builtin_text(s,"\\r",false))
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| {
        parse_white_space(s)
    })

}

fn builtin_any(state: Input) -> Output {
    state.rule(Json5Rule::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(Json5Rule::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Json5Rule::HiddenText, |s| s.match_regex(regex))
}