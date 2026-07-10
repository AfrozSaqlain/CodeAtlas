str1 = "Hello"
str2 = "World"

new_str = str1.__add__(str2)

# Above method is same as
# new_str = str1 + str2

print(new_str)

# For finding the length of a string
# print(len(str1))

# Or we can use dunder method
print(str1.__len__())
