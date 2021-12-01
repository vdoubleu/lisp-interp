// removes troublesome characters from the input
pub fn clean(prog: &String) -> String {
    return prog.trim().replace("\n", " ").replace("\t", " ");
}

#[test]
fn test_clean() {
    assert_eq!(clean(&"  text \t hey \n  here \n  \n  ".to_string()), "text   hey    here");
}

pub fn remove_comment_in_line(line: &String) -> String {
    let mut line = line.clone();
    let comment_start = line.find(";");
    if comment_start.is_some() {
        line = line[..comment_start.unwrap()].to_string();
    }
    return line;
}

#[test]
fn test_remove_comment_in_line(){
    assert_eq!(remove_comment_in_line(&"  text ; hey \n  here \n  \n  ".to_string()), "  text ");
    assert_eq!(remove_comment_in_line(&"  text  hey \n  here \n  \n  ".to_string()), "  text  hey \n  here \n  \n  ");
}