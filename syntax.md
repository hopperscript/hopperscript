# The Hopperscript Language (Planned) Syntax

Hopperscript is a trying-to-be-simple text language for [Hopscotch](https://gethopscotch.com). This file contains the planned syntax for the language.

By default, Hopperscript doesn't know anything about blocks (like *Set Color*, *Move Forward*, etc), rules (like *game starts*), objects (*Circle*, *Monkey*, etc) and everything that isn't the base of the Hopscotch language. But Hopperscript supports importing blocks (with block lists), so you can use Hopscotch blocks (including secret blocks) on Hopperscript. Block lists are files written in [Rhai](https://rhai.rs/) that contains blocks, rules, objects, traits, colors, and more, that Hopperscript can import.

## Defining things
Every definition of a variable, abilities, custom rules, etc *should* be done at the very top of the file. To define something, use the `define` keyword.

### Global Variables
Use `define var"<variable name>"` to define game/global variables:

```
define var "my variable"
define var "myVariable"

...
```

### Objects
Use `define object(objects.<object type>, "<name>")` to define objects. The object types are provided by the block list.

```
define object(objects.PARALLELOGRAM, "Parallelogram 1")
define object(objects.TOUCAN, "My Toucan")

...
```

### Custom Abilities
Use `define ability "<ability name>" { (* code *) }` to define custom abilities.

```
define object(objects.SQUARE, "My Object")
define ability "Say Hello" {
  setText("hello universe", colors.RANDOM)
}

for My Object" {
  when rules.GAME_STARTS() {
    abilities."Say Hello"()
  }
}
```

## Objects
To edit what's inside an object, use the `for` keyword:

```
define object(objects.TOUCAN, "My wonderful toucan")

for "My wonderful toucan" {
  (* your rules... *)
}
```

## Rules
Rules are pretty simple. Just use the `when` keyword:

```
when rules.GAME_STARTS() {
  (* your blocks... *)
}
```

## Blocks
Blocks are also simple. Pretty much self-explanatory:

```
when rules.GAME_STARTS() {
  moveForward(10)
  setPosition(300, 400)
}
```

## Variables
You can use global variables in parameters like this: `moveForward(var"My Variable")`

## Comments
You may see something like `(* this *)` a few times in this file, and they are comments. You can place them in your code and the language will ignore it. We may also compile them to the *Comment* block.
