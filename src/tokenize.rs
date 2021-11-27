pub fn tokenize(s: &String) -> Vec<String> {
    let mut curr_str: String = String::new();
    let mut tokens: Vec<String> = vec!["(".to_string(), "seq".to_string()];
    let mut in_string: bool = false;
    for c in s.chars() {
        if in_string && c != '"' {
            curr_str.push(c);
            continue;
        }

        match c {
            '"' => {
                if in_string {
                    curr_str.push('"');
                    tokens.push(curr_str);
                    curr_str = "".to_string();
                } else {
                    if curr_str.len() != 0 {
                        tokens.push("\"".to_owned() + &curr_str);
                    }
                    curr_str = '"'.to_string();
                }

                in_string = !in_string;
            }
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

    tokens.push(")".to_string());

    return tokens;
}

