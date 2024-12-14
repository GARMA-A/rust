### you can declare a variables using the `let` and `const`

```rust
    let mut y = 10 ;
    y = 9;
    const x :i32 = 0;
    x = 12; // error

```
### the `mut` key word tell rust compiler that is this variable 
### i can change it's value later 

### loops in rust

```rust
    for x in (1..10).step_by(2) {
        println!("{}" , x); // 1 3 5 7 9
    }
```

```rust
    let mut x: i32 = 0;
    loop {
        x += 1;
        println!("{}", x); // 1 2 3 4 5 7 8 9 10
        if x == 10 {
            break;
        }
    }
```

```rust
    let mut x = 0;
    while x <= 10 {
        println!("{}", x);
        x += 1;
    }
```

##  methods on rust 

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```


## Enums in rust


```rust
enum IpAddrKind {
    V4,
    V6,
}
```
```rust
 let four = IpAddrKind::V4;
 let six = IpAddrKind::V6;
```

```rust
  enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
```


```rust
    let x  = 5;
    let y : Option<i8> = Some(5);

    println!("{}" , x+y.unwrap_or(1)); // sum x+y and if y = nothing the default = 1
    // output : 10 
```
```rust 

    let x  = 5;
    let y : Option<i8>  = None; 

    println!("{}" , x+y.unwrap_or(1)); // output : 6


```

### the option type on rust work as the value of y can be `i8` or nothing

### enums with the match 

```rust
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn main()  {

    let c : Coin = Coin::Quarter ;

    println!("{}" , values_in_cents(c)); // output 25

}

fn values_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1 ,
        Coin::Nickel => 5 , 
        Coin::Dime => 10,
        Coin::Quarter=> 25
    }
}

```

### adnvanced matching 
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Exiting...");
            // additional logic if needed
        }
        Message::Move { x, y } => {
            println!("Move to coordinates: ({}, {})", x, y);
            if x > 10 && y > 10 {
                println!("Both x and y are greater than 10!");
            }
        }
        Message::Write(text) => {
            println!("Writing: {}", text);
            let len = text.len();
            println!("The length of the text is: {}", len);
        }
    }
}

fn main() {
    let msg = Message::Move { x: 15, y: 20 };
    process_message(msg);
}

```

### Optional Type with functions 
```rust

fn main()  {

    let num : Option<i8> = Some(2);
    println!("{:?}" , plus_one(num)); // Some(3)
    let num2 : Option<i8> = None;
    println!("{:?}" , plus_one(num2)); // None
}



fn plus_one(x :Option<i8>) -> Option<i8> {
    match x {
        None=> None,
        Some(i) => Some(i+1)
    }
}

```




### String::from("127.0.0.1")
### Converting a string literal (&str) to a String:

### The string literal "127.0.0.1" is of type &str, which is a reference to a fixed sequence of UTF-8 bytes stored in the program's binary (read-only memory, not heap).
### String::from takes this &str and allocates heap memory to create a growable, mutable string (String).
### Heap memory allocation:

### A String consists of three parts:

### Pointer: Points to the start of the allocated memory on the heap.
### Length: Number of bytes currently in use by the string.
### Capacity: Total number of bytes allocated for future growth.

### In this case, String::from allocates sufficient heap memory to hold "127.0.0.1" (9 bytes, including the null terminator), copies the content from the literal into the heap, and initializes the String struct.

## data structures in rust

### arrays
```rust
    let arr:[i8 ; 4] = [1,2,3,4];
    println!("{:?}" , arr); // output : 1 , 2 , 3 , 4
```

### vectors
```rust
    let vc : Vec<i8>  = Vec::new();
    let mut v = vec![1,2,3];

    v.push(4);
    v.push(5);
    println!("{:?} \n {:?}" , v , vc); // 1,2,3,4,5 // []
    v.pop();
    v.pop();
    v.remove(0);
    println!("{:?} \n {:?}" , v , vc); // 2,3 // []
```
###  access out of bound

```rust
 let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; // panic stop the prgram
    let does_not_exist = v.get(100); // return None there is no panic

```
### note that in case of the vector the error is happen on the runtime
### because vectors stored on the heap but if you have an array it will be 
### compile time error 


### very  important note about ownership inside the for loop 
```rust
 let v = vec![100, 32, 57];

    // Iterating by reference
    for i in &v {
        println!("{i}"); // Borrowing the elements; no ownership change
    }

    println!("Still can use v: {:?}", v);

    // Iterating by value
    for i in v {
        println!("{i}"); // Moves ownership of v's elements
    }

    // println!("{:?}", v); // ERROR: v has been moved
```
### iterate over mut reference
```rust
 let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

```
### create vector with different types

```rust
   enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

```

### extract the option type to variable 

```rust
    let numbers = vec![10, 20, 30, 40, 50];
    let index = 2;

    // Declare the variable with a default value
    let value: i32;

    match numbers.get(index) {
        Some(&v) => {
            value = v; // Assign the value if it exists
        }
        None => {
            value = -1; // Assign a default or sentinel value
            println!("No value at index {}", index);
        }
    }

    // Now `value` is accessible here
    println!("The value is: {}", value);
```

### deque in rust 
```rust 
    let mut dq : VecDeque<i32>= VecDeque::new();

    dq.push_back(1);
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(4);
    dq.push_front(5);
    dq.push_front(6);
    // [6,5,4,1,2,3]
    dq.pop_back(); // pop 3
    dq.pop_back(); // pop 2
    dq.pop_front(); // pop 6
    println!("{:?} " , dq); // [5 , 4 , 1]
```

### error handling in rust

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

### the `unwrap` method
### if the statement return Ok() so retrrn the  value of thr expresion 
### if the expresion return error() so panic and print the error

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
    // there is also expect wich the same as unwrap but let you define the message 
    // of the error you want to print
    let file = File::open("welcome.txt").expect("there is no file with the name welcome ");
}
```
### another way to handle it 
```rust
use std::fs::File;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("welcome.txt")?;
    Ok(())
}
```
### you can return that from the fn you make 
```rust
fn read_the_username() -> Result<String, io::Error>{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

     let mut username = String::new();

    return match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}
```

### convert str to int

```rust
    let str : Result<i32 , _ >  = "25".parse();

    println!("{:?}" , str) ;
```


### trait (interfaces in rust)


### real example
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### another example 
```rust
struct Tweet {
    username: String,
    content: String,
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Display {
    fn display(&self) -> String;
}

// Implement the Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implement the Display trait
impl Display for Tweet {
    fn display(&self) -> String {
        format!("Tweet by @{}", self.username)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("garma"),
        content: String::from("Learning Rust is fun!"),
    };

    println!("{}", tweet.summarize()); // Implements Summary
    println!("{}", tweet.display());  // Implements Display
}
```


### life times in rust
```rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}
// now this code will not run becuase we print r and r is refernce a non-valid 
// memory think of it as we refernce out of bound array indexing
```
```error
error[E0597]: `x` does not live long enough
```

### this is a life time problem !! 
### rust create a new future called life time annotation that is worked for the fn's 


### this for example wil make error at compile time
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
```error
error[E0106]: missing lifetime specifier
```

### the corrected code will be

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
#### this tell the compiler that now there is a life time called `'a` and 
#### x,y parameter has the same life time `'a` but actually that not the case 
### if x has different life time from y the fn will take both and will work for 
### the shortest life time if x it the shorter one and it go's out of scope when 
### we return the refernce of it this is `compile time error` and the same to 
### y parameter 


### let's see what i was talking bout here 
```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }
}
// valid code 
```
### let's make small change 
```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // error under string2 
    }
    println!("The longest string is {result}");
}
// now it's not valid code
```
```error
error[E0597]: `string2` does not live long enough
```
### the longest function will return the sortest life time in this case string2
### compiler will see that result life time is not match the string2 life time 
### so that the error apear

## The 3 rules for the life times
### 1. Each parameter that is a refernce  gets it's own lifetime parameter
### 2. If there is exactly one input lifetime parameter , that life time 
### is Assigned to all output life time parameters 

### 3. if there is multiple input lifetime parameters , but one of them is 
### &self or &mut self the lifetime of self is Assigned to all output life time parameters

### this is the rules that the compiler will follow any thing out this rules you will 
### need to manualy write them 


### the third rule applied
```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
```















