
def fib_digit(n):
    assert(1 <= n <= 10**7)
    a, b = 0, 1
    for _ in range(1, n):
        a, b = b % 10, (a + b) % 10
    return b

assert(fib_digit(10) == 5)
assert(fib_digit(55) == 5)
assert(fib_digit(210) == 0)
assert(fib_digit(1111) == 9)
print('Done!')
