"""
When we make recursive calls using a function, it's better to cacche the
results for faster execution of the code.
"""

import functools

def fibonacci(n):
    if n < 2:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)

# This is a custom fibonacci function that uses cache
def fibonacci_cache(n, cache={}):
    if n in cache:
        return cache[n]

    if n == 0:
        return 0
    elif n == 1:
        return 1

    cache[n] = fibonacci_cache(n - 1, cache) + fibonacci_cache(n - 2, cache)
    return cache[n]


# We can decorate the function using a decorator from functools
@functools.cache
def fibonacci_decorated(n):
    if n < 2:
        return n
    return fibonacci_decorated(n - 1) + fibonacci_decorated(n - 2)

print(fibonacci_decorated(40))
