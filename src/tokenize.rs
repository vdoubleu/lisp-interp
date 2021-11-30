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
            '(' | ')' | '[' | ']' => {
                if curr_str.len() != 0 {
                    tokens.push(curr_str);
                    curr_str = "".to_string();
                }
                tokens.push(c.to_string());
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

#[test]
fn test_tokenize() {
    assert_eq!(tokenize(&"(+ (f [(+ 1 (g \"yes\" \"there\")) 2 \"word\"]) 2)".to_string()), vec![
        "(".to_string(),
        "seq".to_string(),
        "(".to_string(),
        "+".to_string(),
        "(".to_string(),
        "f".to_string(),
        "[".to_string(),
        "(".to_string(),
        "+".to_string(),
        "1".to_string(),
        "(".to_string(),
        "g".to_string(),
        "\"yes\"".to_string(),
        "\"there\"".to_string(),
        ")".to_string(),
        ")".to_string(),
        "2".to_string(),
        "\"word\"".to_string(),
        "]".to_string(),
        ")".to_string(),
        "2".to_string(),
        ")".to_string(),
        ")".to_string(),
    ]);

    assert_eq!(tokenize(&"(while (< n 10) (seq (print n) (let n (- n 1))))".to_string()), vec![
        "(".to_string(),
        "seq".to_string(),
        "(".to_string(),
        "while".to_string(),
        "(".to_string(),
        "<".to_string(),
        "n".to_string(),
        "10".to_string(),
        ")".to_string(),
        "(".to_string(),
        "seq".to_string(),
        "(".to_string(),
        "print".to_string(),
        "n".to_string(),
        ")".to_string(),
        "(".to_string(),
        "let".to_string(),
        "n".to_string(),
        "(".to_string(),
        "-".to_string(),
        "n".to_string(),
        "1".to_string(),
        ")".to_string(),
        ")".to_string(),
        ")".to_string(),
        ")".to_string(),
        ")".to_string(),
    ]);
}