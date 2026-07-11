# Lua: A Basic Introduction

Lua is a small, lightweight scripting language. It is commonly used for game scripting, configuration, plugins, and embedded scripting inside larger applications.

## Printing Text

The simplest Lua program prints text:

```lua
print("Hello World!")
```

Lua does not require semicolons at the end of statements.

## Variables

Variables can be created without declaring a type:

```lua
name = "Alex"
score = 10
is_alive = true

print(name)
print(score)
print(is_alive)
```

Lua decides the type from the value.

## Local Variables

Use `local` when a variable should only exist in the current block or function:

```lua
local health = 100
```

Using `local` is a good habit because it avoids accidentally changing global values.

## Functions

Functions group reusable behavior:

```lua
local function greet(name)
    print("Hello, " .. name)
end

greet("Alex")
```

The `..` operator joins strings together.

## Tables

Tables are Lua's main data structure. They can work like arrays, dictionaries, or simple objects:

```lua
local player = {
    name = "Hero",
    health = 100
}

print(player.name)
print(player.health)
```

## Input And Random Numbers

Lua can read terminal input with `io.read()`:

```lua
io.write("Enter a number: ")
local guess = tonumber(io.read())
```

`tonumber()` converts text input into a number.

Random numbers can be generated with `math.random()`:

```lua
math.randomseed(os.time())
local hidden = math.random(1, 100)
```

## Run A Lua Script

```bash
lua intro.lua
```
