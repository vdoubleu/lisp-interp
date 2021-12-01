# Faux Lisp 

A faux lisp interpreter written in Rust. A general programming language with a lisp-like syntax. It currently supports basic expressions like while loops, if statements, functions, modules, and variables; as well as standard data types like strings, numbers, booleans, and even lists.

# Build and Run
 To build the interpreter, first clone the repository. Then, simply run the provided install.sh script like so `./build_install.sh` in the main directory of the project. This should build, test, and install the binary into the appropriate location. If you would like to install the binary to an alternate location, you made need to modify the script. The script will also copy over stdlib files into a standard location. If you wish to install these elsewhere, you may need to include the path where you have them installed when you run the interpreter. Run `fl --help` for a list of flags available to you for when you run the interpreter.

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

Working with lists is similar to other functional languages. The functions `(first <list>)` and `(rest <list>)` are defined for lists and are useful getting elements in a list. You can also extend lists using `(cons <element> <list>)`. To define large lists, instead of chaining `cons` operators together, you can also use square bracket notation like so: `(print [1 2 3 4 5])`.

Below is an example of a function that reverses a list:
```
(let (reverse full-lst)
  (seq
    (let (rev-help lst acc)
      (if (== 0 (len lst))
        acc
        (rev-help (rest lst) (cons (first lst) acc))))
    (rev-help full-lst [])))
```
