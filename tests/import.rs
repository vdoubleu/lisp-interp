use interp::runner::run_prog;

#[test]
fn test_import() {
    let r1 = run_prog(&"(seq (import \"./tests/test-import.ls\") (testfunc testvar))".to_string()).to_string();
    let r2 = "4";
    assert_eq!(r1, r2);
}