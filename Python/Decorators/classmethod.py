"""
This is used to get access to a class attribute.
Something that is not an attribute of an instance.
"""

class Person:
    species = "Homo sapiens"

    @classmethod
    def get_species(cls):
        print(cls)
        return cls.species

# Usage
print(Person.get_species())
