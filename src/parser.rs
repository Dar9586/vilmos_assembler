pub fn escaped(to_escape: char) -> Option<char> {
    match to_escape {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '"' => Some('"'),
        '\\' => Some('\\'),
        '0' => Some('\0'),
        _ => None
    }
}

fn consume_str(buffer: &mut String, mut actual_char: char) -> String {
    let quoted = actual_char == '"';
    let mut final_string = String::new();
    if quoted {
        actual_char = buffer.pop().expect("Invalid string");
    }

    loop {
        if actual_char == '\\' {// escape \
            let to_escape = buffer.pop().expect("Invalid unescape");
            let escaped = escaped(to_escape).expect("Invalid escaped character");
            final_string.push(escaped);
        } else if (quoted && actual_char != '"') || (!quoted && !actual_char.is_whitespace()) { //push other character
            final_string.push(actual_char);
        } else {
            return final_string;
        }
        match buffer.pop() {
            None => { return final_string; }
            Some(ch) => { actual_char = ch; }
        }
    }
}

pub fn parse(str: &str) -> Vec<String> {
    let mut str = str.trim_start().chars().rev().collect::<String>();
    let mut tokens: Vec<String> = Vec::new();
    while let Some(actual_char) = str.pop() {
        match actual_char {
            '#' => return tokens,
            _ => tokens.push(consume_str(&mut str, actual_char))
        }
        str = str.trim_end().parse().unwrap();
    }
    return tokens;
}