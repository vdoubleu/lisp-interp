#[cfg(test)]
mod syntax_check_tests {
    use fl::runner::run_just_prog;

    #[test]
    #[should_panic]
    fn syntax_1() {
        let r1 = run_just_prog(&String::from("(()")).to_string();
        let r2 = "";
        assert_eq!(r1, r2);
    }

    #[test]
    #[should_panic]
    fn syntax_2() {
        let r1 = run_just_prog(&String::from("())")).to_string();
        let r2 = "";
        assert_eq!(r1, r2);
    }

    #[test]
    #[should_panic]
    fn syntax_3() {
        let r1 = run_just_prog(&String::from("([)]")).to_string();
        let r2 = "";
        assert_eq!(r1, r2);
    }

    #[test]
    #[should_panic]
    fn syntax_4() {
        let r1 = run_just_prog(&String::from("[(])")).to_string();
        let r2 = "";
        assert_eq!(r1, r2);
    }

    #[test]
    #[should_panic]
    fn syntax_5() {
        let r1 = run_just_prog(&String::from("[()])")).to_string();
        let r2 = "";
        assert_eq!(r1, r2);
    }
}
