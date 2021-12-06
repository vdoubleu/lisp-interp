pub fn check_prog(input: &String) -> Result<bool, &str> {
    return bracket_checker(&input);
}

fn bracket_checker(input: &String) -> Result<bool, &str> {
    let mut stack = Vec::new();
    for c in input.chars() {
        println!("{:?}   {}", stack, stack.len());
        match c {
            '(' | '[' => stack.push(c),
            ')' | ']' => {
                let stack_res = stack.pop();
                if stack_res.is_none() {
                    return Err("Bracket mismatch");
                }
                
                if c == ')' && stack_res.unwrap() == '(' {
                    continue;
                } else if c == ']' && stack_res.unwrap() == '[' {
                    continue;
                } else {
                    return Err("Bracket mismatch");
                }
            }
            _ => {}
        }
    }
        println!("{:?}   {}", stack, stack.len());
    if stack.len() == 0 {
        Ok(true)
    } else {
        Err("Bracket mismatch, unclosed brackets")
    }
}

#[test]
fn test_bracket_checker() {
    assert_eq!(bracket_checker(&String::from("()")), Ok(true));
    assert_eq!(bracket_checker(&String::from("()[]")), Ok(true));
    assert_eq!(bracket_checker(&String::from("(]")), Err("Bracket mismatch"));
    assert_eq!(bracket_checker(&String::from("([)]")), Err("Bracket mismatch"));
    assert_eq!(bracket_checker(&String::from("([])")), Ok(true));
    assert_eq!(bracket_checker(&String::from("([)")), Err("Bracket mismatch"));
    assert_eq!(bracket_checker(&String::from("([]")), Err("Bracket mismatch, unclosed brackets"));
    assert_eq!(bracket_checker(&String::from("(s;  dkfsd[   sdlfkjs  dfl ] sldk fjsd")), Err("Bracket mismatch, unclosed brackets"));
}
