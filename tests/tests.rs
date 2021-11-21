use interp::runner::run_prog;

#[test]
fn run_prog_tests_1() {
    let r1 = run_prog(&"(seq (let n 2) (let y 3) (+ (- (* n y) 3) 2 (/ 4 n)))".to_string());
    let r2 = "7";
    assert_eq!(r1, r2);
}

#[test]
fn run_prog_tests_2() {
    let r1 = run_prog(&"(seq (+ (- (seq (let n 4)) 3) 2 (/ 4 n)))".to_string());
    let r2 = "4";
    assert_eq!(r1, r2);
}

#[test]
fn run_prog_tests_3() {
    let r1 = run_prog(&"(seq (let n 0) (while (< n 10) (let n (+ n 1))))".to_string());
    let r2 = "10";
    assert_eq!(r1, r2);
}

#[test]
fn run_prog_tests_4() {
    let r1 = run_prog(&"(if (> 3 2) 123 432)".to_string());
    let r2 = "123";
    assert_eq!(r1, r2);
}

#[test]
fn run_prog_tests_5() {
    let r1 = run_prog(&"(if (< 3 2) 123 432)".to_string());
    let r2 = "432";
    assert_eq!(r1, r2);
}
