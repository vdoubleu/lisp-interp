pub fn tokenize(s: &String) -> Vec<String> {
    let mut curr_str: String = String::new();
    let mut tokens: Vec<String> = Vec::new();
    for c in s.chars() {
        match c {
            '(' | ')' => {
                if curr_str.len() != 0 {
                    tokens.push(curr_str);
                    curr_str = "".to_string();
                }
                tokens.push(
                    if c == '(' {
                        "(".to_string()
                    } else {
                        ")".to_string()
                    }
                );
            }
            ' ' | '\n' => {
                if curr_str.len() != 0 {
                    tokens.push(curr_str);
                    curr_str = "".to_string();
                }
            }
            other => {
                curr_str.push(other);
            }
        }
    }

    return tokens;
}

