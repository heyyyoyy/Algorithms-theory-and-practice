
def fib_mod(n, m):
    fibPrev, fib = 0, 1
    cached = [fibPrev, fib]

    for _ in range(1, n):
        fibPrev, fib = fib, (fib + fibPrev) % m
        if fibPrev == 0 and fib == 1:
            cached.pop()
            break
        else:
            cached.append(fib)

    offset = n % len(cached)
    print(cached[offset])
