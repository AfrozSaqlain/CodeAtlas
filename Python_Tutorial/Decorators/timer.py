"""
A decorator is a function that takes another function as an argument
and returns a new function that extends or modifies the behavior
of the original function.

Inside the decorator, we define a wrapper function. The wrapper
can accept any number of positional and keyword arguments (*args, **kwargs)
so it can handle any decorated function.

The wrapper calls the original function, optionally adds extra behavior
(before or after the function call), and returns the result.

Finally, the decorator returns the wrapper function.
"""

import time

def timer(func): # This is the decorator
    def wrapper(*args, **kwargs): # This means that aur decorated function can take any number of positional arguments and any number of keyword arguments
        start = time.time()
        result = func(*args, **kwargs) # Call the decorated function
        end = time.time()
        print(f"Function {func.__name__!r} ran in {end - start} seconds")
        return result
    return wrapper

@timer
def example_function(n):
    return f"The sum is {sum(range(n))}"

# The above code is similar to the below one
#example_function = timer(example_function)

print(example_function(10000000))

