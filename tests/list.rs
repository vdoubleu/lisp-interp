#[cfg(test)]
mod list_tests {
    use interp::runner::run_just_prog;

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
}
