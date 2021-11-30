#[cfg(test)]
mod import_tests {
    use interp::runner::run_just_prog;

    #[test]
    fn test_import() {
        let r1 = run_just_prog(&"(seq (import \"./tests/test-import.fl\") (testfunc testvar))".to_string()).to_string();
        let r2 = "4";
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_stdlib() {
        let r1 = run_just_prog(&"(seq (import \"list\") (reverse [1 2 3 4 5]))".to_string()).to_string();
        let r2 = "[5 4 3 2 1]";
        assert_eq!(r1, r2);
    }
}
