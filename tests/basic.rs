use interp::runner::run_prog;

#[test]
fn basic_1() {
    let r1 = run_prog(&"(seq (let n 2) (let y 3) (+ (- (* n y) 3) 2 (/ 4 n)))".to_string());
    let r2 = "7";
    assert_eq!(r1, r2);
}

#[test]
fn basic_2() {
    let r1 = run_prog(&"(seq (+ (- (seq (let n 4)) 3) 2 (/ 4 n)))".to_string());
    let r2 = "4";
    assert_eq!(r1, r2);
}

#[test]
fn basic_3() {
    let r1 = run_prog(&"(seq (let n 0) (while (< n 10) (let n (+ n 1))))".to_string());
    let r2 = "10";
    assert_eq!(r1, r2);
}

#[test]
fn basic_4() {
    let r1 = run_prog(&"(if (> 3 2) 123 432)".to_string());
    let r2 = "123";
    assert_eq!(r1, r2);
}

#[test]
fn basic_5() {
    let r1 = run_prog(&"(if (< 3 2) 123 432)".to_string());
    let r2 = "432";
    assert_eq!(r1, r2);
}

#[test]
fn basic_6() {
    let r1 = run_prog(&"(seq (let (add n1 n2) (+ n1 n2)) (add 1 2))".to_string());
    let r2 = "3";
    assert_eq!(r1, r2);
}

#[test]
fn basic_7() {
    let r1 = run_prog(&"(seq (let (f a) (seq (print a) (if (< a 5) (f (+ a 1)) 0))) (f 0))".to_string());
    let r2 = "0";
    assert_eq!(r1, r2);
}

#[test]
fn basic_8() {
    let r1 = run_prog(&"(> 1 2)".to_string());
    let r2 = "false";
    assert_eq!(r1, r2);
}

#[test]
fn basic_9() {
    let r1 = run_prog(&"(> 2 1)".to_string());
    let r2 = "true";
    assert_eq!(r1, r2);
}

#[test]
fn basic_10() {
    let r1 = run_prog(&"(&& (< 2 1) (> 2 1))".to_string());
    let r2 = "false";
    assert_eq!(r1, r2);
}

#[test]
fn basic_11() {
    let r1 = run_prog(&"(|| (< 2 1) (> 2 1))".to_string());
    let r2 = "true";
    assert_eq!(r1, r2);
}

#[test]
fn basic_12() {
    let r1 = run_prog(&"(+   \"front \"   \"mid \"  \"back\")".to_string());
    let r2 = "front mid back";
    assert_eq!(r1, r2);
}

#[test]
fn basic_13() {
    let r1 = run_prog(&"(+ (num \"4\") (num (str 2)) (len \"abcd\"))".to_string());
    let r2 = "10";
    assert_eq!(r1, r2);
}
