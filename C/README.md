# C: A Basic Introduction

C is a compiled, procedural programming language. It is often used to learn how computers work close to the hardware because it gives you direct control over memory, variables, functions, and program flow.

## Your First C Program

Most C programs start with a `main()` function. This is the entry point: when the program runs, execution begins inside `main()`.

```c
#include <stdio.h>

int main() {
    printf("Hello World!\n");
    return 0;
}
```

`#include <stdio.h>` gives access to standard input/output functions such as `printf`.

`printf()` prints text to the terminal.

`return 0` tells the operating system that the program finished successfully.

## Functions

Functions let you group reusable logic.

```c
void message() {
    printf("Hello World!\n");
}
```

The `void` return type means the function does not return a value. You can call the function from `main()`:

```c
int main() {
    message();
    return 0;
}
```

## Compile And Run

C code must be compiled before it can run:

```bash
gcc -o hello hello.c
./hello
```

`gcc` is the compiler, `hello.c` is the source file, and `hello` is the executable that gets created.

Note: We will add a .o extension to the bianries compiled which will make no difference but it's a good practise.
