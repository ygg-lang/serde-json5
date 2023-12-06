use super::*;

pub(super) fn parse_cst(input: &str, rule: Json5Token) -> OutputResult<Json5Token> {
    state(input, |state| match rule {
        Json5Token::Value => parse_value(state),
        Json5Token::Object => parse_object(state),
        Json5Token::ObjectPair => parse_object_pair(state),
        Json5Token::ObjectKey => parse_object_key(state),
        Json5Token::Array => parse_array(state),
        Json5Token::String => parse_string(state),
        Json5Token::DoubleStringElement => parse_double_string_element(state),
        Json5Token::SingleStringElement => parse_single_string_element(state),
        Json5Token::HexEscape => parse_hex_escape(state),
        Json5Token::AnyEscape => parse_any_escape(state),
        Json5Token::DoubleStringText => parse_double_string_text(state),
        Json5Token::SingleStringText => parse_single_string_text(state),
        Json5Token::Number => parse_number(state),
        Json5Token::Boolean => parse_boolean(state),
        Json5Token::Null => parse_null(state),
        Json5Token::Identifier => parse_identifier(state),
        Json5Token::Colon => parse_colon(state),
        Json5Token::Comma => parse_comma(state),
        Json5Token::Comment => parse_comment(state),
        Json5Token::WhiteSpace => parse_white_space(state),
        Json5Token::String0 => parse_string_0(state),
        Json5Token::String1 => parse_string_1(state),
        Json5Token::Boolean0 => parse_boolean_0(state),
        Json5Token::Boolean1 => parse_boolean_1(state),
        Json5Token::HiddenText => unreachable!(),
    })
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5Token::Value, |s| {
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
    state.rule(Json5Token::Object, |s| {
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
    state.rule(Json5Token::ObjectPair, |s| {
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
    state.rule(Json5Token::ObjectKey, |s| {
        Err(s)
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_string(s).and_then(|s| s.tag_node("string")))
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5Token::Array, |s| {
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
    state.rule(Json5Token::String, |s| {
        Err(s)
            .or_else(|s| parse_string_0(s).and_then(|s| s.tag_node("string_0")))
            .or_else(|s| parse_string_1(s).and_then(|s| s.tag_node("string_1")))
    })
}
#[inline]
fn parse_double_string_element(state: Input) -> Output {
    state.rule(Json5Token::DoubleStringElement, |s| {
        Err(s)
            .or_else(|s| parse_hex_escape(s).and_then(|s| s.tag_node("hex_escape")))
            .or_else(|s| parse_any_escape(s).and_then(|s| s.tag_node("any_escape")))
            .or_else(|s| parse_double_string_text(s).and_then(|s| s.tag_node("double_string_text")))
    })
}
#[inline]
fn parse_single_string_element(state: Input) -> Output {
    state.rule(Json5Token::SingleStringElement, |s| {
        Err(s)
            .or_else(|s| parse_hex_escape(s).and_then(|s| s.tag_node("hex_escape")))
            .or_else(|s| parse_any_escape(s).and_then(|s| s.tag_node("any_escape")))
            .or_else(|s| parse_single_string_text(s).and_then(|s| s.tag_node("single_string_text")))
    })
}
#[inline]
fn parse_hex_escape(state: Input) -> Output {
    state.rule(Json5Token::HexEscape, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\\\u[0-9a-fA-F]{4})").unwrap())
        })
    })
}
#[inline]
fn parse_any_escape(state: Input) -> Output {
    state.rule(Json5Token::AnyEscape, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)(\\\\.)").unwrap())
        })
    })
}
#[inline]
fn parse_double_string_text(state: Input) -> Output {
    state.rule(Json5Token::DoubleStringText, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^\"\\\\]+)").unwrap())
        })
    })
}
#[inline]
fn parse_single_string_text(state: Input) -> Output {
    state.rule(Json5Token::SingleStringText, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([^'\\\\]+)").unwrap())
        })
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5Token::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([+-]?(0|[1-9][0-9]*))").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5Token::Boolean, |s| {
        Err(s)
            .or_else(|s| parse_boolean_0(s).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| parse_boolean_1(s).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5Token::Null, |s| s.match_string("null", false))
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(Json5Token::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(?x)([_\\p{XID_start}][\\p{XID_continue}]*)").unwrap())
        })
    })
}
#[inline]
fn parse_colon(state: Input) -> Output {
    state.rule(Json5Token::Colon, |s| s.match_string(":", false))
}
#[inline]
fn parse_comma(state: Input) -> Output {
    state.rule(Json5Token::Comma, |s| s.match_string(",", false))
}
#[inline]
fn parse_comment(state: Input) -> Output {
    state.rule(Json5Token::Comment, |s| {
        s.sequence(|s| Ok(s).and_then(|s| builtin_text(s, "#", false)).and_then(|s| s.rest_of_line()))
    })
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5Token::WhiteSpace, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, " ", false))
            .or_else(|s| builtin_text(s, "\\n", false))
            .or_else(|s| builtin_text(s, "\\r", false))
    })
}
#[inline]
fn parse_string_0(state: Input) -> Output {
    state.rule(Json5Token::String0, |s| {
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
    state.rule(Json5Token::String1, |s| {
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
    state.rule(Json5Token::Boolean0, |s| s.match_string("true", false))
}
#[inline]
fn parse_boolean_1(state: Input) -> Output {
    state.rule(Json5Token::Boolean1, |s| s.match_string("false", false))
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_comment(s).or_else(|s| parse_white_space(s)))
}

fn builtin_any(state: Input) -> Output {
    state.rule(Json5Token::HiddenText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(Json5Token::HiddenText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Json5Token::HiddenText, |s| s.match_regex(regex))
}
