#[cfg(test)]
mod list_tests {
    use fl::runner::run_just_prog;

    #[test]
    fn list_rev() {
        let prog = r#"
        (seq
            (let (reverse full-lst)
                (seq
                (let (rev-help lst acc)
                    (if (== 0 (len lst))
                    acc
                    (rev-help (rest lst) (cons (first lst) acc))))
                (rev-help full-lst [])))
            (reverse [1 2 3 4 5])
        )
    "#;
        let output = run_just_prog(&prog.to_string()).to_string();
        assert_eq!(output, "[5 4 3 2 1]");
    }

    #[test]
    fn import_list() {
        let prog = r#"(import "list") (let l (reverse [1 2 3])) (empty? l) (append l 1) (second l) (third l) (ind l 1)"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "2");
    }

    #[test]
    fn list_rev2() {
        let prog = r#"(reverse [1 2 3])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "[3 2 1]");

        let prog2 = r#"(reverse [])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "[]");
    }

    #[test]
    fn empty_list() {
        let prog = r#"(empty? [1 2 3])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "false");

        let prog2 = r#"(empty? [])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "true");
    }

    #[test]
    fn append() {
        let prog = r#"(append [1 2 3] 4)"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "[1 2 3 4]");

        let prog2 = r#"(append [] 1)"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "[1]");
    }

    #[test]
    fn ind() {
        let prog = r#"(ind [1 2 3] 0)"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "1");

        let prog2 = r#"(ind [1 2] 1)"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "2");

        let prog3 = r#"(ind [] 1)"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "[]");
    }

    #[test]
    fn second() {
        let prog = r#"(second [1 2 3])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "2");

        let prog2 = r#"(second [1 2])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "2");

        let prog3 = r#"(second [1])"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "[]");

        let prog4 = r#"(second [])"#;
        assert_eq!(run_just_prog(&prog4.to_string()).to_string(), "[]");
    }

    #[test]
    fn third() {
        let prog = r#"(third [1 2 3])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "3");

        let prog2 = r#"(third [1 2])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "[]");

        let prog3 = r#"(third [1])"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "[]");

        let prog4 = r#"(third [])"#;
        assert_eq!(run_just_prog(&prog4.to_string()).to_string(), "[]");
    }

    #[test]
    fn last() {
        let prog = r#"(last [1 2 3])"#;
        assert_eq!(run_just_prog(&prog.to_string()).to_string(), "3");

        let prog2 = r#"(last [1 2])"#;
        assert_eq!(run_just_prog(&prog2.to_string()).to_string(), "2");

        let prog3 = r#"(last [1])"#;
        assert_eq!(run_just_prog(&prog3.to_string()).to_string(), "1");

        let prog4 = r#"(last [])"#;
        assert_eq!(run_just_prog(&prog4.to_string()).to_string(), "[]");
    }
}
