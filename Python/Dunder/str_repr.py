class Car:
    def __init__(self, make, model, year):
        self.make = make
        self.model = model
        self.year = year

    # __str__ is used for user-friendly output
    def __str__(self):
        return f"{self.year} {self.make} {self.model}"
    
    # __repr__ is used for more detailed unambiguous output
    def __repr__(self):
        return f"Class(make='{self.make}', model='{self.model}', year='{self.year}')"

my_car = Car("Lamborghini", "Aventador", "2027")

print(str(my_car))
print(repr(my_car))
