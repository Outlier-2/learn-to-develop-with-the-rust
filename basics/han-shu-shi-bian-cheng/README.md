---
icon: function
---

# 函数式编程

Rust 作为一种系统编程语言，主要以安全和高效著称。尽管它并不是一门纯函数式编程语言，但它借鉴了许多函数式编程语言的特性，提供了强大的函数式编程支持。这些特性包括闭包、高阶函数、迭代器和模式匹配等。

**1. 闭包（Closures）**

闭包是一种可以捕获其环境中变量的函数。与普通函数不同，闭包可以访问在其定义时存在于作用域中的变量，而不需要显式传递这些变量。

```rust
fn main() {
    let x = 10;
    let add_x = |y| y + x;
    
    println!("Result: {}", add_x(5)); // 输出 15
}
```

在这个例子中，`add_x` 是一个闭包，它捕获了环境中的变量 `x`，并将其与参数 `y` 相加。

闭包的类型是匿名的，因此通常使用 `impl Fn` 特性进行传递。

```rust
fn apply<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    let x = 10;
    let add_x = |y| y + x;
    
    println!("Result: {}", apply(add_x)); // 输出 13
}
```

**2. 高阶函数（Higher-Order Functions）**

高阶函数是指接受函数作为参数或返回一个函数的函数。Rust 支持高阶函数，使得函数的组合和复用变得更加容易。

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn filter<F>(arr: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    arr.into_iter().filter(|x| f(*x)).collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_numbers = filter(numbers, is_even);
    
    println!("{:?}", even_numbers); // 输出 [2, 4, 6]
}
```

在这个例子中，`filter` 函数接收另一个函数 `is_even` 作为参数，返回经过过滤后的新数组。

**3. 迭代器（Iterators）**

迭代器是 Rust 中用于遍历集合（如数组、向量等）的强大工具。通过使用迭代器，您可以轻松地进行懒惰计算和链式调用。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    
    println!("Sum: {}", sum); // 输出 15
}
```

迭代器可以链式调用多个方法，例如 `map`、`filter` 和 `collect`，从而实现强大的数据处理能力。

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let even_squares: Vec<i32> = numbers.iter()
                                        .filter(|x| *x % 2 == 0)
                                        .map(|x| x * x)
                                        .collect();
    
    println!("{:?}", even_squares); // 输出 [4, 16, 36]
}
```

**4. 模式匹配（Pattern Matching）**

模式匹配是函数式编程的一个重要概念，Rust 提供了强大的模式匹配功能，使得代码更加简洁和易读。最常用的模式匹配结构是 `match` 表达式。

```rust
fn main() {
    let number = 2;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

在这个例子中，`match` 表达式根据 `number` 的值执行不同的分支。

除了 `match`，Rust 还提供了 `if let` 和 `while let` 语法，用于简化某些特定的模式匹配场景。

```rust
fn main() {
    let some_value = Some(10);
    
    if let Some(x) = some_value {
        println!("Value: {}", x); // 输出 10
    }
}
```

**5. `Option` 和 `Result` 类型**

Rust 的标准库中有两个重要的枚举类型：`Option` 和 `Result`，它们广泛应用于函数式编程中。

* `Option<T>` 用于表示一个值可能存在或不存在。它有两个变体：`Some(T)` 表示存在一个值，`None` 表示没有值。

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
```

* `Result<T, E>` 用于表示一个操作可能成功或失败。它有两个变体：`Ok(T)` 表示成功，`Err(E)` 表示失败。

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    match divide(4.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
```

**6. `unwrap`、`expect` 和 `?` 操作符**

`unwrap` 和 `expect` 是 Rust 中常用的处理 `Option` 和 `Result` 的方法，它们会在值不存在或操作失败时引发程序崩溃。

```rust
fn main() {
    let some_value = Some(10);
    let value = some_value.unwrap(); // 如果 `some_value` 是 `None`，则程序崩溃
    
    let some_result: Result<i32, &str> = Ok(42);
    let result = some_result.expect("Something went wrong"); // 如果是 `Err`，则输出错误信息并崩溃
}
```

`?` 操作符用于传播错误，它可以自动将 `Result` 或 `Option` 中的错误向上传播，从而简化错误处理代码。

```rust
fn read_number_from_file() -> Result<i32, io::Error> {
    let mut file = File::open("number.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Could not parse number"))?;
    Ok(number)
}
```

在这个例子中，`?` 操作符会在发生错误时自动返回 `Err`，从而避免了大量的 `match` 代码。

#### 总结

Rust 提供了丰富的函数式编程特性，包括闭包、高阶函数、迭代器、模式匹配和错误处理等。这些特性不仅增强了代码的简洁性和可读性，也提升了代码的安全性和性能。通过掌握这些函数式编程技巧，您可以在 Rust 中编写更加优雅和高效的代码。
