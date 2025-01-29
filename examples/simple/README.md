# A simple example

This examples passes the default spanner to function that is traced

This example outputs the following trace:

```plaintext
┌fib5
|  fib4
|   ┌fib3
|   ┆  fib2
|   ┆  fib2
|   ┆  fib1
|   ┆  fib1
|   └fib3
|   ┌fib2
|   └fib2
|  fib4
|  fib3
|   ┌fib2
|   └fib2
|   ┌fib1
|   └fib1
|  fib3
└fib5
```
