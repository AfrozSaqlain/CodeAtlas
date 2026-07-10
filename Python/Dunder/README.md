Everything that one creates in python is an object.

For example:

- A string, such as "Hello" is an object of type "str"
- Any number, say 5, is an object of type "int"
- A function, say def add(), is an object of type "function"

Now why we can add to strings is because these two are objects of same class and there's a dunder method defined which is mapped to the "+" operator.

```
str1 == "Hello"
str2 = "World"

new_str = str1 + str2
```

So in "str" class, there's a dunder method known as `__add__()` which defines what happens when we add two string objects.
