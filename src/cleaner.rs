// removes troublesome characters from the input
pub fn clean(prog: &String) -> String {
    return prog.trim().replace("\n", " ").replace("\t", " ");
}

#[test]
fn test_clean() {
    assert_eq!(clean(&"  text \t hey \n  here \n  \n  ".to_string()), "text   hey    here");
}