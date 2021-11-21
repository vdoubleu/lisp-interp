# Faux Lisp 

A faux lisp interpreter written in Rust. A general programming language with a lisp-like syntax. It currently supports basic expressions like while loops, if statements, and variables.

```
(while <cond> 
  <body>)
```

```
(if <cond>
  <true-cond>
  <false-cond>)
```

```
(if <cond>
  <true-cond>)
```

```
(seq
  (let <var> <expr>)
  <stmts>)
```
