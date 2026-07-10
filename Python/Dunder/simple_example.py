class Counter:
    def __init__(self):
        self.value = 1

    def count_up(self):
        self.value += 1

    def count_down(self):
        self.value -= 1

    def __str__(self):
        return f"Count = {self.value}"

    def __add__(self, other): # When we do str1 + str2, python tries to find __add__() method in first object and the 2nd object is then used as a parameter here, as "other"
        if isinstance(other, Counter):
            return self.value + other.value
        else:
            raise Exception("Invalid Type")

count1 = Counter()
count2 = Counter()

count1.count_up()
count2.count_up()

print(count1, count2) # ---> This will only print the value when we have defined __str__() method.
print(count1 + count2) # ---> This will work only when we have defined __add__() method
print(count1 + 2)
