# Global static

The same as `simple` example, but uses `std::sync::LazyLock`
to avoid passing the spanner to functions

This is the intended way to use this library

The spanner is configured to not skip any vertical lines

This example outputs the following trace:

```plaintext
┌fib(5)
| ┌fib(4)
| ¦ ┌fib(3)
| ¦ ┆ ┌fib(2)
| ¦ ┆ └fib(2)
| ¦ ┆ ┌fib(1)
| ¦ ┆ └fib(1)
| ¦ └fib(3)
| ¦ ┌fib(2)
| ¦ └fib(2)
| └fib(4)
| ┌fib(3)
| ¦ ┌fib(2)
| ¦ └fib(2)
| ¦ ┌fib(1)
| ¦ └fib(1)
| └fib(3)
└fib(5)
```
