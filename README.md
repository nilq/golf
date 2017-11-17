## golf

a sweet matching oriented language, that transpiles to lua.

### syntax examples

hello world
```
"yo world" |> print
```

recursive fib
```
fib = {
  |0| 0
  |1| 1
  |n| fib (n - 1) + fib (n - 2)
}
```

flow-control
```
if = {
  |true body|    body!  
  |false _ body| body
}

max = {
  |a b|
    if (a < b) {
      b
    } {
      a
    }
}
```
