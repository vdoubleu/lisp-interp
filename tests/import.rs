#[cfg(test)]
mod import_tests {
    use fl::runner::run_just_prog;

    #[test]
    fn test_import() {
        let r1 = run_just_prog(&"(seq (import \"./tests/test-import.fl\") (testfunc testvar))".to_string()).to_string();
        let r2 = "4";
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_stdlib() {
        let r1 = run_just_prog(&"(seq (import \"list\") (import \"string\") (concat-string-list (reverse (list \"hello\"))))".to_string()).to_string();
        let r2 = "olleh";
        assert_eq!(r1, r2);
    }

    #[test]
    fn multi_import() {
        let r1 = run_just_prog(&"(seq (import \"list\" \"string\") (concat-string-list (reverse (list \"hello\"))))".to_string()).to_string();
        let r2 = "olleh";
        assert_eq!(r1, r2);
    }
}
