#[cfg(test)]
mod fib_tests {
    use fl::runner::run_just_prog;

    #[test]
    fn fib_rec() {
        let prog = r#"
    (seq
        (let (fib n)
            (if (== n 0)
            1
            (if (== n 1)
                1
                (+ (fib (- n 2)) (fib (- n 1))))))

        (print (fib 5))
    )
    "#;
        let output = run_just_prog(&prog.to_string()).to_string();
        assert_eq!(output, "8");
    }

    #[test]
    fn fib_iter() {
        let prog = r#"
        (seq
            (let a 0)
            (let b 1)
            (let n 5)
          
            (while (>= n 0)
              (seq
                (let temp b)
                (let b (+ a b))
                (let a temp)
                (let n (- n 1))
              ))
            (print b)
          )
    "#;
        let output = run_just_prog(&prog.to_string()).to_string();
        assert_eq!(output, "13");
    }
}
