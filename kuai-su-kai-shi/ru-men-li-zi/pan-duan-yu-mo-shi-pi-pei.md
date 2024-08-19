# 📏 判断与模式匹配

Rust 提供了多种方式来进行条件判断和模式匹配，最常见的包括 `if`、`match`、模式匹配表达式以及错误处理的 `Result` 类型。下面将详细解释这些语法，并提供相关示例。

### 1. 可驳回与不可驳回匹配模式

在Rust中，模式匹配分为两种类型：**可驳回匹配（refutable patterns）和不可驳回匹配（irrefutable patterns）**。

#### 1.1 不可驳回匹配

不可驳回匹配是指任何情况下都会成功的匹配。这种模式通常出现在`let`绑定中，因为Rust要求`let`语句必须总是成功。

```rust
let x = 5;  // 这里的模式 `x` 是不可驳回的，因为无论右边的值是什么，都会匹配。
```

#### 1.2 可驳回匹配

可驳回匹配是指在某些情况下可能会失败的匹配。这类匹配通常出现在`if let`、`while let`和`match`语句中。

```rust
let some_value = Some(10);

if let Some(x) = some_value {
    println!("Matched value: {}", x);  // 这是一个可驳回匹配，因为如果 `some_value` 是 `None`，匹配就会失败。
} else {
    println!("No match");
}
```

在 `match` 表达式中，如果模式没有被穷尽匹配，就会产生编译错误，因此通常会使用通配符 `_` 来处理剩余的情况：

```rust
let number = 7;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),  // 这里的 `_` 是一个不可驳回模式，它匹配所有剩余的情况。
}
```

### 2. `if` 条件判断

`if` 语句是Rust中最基本的条件判断语句。它用于根据条件表达式的结果来执行不同的代码块。

#### 2.1 基本语法

```rust
let number = 10;

if number > 5 {
    println!("Number is greater than 5");
} else if number < 5 {
    println!("Number is less than 5");
} else {
    println!("Number is equal to 5");
}
```

#### 2.2 单行 `if` 语句

在Rust中，`if` 语句可以直接用作表达式，特别是在单行中可以用于赋值。

```rust
let number = 10;
let is_positive = if number > 0 { true } else { false };
```

### 3. `match` 模式匹配

`match` 是Rust中功能强大且常用的控制流结构。它允许基于模式对一个值进行分支处理，并且在处理枚举类型时尤为有用。

#### 3.1 基本用法

`match` 语句的语法类似于其他语言中的 `switch` 语句，但它不仅支持简单的值匹配，还支持复杂的模式匹配。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;

match direction {
    Direction::Up => println!("Moving up!"),
    Direction::Down => println!("Moving down!"),
    Direction::Left => println!("Moving left!"),
    Direction::Right => println!("Moving right!"),
}
```

#### 3.2 绑定模式中的值

`match` 语句允许在模式中绑定值，这样可以提取结构体、枚举或其他数据类型中的字段值。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Quit => println!("Quit message"),
    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
}
```

#### 3.3 `_` 通配符

在 `match` 语句中，`_` 可以用作通配符，用于匹配所有未被显式匹配的情况。

```rust
let number = 7;

match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}
```

#### 3.4 匹配守卫（Match Guard）

匹配守卫是一个额外的条件，允许在模式匹配中使用 `if` 表达式来进一步限定匹配条件。

```rust
let number = 7;

match number {
    n if n < 5 => println!("Less than 5"),
    n if n == 5 => println!("Equal to 5"),
    _ => println!("Greater than 5"),
}
```

### 4. `if let` 与 `while let`

Rust 提供了 `if let` 和 `while let` 来简化某些情况下的模式匹配。这些语法适用于只关心某个模式的匹配情况，而不需要处理所有的可能性。

#### 4.1 `if let`

`if let` 语句用于当一个模式匹配成功时执行代码，而忽略其他可能性。

```rust
let some_value = Some(10);

if let Some(value) = some_value {
    println!("Matched value: {}", value);
} else {
    println!("No match");
}
```

#### 4.2 `while let`

`while let` 语句用于在一个模式匹配成功时执行循环体，并在匹配失败时退出循环。

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Top value: {}", top);
}
```

### 5. `Result` 类型与错误处理

Rust 的 `Result` 类型广泛用于错误处理。它是一个枚举类型，具有两个变体：`Ok(T)` 表示操作成功，并包含成功时的返回值；`Err(E)` 表示操作失败，并包含错误信息。

#### 5.1 基本用法

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
```

#### 5.2 `unwrap` 和 `expect`

`Result` 类型有两种常用的方法用于快速处理结果：`unwrap` 和 `expect`。这两种方法在操作失败时会导致程序崩溃。

*   `unwrap`：如果 `Result` 是 `Ok`，则返回内部值；如果是 `Err`，则调用 `panic!` 宏。

    ```rust
    let result = divide(10, 2).unwrap();
    println!("Result: {}", result);
    ```
*   `expect`：与 `unwrap` 类似，但在 `panic!` 时可以提供自定义的错误信息。

    ```rust
    let result = divide(10, 0).expect("Division failed");
    ```

#### 5.3 链式错误处理 (`?` 运算符)

Rust 提供了 `?` 运算符来简化错误处理。它可以用于 `Result` 类型的链式调用，使得代码更加简洁。

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn calculate() -> Result<(), String> {
    let result = divide(10, 2)?;
    println!("Result: {}", result);
    Ok(())
}
```

在上面的例子中，`?` 运算符自动处理了 `Result` 的错误分支，并在发生错误时返回错误。

### 6. 综合示例

以下示例展示了如何使用 `match`、`if let` 和 `Result` 类型在 Rust 中进行复杂的条件处理和错误处理。

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let coin = Coin::Penny;
    let cents = value_in_cents(coin);
    println!("Value in cents: {}", cents);

    let result = divide(10, 0);

    if let Err(e) = result {
        println!("Error: {}", e);
    } else {
        println!("Result: {:?}", result.unwrap());
    }
}
```

通过这些示例，你可以更加深入地理解 Rust 中的判断语句、模式匹配与错误处理机制。
