# Faux Lisp 

A faux lisp interpreter written in Rust. A general programming language with a lisp-like syntax. It currently supports basic expressions like while loops, if statements, functions, modules, and variables; as well as standard data types like strings, numbers, and booleans.

# Build and Run
 To build the interpreter, first clone the repository. Then ,simply run `cargo build --release` in the main directory of the project. After that, you should be able to find the binary under `target/release`.

# Features
The most basic program looks like this:
```
(+ 1 2)
```
This is a simple program that adds together 1 and 2. However, this does not actually output anything. To do that, we will need:
```
(println (+ 1 2))
```
With this, we will print out the result of this addition. Note that the language currently only supports the basic 4 math operations: +, -, \*, and /.

For more complex programs, you may need to utilize control flow. Currently two forms of control flow are supported, while loops and if statements.
```
(while <cond> 
  <body>)
```

```
(if <cond>
  <true-cond>
  <false-cond>)
```

For if statements, a false condition is not absolutely necessary, and not including one can be quite useful when programming sequentially.
```
(if <cond>
  <true-cond>)
```

Numbers can be compared using standard comparison operators: <, >, <=, >=, ==, and !=.

To fully take advantage of these control flow options, you will also need variables. Variables can be denoted using `(let <var-name> <val>)`. Using a similar syntax, you can also create functions:
```
(let (<func-name> <func-params>)
  <func-body>)
```

The language is by default functional, and so sequential programming requires you to use `(seq <stmt1> <stmt2> ...)`. 

Doing so allows the user to use certain language features more effectively:
```
(seq
  (let n 2)
  (println n))
```

Below are a few examples of what can be done in this language:

Here is a program that outputs the nth Fibonacci number iteratively:
```
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
```

Here is a program that outputs the nth Fibonacci number recursively:
```
(seq
  (let (fib n)
    (if (== n 0)
      1
    (if (== n 1)
        1
        (+ (fib (- n 2)) (fib (- n 1))))))

  (println (fib 5))
)
```

You can also store and separate your code in several different files, and import them into your main file using `(import <file-name>)`. This will allow you to import all the functions and variables defined in another file into your current one.
```
;; main.ls
(import "module.ls")
(print testvar)
```

```
;; module.ls
(let testvar 4)
```

Working with data types like booleans and strings is as you would expect.
```
(print (+ "hello " "there"))
(print (if (&& true (== "first" "second")) (len "true case") "false case"))
(print (== "3" (string (num "3"))))
```


