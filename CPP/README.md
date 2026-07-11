# C++: A Basic Introduction

C++ is a compiled programming language built on top of C. It supports procedural programming like C, but also adds object-oriented programming, stronger abstractions, and a large standard library.

## Your First C++ Program

```cpp
#include <iostream>
using namespace std;

int main() {
    cout << "Hello World!" << endl;
    return 0;
}
```

`#include <iostream>` gives access to input and output streams.

`cout` prints text to the terminal.

`cin` reads input from the user.

`endl` ends the current line.

## Variables And Input

C++ variables must have a type:

```cpp
int age;
cout << "Enter your age: ";
cin >> age;
cout << "You are " << age << " years old." << endl;
```

`int` stores whole numbers. The `>>` operator reads input into a variable, and the `<<` operator sends output to the terminal.

## Functions

Functions let you reuse logic:

```cpp
int add(int a, int b) {
    return a + b;
}
```

## Compile And Run

```bash
g++ -o math_op math_op.cpp
./math_op
```

`g++` compiles C++ source code into an executable program.

Note: We will add a .out extension to the bianries compiled which will make no difference but it's a good practise.
