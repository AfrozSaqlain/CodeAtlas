"""
Property decorator is used for various methods inside a class.

When there's an '_' before a variable name that means we shouldn't 
access it from outside of that class as this is a 'private attribute'.
Note that technically we can do that but we shouldn't.

We use @property in combination with a private attribute so that we can control
the access of that attribute
"""

class Circle:
    def __init__(self, radius):
        self._radius = radius
    
    @property                       # This is a getter
    def radius(self):
        """Get the radius of the circle"""
        print("Called getter")
        return self._radius
    
    @radius.setter                  # This is a setter
    def radius(self, value):
        """Set the radius of the circle. Must be positive"""
        if value > 0:
            self._radius = value
        else:
            raise ValueError("Radius must be positive")

    @property
    def diameter(self):
        """Get the diameter of the circle"""
        return self._radius * 2
    
    @radius.deleter
    def radius(self):
        print("Deleted")
        del self._radius
    
# Usage
c = Circle(5)
print(c.radius) # This is called using getter 
print(c.diameter) # This is called using getter 

c.radius = 10 # We call setter here to assign value to this radius
print(c.radius) # This is called using getter 
print(c.diameter) # This is called using getter 

del c.radius
print(c.radius) # This will throw a ValueError
