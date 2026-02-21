"""
Used to denote a method inside of a class as static.

Static method means something that belongs to a class,
and not to an instance of a class. It won't have a 'self' argument.

"""


class Math:
    @staticmethod
    def add(x, y):
        return x + y

    @staticmethod
    def multiply(x, y):
        return x * y

# Usage
print(Math.add(5, 7))

print(Math.multiply(3, 6))


m = Math()

print(m.add(2, 7))
