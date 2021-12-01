#[cfg(test)]
mod string_tests {
    use fl::runner::run_just_prog;

    #[test]
    fn concat_string_list() {
        let prog = r#"(concat-string-list ["1" "2" "3"])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "123");

        let prog2 = r#"(concat-string-list [])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "");
    }

    #[test]
    fn substr_start() {
        let prog = r#"(substr-start "ab" 0)"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "ab");

        let prog2 = r#"(substr-start "ab" 1)"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "b");

        let prog3 = r#"(substr-start "ab" 2)"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "");

        let prog4 = r#"(substr-start "ab" 3)"#;
        assert_eq!(run_just_prog(&prog4.to_string()).to_string(), "");
    }

    #[test]
    fn substr_end() {
        let prog = r#"(substr-end "ab" 0)"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "");

        let prog2 = r#"(substr-end "ab" 1)"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "a");

        let prog3 = r#"(substr-end "ab" 2)"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "ab");

        let prog4 = r#"(substr-end "ab" 3)"#;
        assert_eq!(run_just_prog(&prog4.to_string()).to_string(), "ab");
    }

    
}
