
def fib(n):
    assert 1 <= n <= 40
    a, b = 0, 1
    for _ in  range(n-1):
        a, b = b, a + b
    return b

assert fib(5) == 5
assert fib(10) == 55
assert fib(40) == 102334155
