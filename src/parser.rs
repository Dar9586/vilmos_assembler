pub fn escaped(to_escape: Option<char>) -> Option<char> {
    if to_escape.is_none() {
        return None;
    }
    match to_escape.unwrap() {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '"' => Some('"'),
        '\\' => Some('\\'),
        '0' => Some('\0'),
        _ => None
    }
}

fn consume_str(buffer: &mut String, mut actual_char: char) -> Option<String> {
    let quoted = actual_char == '"';
    let mut final_string = String::new();
    if quoted {
        match buffer.pop() {
            None => { return None; }
            Some(ch) => { actual_char = ch; }
        }
    }

    loop {
        if actual_char == '\\' {// escape \
            let to_escape = buffer.pop();
            let escaped = escaped(to_escape);
            if escaped.is_none() {
                return None
            }
            final_string.push(escaped.unwrap());
        } else if (quoted && actual_char != '"') || (!quoted && !actual_char.is_whitespace()) { //push other character
            final_string.push(actual_char);
        } else {
            return Some(final_string);
        }
        match buffer.pop() {
            None => { return Some(final_string); }
            Some(ch) => { actual_char = ch; }
        }
    }
}

pub fn parse(str: &str) -> Option<Vec<String>> {
    let mut str = str.trim_start().chars().rev().collect::<String>();
    let mut tokens: Vec<String> = Vec::new();
    while let Some(actual_char) = str.pop() {
        match actual_char {
            '#' => break,
            _ => match consume_str(&mut str, actual_char) {
                None => return None,
                Some(str) => tokens.push(str)
            }
        }
        str = str.trim_end().parse().unwrap();
    }
    return Some(tokens);
}