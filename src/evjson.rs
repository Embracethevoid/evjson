use std::collections::HashMap;
// use std::ops::{Index, IndexMut};
#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
#[derive(Debug, PartialEq)]
pub enum EVValue {
    Object(EVObject),
    Array(Vec<EVValue>),
    Str(String),
    Number(Number),
    Boolean(bool),
    Null,
}

// pub struct EVObject {
//     data:HashMap
// }
pub type EVObject = HashMap<String, Box<EVValue>>;

pub fn new() -> EVObject {
    let data: HashMap<String, Box<EVValue>> = HashMap::new();
    data
}

pub fn stringify(object: &EVObject, space: u8) -> String {
    stringify_object(object, space, 0)
}
fn stringify_object(object: &EVObject, space: u8, current_space: u8) -> String {
    let spaces: String = if space == 0 {
        String::from(" ".repeat(current_space as usize))
    } else {
        format!("\n{}", " ".repeat((space + current_space) as usize))
    };
    let spaces_end: String = if space == 0 {
        String::from("")
    } else {
        format!("\n{}", " ".repeat(current_space as usize))
    };
    let space_between_items: String = if space == 0 {
        String::from(",")
    } else {
        format!(",\n{}", " ".repeat((current_space + space) as usize))
    };
    let mut pairs: Vec<String> = Vec::new();
    for (key, value) in object {
        pairs.push(format!(
            "\"{}\" : {}",
            key,
            stringify_value(value, space, current_space + space)
        ));
    }
    return format!(
        "{{{}{}{}}}",
        spaces,
        pairs.join(&space_between_items),
        spaces_end
    );
}
pub fn stringify_value(value: &EVValue, space: u8, current_space: u8) -> String {
    let spaces: String = if space == 0 {
        String::from(" ".repeat(current_space as usize))
    } else {
        format!("\n{}", " ".repeat((space + current_space) as usize))
    };
    let spaces_end: String = if space == 0 {
        String::from("")
    } else {
        format!("\n{}", " ".repeat(current_space as usize))
    };
    match value {
        EVValue::Str(s) => format!("\"{}\"", s.as_str()),
        EVValue::Number(n) => match n {
            Number::Integer(i) => format!("{}", i),
            Number::Float(f) => format!("{}", f),
        },
        EVValue::Array(a) => {
            let mut tmp = Vec::new();
            for v in a {
                tmp.push(stringify_value(v, space, current_space + space))
            }
            format!(
                "[{}{}{}]",
                spaces,
                tmp.join(&format!(",{}", spaces)),
                spaces_end
            )
        }
        EVValue::Object(o) => stringify_object(o, space, current_space),
        EVValue::Boolean(b) => if *b {
            String::from("true")
        } else {
            String::from("false")
        },
        EVValue::Null => String::from("null"),
    }
}
fn parse_key(index: usize, chars: &Vec<char>) -> Result<(usize, String), String> {
    let mut _index = index;
    loop {
        if _index < chars.len() {
            let c = chars[_index];
            match c {
                ' ' | '\t' | '\n' => (),
                '\"' => {
                    return parse_string(_index + 1, chars);
                }
                _ => {
                    return Err(format!(
                        "Invalid Syntax of charactor {},at position {}.",
                        c, _index
                    ))
                }
            }
        } else {
            return Err("invalid end of JSON".to_string());
        }
        _index += 1;
    }
}
fn parse_array(index: usize, chars: &Vec<char>) -> Result<(usize, EVValue), String> {
    let mut _index = index;
    let mut _list: Vec<EVValue> = Vec::new();
    loop {
        if _index < chars.len() {
            loop {
                if _index < chars.len() {
                    let c = chars[_index];
                    match c {
                        ' ' | '\n' | '\t' => _index += 1,
                        ',' if !_list.is_empty() => break,
                        ']' => return Ok((_index + 1, EVValue::Array(_list))),
                        _ => {
                            let (_ind, _value) = parse_value(_index, chars)?;
                            _index = _ind;
                            _list.push(_value);
                        }
                    }
                } else {
                    return Err(String::from("Invalid End of JSON file"));
                }
            }
        } else {
            return Err(String::from("Invalid End of JSON file"));
        }
        _index += 1;
    }
}
fn parse_pair(index: usize, chars: &Vec<char>) -> Result<(usize, String, EVValue), String> {
    let mut _index = index;
    let (_ind, _key) = parse_key(index, chars)?;
    _index = _ind;
    loop {
        if _index < chars.len() {
            let c = chars[_index];
            match c {
                ' ' | '\t' | '\n' => (),
                ':' => {
                    _index += 1;
                    break;
                }
                _ => {
                    return Err(format!(
                        "Invalid Syntax of charactor {},at position {}.",
                        c, _index
                    ))
                }
            }
        } else {
            return Err("Invalid end of JSON".to_string());
        }
        _index += 1;
    }
    let (_ind, _value) = parse_value(_index, chars)?;
    return Ok((_ind, _key, _value));
}
fn parse_object(index: usize, chars: &Vec<char>) -> Result<(usize, EVObject), String> {
    let mut object = new();
    let mut _index = index;
    loop {
        if _index < chars.len() {
            loop {
                if _index < chars.len() {
                    let c = chars[_index];

                    match c {
                        ' ' | '\t' | '\n' => _index += 1,
                        ',' if !object.is_empty() => {
                            break;
                        }
                        '}' => {
                            return Ok((_index + 1, object));
                        }
                        _ => {
                            let (_ind, _key, _value) = parse_pair(_index, chars)?;
                            object.insert(_key, Box::new(_value));
                            _index = _ind;
                        }
                    }
                } else {
                    return Err("Invalid end".to_string());
                }
            }
        } else {
            return Err(String::from("Invalid JSON Ending"));
        }
        _index += 1;
    }
}
fn parse_number(_index: usize, chars: &Vec<char>) -> Result<(usize, EVValue), String> {
    let mut index = _index;
    let mut dot = false;
    let mut num = String::from("");
    loop {
        if index < chars.len() {
            let c = chars[index];
            match c {
                '0'...'9' | '-' => {
                    num.push(c);
                }
                '.' if !dot => {
                    num.push(c);
                    dot = true;
                }
                _ => {
                    if dot {
                        match num.parse::<f64>() {
                            Ok(_f) => return Ok((index, EVValue::Number(Number::Float(_f)))),
                            _ => return Err("failed to parse number".to_string()),
                        }
                    } else {
                        match num.parse::<i64>() {
                            Ok(_i) => return Ok((index, EVValue::Number(Number::Integer(_i)))),
                            _ => return Err("failed to parse number".to_string()),
                        }
                    }
                }
            }
        } else {
            if dot {
                match num.parse::<f64>() {
                    Ok(_f) => return Ok((index, EVValue::Number(Number::Float(_f)))),
                    _ => return Err("failed to parse number".to_string()),
                }
            } else {
                match num.parse::<i64>() {
                    Ok(_i) => return Ok((index, EVValue::Number(Number::Integer(_i)))),
                    _ => return Err("failed to parse number".to_string()),
                }
            }
        }
        index += 1;
    }
}
fn parse_boolean(index: usize, chars: &Vec<char>) -> Result<(usize, EVValue), String> {
    let mut _index = index;
    let c = chars[_index];
    match c {
        't' => {
            if index + 4 <= chars.len() && chars[index..index + 4] == ['t', 'r', 'u', 'e'] {
                return Ok((index + 4, EVValue::Boolean(true)));
            } else {
                return Err(format!(
                    "Invalid Syntax of charactor {},at position {}.",
                    c, _index
                ));
            }
        }
        'f' => {
            if index + 5 <= chars.len() && chars[index..index + 5] == ['f', 'a', 'l', 's', 'e'] {
                return Ok((index + 5, EVValue::Boolean(false)));
            } else {
                return Err(format!(
                    "Invalid Syntax of charactor {},at position {}.",
                    c, _index
                ));
            }
        }
        _ => {
            return Err(format!(
                "Invalid Syntax of charactor {},at position {}.",
                c, _index
            ));
        }
    }
}
fn parse_null(index: usize, chars: &Vec<char>) -> Result<(usize, EVValue), String> {
    if index + 4 <= chars.len() && chars[index..index + 4] == ['n', 'u', 'l', 'l'] {
        return Ok((index + 4, EVValue::Null));
    } else {
        return Err(format!(
            "Invalid Syntax of charactor {},at position {}.",
            chars[index], index
        ));
    }
}
fn parse_value(index: usize, chars: &Vec<char>) -> Result<(usize, EVValue), String> {
    let mut _index: usize = index;
    loop {
        if _index < chars.len() {
            let c = chars[_index];
            match c {
                ' ' | '\t' | '\n' => (),
                '\"' => {
                    let (_ind, _str) = parse_string(_index + 1, chars)?;
                    return Ok((_ind, EVValue::Str(_str)));
                }
                '[' => return parse_array(_index + 1, chars), //parseArray(chars),
                '{' => {
                    let (_ind, _object) = parse_object(_index + 1, chars)?;
                    return Ok((_ind, EVValue::Object(_object)));
                } //parse_object
                't' | 'f' => return parse_boolean(_index, chars),
                'n' => return parse_null(_index, chars), //parse bool and null
                '0'...'9' | '-' => {
                    let (_ind, _num) = parse_number(_index, chars)?;
                    return Ok((_ind, _num));
                }
                _ => {
                    return Err(format!(
                        "Invalid Syntax of charactor {},at position {}.",
                        c, _index
                    ));
                }
            }
        } else {
            return Err("".to_string());
        }
        _index += 1;
    }
}
fn parse_string(index: usize, chars: &Vec<char>) -> Result<(usize, String), String> {
    let mut s = String::from("");
    let mut _index = index;
    loop {
        if _index < chars.len() {
            let c = chars[_index];
            match c {
                '\"' => return Ok((_index + 1, s)),
                _ => s.push(c),
            }
        } else {
            break;
        }
        _index += 1;
    }
    return Err("Invalid JSON end".to_string());
}
pub fn parse(_input: String) -> Result<EVValue, String> {
    let chars: Vec<char> = _input.chars().collect();
    let mut result: Option<EVValue> = None;
    let mut _index: usize = 0;
    loop {
        if _index < chars.len() {
            let (_ind, _value) = parse_value(_index, &chars)?;
            if _ind >= chars.len() {
                return Ok(_value);
            }
            result = Some(_value);
            _index = _ind;
            let c = chars[_index];
            match c {
                ' ' | '\t' | '\n' => _index += 1,
                _ => {
                    return Err(format!(
                        "Invalid Syntax of charactor {},at position {}.",
                        c, _index
                    ))
                }
            }
        } else {
            match result {
                Some(_v) => return Ok(_v),
                _ => return Err("failed to parse".to_string()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /* parse string */
    #[test]
    fn it_parses_string_from_begining() {
        assert_eq!(
            Ok((5 as usize, "fire".to_string())),
            parse_string(0, &vec!['f', 'i', 'r', 'e', '\"', '\"'])
        )
    }

    #[test]
    fn it_returns_err_if_string_does_not_end() {
        assert_eq!(
            Err("Invalid JSON end".to_string()),
            parse_string(0, &vec!['f', 'i', 'r', 'e'])
        )
    }

    /* parse number */
    #[test]
    fn it_parses_a_float() {
        assert_eq!(
            Ok((3 as usize, EVValue::Number(Number::Float(1.2)))),
            parse_number(0, &vec!['1', '.', '2', ' '])
        )
    }

    #[test]
    fn it_parses_a_integer() {
        assert_eq!(
            Ok((2 as usize, EVValue::Number(Number::Integer(12)))),
            parse_number(0, &vec!['1', '2', ' '])
        )
    }

    #[test]
    fn it_parses_a_negative_float() {
        assert_eq!(
            Ok((4 as usize, EVValue::Number(Number::Float(-1.2)))),
            parse_number(0, &vec!['-', '1', '.', '2', ']'])
        )
    }

    #[test]
    fn it_parses_a_negative_integer() {
        assert_eq!(
            Ok((3 as usize, EVValue::Number(Number::Integer(-12)))),
            parse_number(0, &vec!['-', '1', '2', ']'])
        )
    }

    #[test]
    fn it_returns_an_err_if_only_have_negative_symbol() {
        assert_eq!(
            Err("failed to parse number".to_string()),
            parse_number(0, &vec!['-', ']'])
        )
    }

    /* parse boolean */
    #[test]
    fn it_parses_a_true() {
        assert_eq!(
            Ok((6 as usize, EVValue::Boolean(true))),
            parse_boolean(2, &vec![' ', ':', 't', 'r', 'u', 'e'])
        )
    }

    #[test]
    fn it_parses_a_false() {
        assert_eq!(
            Ok((7 as usize, EVValue::Boolean(false))),
            parse_boolean(2, &vec![' ', ':', 'f', 'a', 'l', 's', 'e'])
        )
    }

    #[test]
    fn it_returns_error_when_parsing_part_of_true() {
        assert_eq!(
            Err(format!(
                "Invalid Syntax of charactor {},at position {}.",
                't', 2
            )),
            parse_boolean(2, &vec![' ', ':', 't', 'r', 'u'])
        )
    }

    #[test]
    fn it_returns_error_when_parsing_part_of_false() {
        assert_eq!(
            Err(format!(
                "Invalid Syntax of charactor {},at position {}.",
                'f', 2
            )),
            parse_boolean(2, &vec![' ', ':', 'f', 'r', 'u'])
        )
    }

    /* parsing null */
    #[test]
    fn it_parses_a_null() {
        assert_eq!(
            Ok((6 as usize, EVValue::Null)),
            parse_null(2, &vec![' ', ':', 'n', 'u', 'l', 'l'])
        )
    }

    #[test]
    fn it_returns_error_when_parsing_part_of_null() {
        assert_eq!(
            Err(format!(
                "Invalid Syntax of charactor {},at position {}.",
                'n', 2
            )),
            parse_null(2, &vec![' ', ':', 'n'])
        )
    }

    /* parsing array */
    #[test]
    fn it_parses_an_empty_array() {
        assert_eq!(
            Ok((6 as usize, EVValue::Array(vec![]))),
            parse_array(2, &vec![' ', ' ', ' ', ' ', ' ', ']'])
        )
    }

    #[test]
    fn it_parses_an_array_with_a_string() {
        assert_eq!(
            Ok((
                6 as usize,
                EVValue::Array(vec![EVValue::Str("a".to_string())])
            )),
            parse_array(0, &vec![' ', '\"', 'a', '\"', ' ', ']'])
        )
    }

    #[test]
    fn it_parses_an_array_with_two_strings() {
        assert_eq!(
            Ok((
                11 as usize,
                EVValue::Array(vec![
                    EVValue::Str("a".to_string()),
                    EVValue::Str("a".to_string())
                ])
            )),
            parse_array(
                0,
                &vec![' ', '\"', 'a', '\"', ',', ' ', '\"', 'a', '\"', ',', ']']
            )
        )
    }
    /* parse key */
    #[test]
    fn it_parses_a_key() {
        assert_eq!(
            Ok((4 as usize, "a".to_string())),
            parse_key(0, &vec![' ', '\"', 'a', '\"'])
        )
    }

    #[test]
    fn it_returns_error_for_part_of_key() {
        assert_eq!(
            Err("Invalid JSON end".to_string()),
            parse_key(0, &vec![' ', '\"'])
        )
    }

    #[test]
    fn it_returns_error_for_invalid_charactor() {
        assert_eq!(
            Err("Invalid Syntax of charactor a,at position 1.".to_string()),
            parse_key(0, &vec![' ', 'a'])
        )
    }
}
