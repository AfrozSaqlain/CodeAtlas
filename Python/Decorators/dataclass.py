class Prodcut:
    def __init__(self, name: str, price: float, quantity: int = 0):
        self.name = name
        self.price = price
        self.quantity = quantity

    def total_cost(self) -> float:
        return self.price * self.quantity

    def __repr__(self) -> str:
        return (
                f"Product(name={self.name!r}, price={self.price}, quantity={self.quantity})"
                )

    def __eq__(self, other):
        if not isinstance(other, Prodcut):
            return NotImplemented

        return (
                self.name == other.name
                and self.price == other.price
                and self.quantity == other.quantity
                )


p = Prodcut(name="Mac", price=20000, quantity=1)
q = Prodcut(name="Arch", price=20000, quantity=2)

print(p.total_cost())
print(q.total_cost())

print(q)

print(p == q)





"""
All the above things can be simply replaced by dataclass decorator.

We do not need to write __init__, __repr__ and __eq__ method now.
"""
from dataclasses import dataclass

@dataclass
class Prodcut2:
    name: str
    price: float
    quantity: int = 0

    def total_cost(self) -> float:
        return self.price * self.quantity

a = Prodcut2(name='Linux', price=30000, quantity=3)
b = Prodcut2(name='Gentoo', price=50000, quantity=3)

print(a)
print(b)

print(a == b)
print(a.total_cost())
