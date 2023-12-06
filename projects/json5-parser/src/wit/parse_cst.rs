use super::*;

pub(super) fn parse_cst(input: &str, rule: Json5) -> OutputResult<Json5> {
    state(input, |state| match rule {
        Json5::Value => parse_value(state),
        Json5::Object => parse_object(state),
        Json5::ObjectPair => parse_object_pair(state),
        Json5::ObjectKey => parse_object_key(state),
        Json5::Array => parse_array(state),
        Json5::String => parse_string(state),
        Json5::DoubleStringElement => parse_double_string_element(state),
        Json5::SingleStringElement => parse_single_string_element(state),
        Json5::HexEscape => parse_hex_escape(state),
        Json5::AnyEscape => parse_any_escape(state),
        Json5::DoubleStringText => parse_double_string_text(state),
        Json5::SingleStringText => parse_single_string_text(state),
        Json5::Number => parse_number(state),
        Json5::Boolean => parse_boolean(state),
        Json5::Null => parse_null(state),
        Json5::Identifier => parse_identifier(state),
        Json5::COLON => parse_colon(state),
        Json5::COMMA => parse_comma(state),
        Json5::Comment => parse_comment(state),
        Json5::WhiteSpace => parse_white_space(state),
        Json5::String0 => parse_string_0(state),
        Json5::String1 => parse_string_1(state),
        Json5::Boolean0 => parse_boolean_0(state),
        Json5::Boolean1 => parse_boolean_1(state),
        Json5::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5::Value, |s| {
        Err(s)
            .or_else(|s| parse_object(s).and_then(|s| s.tag_node("object")))
            .or_else(|s| parse_array(s).and_then(|s| s.tag_node("array")))
            .or_else(|s| parse_string(s).and_then(|s| s.tag_node("string")))
            .or_else(|s| parse_number(s).and_then(|s| s.tag_node("number")))
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
            .or_else(|s| parse_null(s).and_then(|s| s.tag_node("null")))
    })
}
#[inline]
fn parse_object(state: Input) -> Output {
    state.rule(Json5::Object, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_object_pair(s).and_then(|s| s.tag_node("object_pair")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| {
                                                            parse_object_pair(s).and_then(|s| s.tag_node("object_pair"))
                                                        })
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_object_pair(state: Input) -> Output {
    state.rule(Json5::ObjectPair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_object_key(s).and_then(|s| s.tag_node("object_key")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_colon(s).and_then(|s| s.tag_node("colon")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
        })
    })
}
#[inline]
fn parse_object_key(state: Input) -> Output {
    state.rule(Json5::ObjectKey, |s| {
        Err(s)
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_string(s).and_then(|s| s.tag_node("string")))
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5::Array, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "[", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s).and_then(|s| builtin_ignore(s)).and_then(|s| {
                                                s.sequence(|s| {
                                                    Ok(s)
                                                        .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                                                        .and_then(|s| builtin_ignore(s))
                                                        .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
                                                })
                                            })
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| parse_comma(s).and_then(|s| s.tag_node("comma")))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "]", false))
        })
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Json5::String, |s| {
        Err(s)
            .or_else(|s| parse_string_0(s).and_then(|s| s.tag_node("string_0")))
            .or_else(|s| parse_string_1(s).and_then(|s| s.tag_node("string_1")))
    })
}
#[inline]
fn parse_double_string_element(state: Input) -> Output {
    state.rule(Json5::DoubleStringElement, |s| {
        Err(s)
            .or_else(|s| parse_hex_escape(s).and_then(|s| s.tag_node("hex_escape")))
            .or_else(|s| parse_any_escape(s).and_then(|s| s.tag_node("any_escape")))
            .or_else(|s| parse_double_string_text(s).and_then(|s| s.tag_node("double_string_text")))
    })
}
#[inline]
fn parse_single_string_element(state: Input) -> Output {
    state.rule(Json5::SingleStringElement, |s| {
        Err(s)
            .or_else(|s| parse_hex_escape(s).and_then(|s| s.tag_node("hex_escape")))
            .or_else(|s| parse_any_escape(s).and_then(|s| s.tag_node("any_escape")))
            .or_else(|s| parse_single_string_text(s).and_then(|s| s.tag_node("single_string_text")))
    })
}
#[inline]
fn parse_hex_escape(state: Input) -> Output {
    state.rule(Json5::HexEscape, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\\\u[0-9a-fA-F]{4})").unwrap())
        })
    })
}
#[inline]
fn parse_any_escape(state: Input) -> Output {
    state.rule(Json5::AnyEscape, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\\\.)").unwrap())
        })
    })
}
#[inline]
fn parse_double_string_text(state: Input) -> Output {
    state.rule(Json5::DoubleStringText, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^\"\\\\]+)").unwrap())
        })
    })
}
#[inline]
fn parse_single_string_text(state: Input) -> Output {
    state.rule(Json5::SingleStringText, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^'\\\\]+)").unwrap())
        })
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([+-]?(0|[1-9][0-9]*))").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5::Boolean, |s| {
        Err(s)
            .or_else(|s| parse_boolean_0(s).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| parse_boolean_1(s).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5::Null, |s| s.match_string("null", false))
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(Json5::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([_\\p{XID_start}][\\p{XID_continue}]*)").unwrap())
        })
    })
}
#[inline]
fn parse_colon(state: Input) -> Output {
    state.rule(Json5::COLON, |s| s.match_string(":", false))
}
#[inline]
fn parse_comma(state: Input) -> Output {
    state.rule(Json5::COMMA, |s| s.match_string(",", false))
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(Json5::Comment, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "#", false)).and_then(|s| s.rest_of_line()))
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5::WhiteSpace, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, " ", false))
            .or_else(|s| builtin_text(s, "\\n", false))
            .or_else(|s| builtin_text(s, "\\r", false))
    })
}
#[inline]
fn parse_string_0(state: Input) -> Output {
    state.rule(Json5::String0, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "\"", false))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_double_string_element(s)))
                .and_then(|s| builtin_text(s, "\"", false))
        })
    })
}
#[inline]
fn parse_string_1(state: Input) -> Output {
    state.rule(Json5::String1, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "'", false))
                .and_then(|s| s.repeat(0..4294967295, |s| parse_single_string_element(s)))
                .and_then(|s| builtin_text(s, "'", false))
        })
    })
}
#[inline]
fn parse_boolean_0(state: Input) -> Output {
    state.rule(Json5::Boolean0, |s| s.match_string("true", false))
}
#[inline]
fn parse_boolean_1(state: Input) -> Output {
    state.rule(Json5::Boolean1, |s| s.match_string("false", false))
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_comment(s).or_else(|s| parse_white_space(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(Json5::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(Json5::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Json5::HiddenText, |s| s.match_regex(regex))
}
