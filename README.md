# RUST PROGRAMMING LANGAUGE CRASH COURSE

Rust is a safe, cocurrent system programming language developed by a mozila employee -
 Graydon Hoare in 2006

## main file

```rs
fn main() {
    println!("Your application starts here!")
}
```

## print command

```rs
fn main(){
    let x:str = "you";
    println!("I love {}", x)
}
```

## Variables

Variables hold primitive data or references to data. Variables are immutable by default.
Rust is a block-scoped language

```rs
fn main(){
    let x:str = "you";
    const Y:f32 = 2.5; // constant should be in cap
}
```

## Data Types

Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.

### Scalar (Primitive)

- Integers: u8, i8, u16, u32, i32, u64,i64, u128, i128 (number of bits they take in memory)
  
  ```rs
    fn main(){
        let x:i32 = 39;
        const Y:u8 = 5;
    }
   ```

- Floats:  f32, f64
  
    ```rs
    fn main(){
        let x:f32 = 0.5;
        const Y:f64 = 2.5;
    }
   ```

- Boolean (bool)

    ```rs
    fn main(){
        let x = true;
        const Y:bool = false;
    }
   ```

- Characters (char)

    ```rs
    fn main(){
        let x:char = 'a';
    }
    ```

### Compound

- Tuples -  A list of elements of different data types, which has a max of 12 elements
  
    ```rs
    fn main(){
        let person: (&str, &str, i8) = ("Chinedu", "Achina", 27);
    }
    ```

- Arrays - Fixed list of elements, which are all of the same data types  
  
    ```rs
    fn main(){
        let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    }
    ```

## Strings

This is an array of characters

```rs
fn main(){
let x = String::new("Hello");
```

## Vectors

This allows you to store more than one value in a single data structure,
that puts all the values next to each other in memory

```rs
fn main(){
let v:Vec<i32> = Vec::new();
let v = vec![1,2,3,4,5]
```

## Conditionals

This is a set of rules performed if a certain condition is met. It is like an 'IF-THEN' statement.
(IF a condition is met, THEN an action is performed)

```rs
fn main(){
    let mut x:i32 = 3;
    if x < 0 {
        println!("{} is negative", x);
    }else if x > 0 {
        println!("{} is positive", x);
    }else{
        println!("{} is zero", x);
    }
}
```

## Loops

This is used to repeat a specific block untill some end condition is met

### Loop

```rs
fn main(){
    let mut i = 0;

    loop{
        println!("The number is {}", i);
        i += 1;
    }
}
```

### For Loop

this is a repeatedly executs the loop code while a given condition is TRUE.
It tests the condition before executing the body loop.

```rs
fn main(){
    let  a = [10,20,30,40];

    for i in a.iter(){
        println!("The number is {}", i);
     }
}
```

### While Loop

While the *condtion is true*, the code within the loop is executed.

```rs
fn main(){
    let mut i = 0;

    while i < 5 {
        println!("The number is {}", i);
        i += 1;
    }
}
```

## Functions

This is a block of orgazined, reusable code that is used to perform a single, related action.

```rs
fn main(){
    prod(3,9);
}

fn prod(x:i32, y:i32){
    println!("The product of {} & {} is: {}", x, y, x*y);
}
```

## Pointer Reference

## Structure

This is a user defined data type that consist of variables of different data type.

```rs
struct rectangle{
    width:u32,
    height:u32
}

fn main(){
    let rect = rectangle {width: 20, height: 10};
    println!("Th area of the rectangle is {}", area(&rect));
}

fn area(Rectangle: &rectangle) -> u32{
    Rectangle.width * Rectangle.height
}

```

## Enums

This is a custom data type which contains some definite values.

```rs
enum Rainbow{
    violet=1; // variant
    indigo=2;
    blue=3;
    green=4;
    yellow=5
    orange=6;
    red=7;
}
```

## Ownership

When a block of code owns a resource, it is known as ownership.

```rs
fn main(){
    let x:i32 = 3;
    let y:i32 = x; // x is destroyed!!
}
```

## Modules

This is a namespace which contains the definations of the functions or its types.
It is a collection of items such as functions, traits, struct, blocks.

```rs
mod cplus {
    pub fn cplus() {
        println!("C++ is an object oriented programming language")
    }
}
```
