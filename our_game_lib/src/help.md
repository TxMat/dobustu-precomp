# Rust Cheat Cheat

## Table of Contents

- [Rust Cheat Cheat](#rust-cheat-cheat)
  - [Table of Contents](#table-of-contents)
  - [Rust](#rust)
    - [Basic](#basic)
      - [Variables](#variables)
      - [Operators](#operators)
      - [Functions](#functions)
      - [Pattern Matching](#pattern-matching)
      - [Structs](#structs)
      - [Enums](#enums)
      - [Ownership](#ownership)
      - [References](#references)
      - [Slices](#slices)
      - [Iterators](#iterators)
      - [Traits](#traits)
    - [Intermediate](#basic)
      - [Macros](#macros)
      - [Closures](#closures)
      - [Modules](#modules)
      - [Error Handling](#error-handling)
      - [Testing](#testing)
      - [Documentation](#documentation)
    - [Advanced](#basic)
      - [Unsafe](#unsafe)

## Rust

### Basic

#### Variables

in rust, variables are immutable by default. To make a variable mutable, use the `mut` keyword.
use `let` to declare a variable.

you can also specify the type of the variable using `:`

```rust
fn main() {
    let x = 5;
    let y: i32 = 5;
    let mut z = 5;
}
```

rust will also infer the type of the variable if you don't specify it.

```rust
fn main() {
    let x = 5; // i32
    let y = 5.0; // f64
    let z = true; // bool
    let a = 'a'; // char
    let b = (1, 2); // tuple
    let c = [1, 2, 3]; // array
}
```

#### Operators

Rust has the following operators:

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `>`, `<`, `>=`, `<=`
- Logical: `&&`, `||`, `!`
- Bitwise: `&`, `|`, `^`, `<<`, `>>`
- Assignment: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`
- Misc: `.` (field access), `::` (method access), `?` (error propagation) `we'll use the '?' operator a lot`
- Range: `..`, `..=`
- Dereference: `*`
- Reference: `&`, `&mut`
- Type-casting: `as`
- Size-of: `sizeof`
- Index: `[]`
- Tuple: `(,)`
- Closure: `||`
- Macro: `!`

#### Functions

Functions are declared using the `fn` keyword. The return type of the function is specified using `->`.

```rust
fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let result = add(5, 5);
    println!("{}", result);
}
```

#### Pattern Matching

Pattern matching is used to match a value against a pattern. It is declared using the `match` keyword and used a lot in rust.

```rust
fn main() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }
}
```

match is similar to switch in other languages, but it's more powerful.

```rust
fn main() {
    fn string_checker(s: String) -> Result<(), &str> {
        if s.len() > 5 {
            Ok(())
        } else {
            Err("string is too short")
        }
    }

    match string_checker(String::from("hello")) {
        Ok(()) => println!("string is valid success"),
        Err(e) => println!("string is invalid {}", e),
    } 
}
```

#### Structs

Structs are used to create custom data types. They are declared using the `struct` keyword.

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 5, y: 5 };
    println!("x: {}, y: {}", p.x, p.y);
}
```

#### Enums

Enums are used to create custom data types. They are declared using the `enum` keyword.

```rust
fn main() {
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}
```

#### Ownership

Rust has a unique ownership system. It allows you to manage memory safely without a garbage collector.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);
    // error: value borrowed here after move
    // s1 has been moved to s2
}
```

#### References

References allow you to borrow a value without taking ownership of it. They are declared using the `&` symbol.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}", len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

#### Slices

Slices are a reference to a portion of a collection. They are declared using the `&[T]` syntax.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}
```

#### Iterators

Iterators are used to loop over a collection. They are declared using the `iter` method.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    for i in a.iter() {
        println!("{}", i);
    }
}
```

#### Traits

Traits are used to define shared behavior between types. They are declared using the `trait` keyword.

```rust
fn main() {
    trait Animal {
        fn make_sound(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn make_sound(&self) {
            println!("woof");
        }
    }

    impl Animal for Cat {
        fn make_sound(&self) {
            println!("meow");
        }
    }

    let dog = Dog;
    let cat = Cat;

    dog.make_sound();
    cat.make_sound();
}
```

### Intermediate

#### Macros

Macros are used to define custom syntax. They are declared using the `macro_rules!` macro.

```rust
fn main() {
    macro_rules! say_hello {
        () => {
            println!("hello");
        };
    }

    say_hello!();
}
```

#### Closures

Closures are used to define anonymous functions. They are declared using the `||` syntax.

```rust
fn main() {
    let add = |x, y| x + y;
    let result = add(5, 5);
    println!("{}", result);
}
```

#### Modules

Modules are used to organize code. They are declared using the `mod` keyword.

```rust
fn main() {
    mod math {
        pub fn add(x: i32, y: i32) -> i32 {
            x + y
        }
    }

    let result = math::add(5, 5);
    println!("{}", result);
}
```

#### Error Handling

Rust has a powerful error handling system. It uses the `Result` type to represent success or failure.

```rust
fn main() {
    use std::fs::File;
    use std::io::Error;

    fn read_file() -> Result<String, Error> {
        let f = File::open("hello.txt")?;
        // note the '?' operator
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_file() {
        Ok(s) => println!("{}", s),
        Err(e) => println!("{}", e),
    }
    
    // or 
    
    if let Ok(s) = read_file() {
        println!("{}", s);
    } else {
        println!("{}", e);
    }
}
```

#### Testing

Rust has a built-in testing framework. Tests are declared using the `#[test]` attribute.

```rust
fn main() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[test]
    fn test_add() {
        assert_eq!(add(5, 5), 10);
    }
}
```

#### Documentation

Rust has a built-in documentation system. Documentation is declared using the `///` syntax.

```rust
fn main() {
    /// Adds two numbers together.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = add(5, 5);
    /// assert_eq!(result, 10);
    /// ```
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}
```

### Advanced

#### Unsafe

Rust has an `unsafe` keyword that allows you to bypass some of the safety checks.

```rust
fn main() {
    unsafe {
        // unsafe code goes here
    }
}
```

we won't use `unsafe` keyword, but it's good to know that it exists.