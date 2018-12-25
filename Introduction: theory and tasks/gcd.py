
def gcd(a, b):
    while all((a, b)):
        if a >= b:
            a = a % b
        else:
            b = b % a
    return max(a, b)

assert(gcd(18, 35) == 1)
assert(gcd(14159572, 63967072) == 4)
print('Done!')
