---
icon: '8'
---

# 猜数字

#### **实现一个猜数字游戏**

在 Rust 中，我们可以利用标准库和一些常用的第三方库来实现一个简单的命令行猜数字游戏。这个游戏的目标是让玩家猜测一个随机生成的数字，直到猜中为止。

**1. 引入必要的库**

在实现猜数字游戏时，我们需要引入以下几个标准库：

* **`std::io`**：处理用户的输入和输出。
* **`std::cmp::Ordering`**：用于比较用户的输入和目标数字的大小。
* **`rand`**：生成随机数的第三方库。

以下是代码中的库引入部分：

```rust
use std::io; // 用于读取用户输入
use std::cmp::Ordering; // 用于比较猜测结果
use rand::Rng; // 用于生成随机数
```

**2. 编写游戏逻辑**

接下来，我们将编写游戏的核心逻辑。首先，生成一个随机数作为目标数字，然后进入一个循环，让玩家不断猜测，直到猜中为止。

```rust
fn main() {
    println!("猜一个介于 1 和 100 之间的数字!");

    // 生成一个随机数，范围是1到100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入你的猜测:");

        // 定义一个可变的字符串变量，用于存储用户的输入
        let mut guess = String::new();

        // 读取用户输入并存储在guess中
        io::stdin()
            .read_line(&mut guess)
            .expect("无法读取行");

        // 尝试将用户输入转换为u32类型，如果失败则提示用户重新输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字!");
                continue;
            }
        };

        // 比较用户的输入和目标数字
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了!"),
            Ordering::Greater => println!("太大了!"),
            Ordering::Equal => {
                println!("你猜对了!");
                break;
            }
        }
    }
}
```

**3. 运行和测试项目**

编写完代码后，你可以通过以下命令运行游戏：

```bash
cargo run
```

这会编译并运行程序，你可以在终端中玩这个猜数字游戏，程序会根据你的输入给出提示，直到你猜中目标数字。

#### **总结**

通过这个简单的命令行游戏示例，我们熟悉了 Rust 中的几个常用标准库，包括处理用户输入输出的 `std::io`，用于比较的 `std::cmp::Ordering`，以及生成随机数的 `rand` 库。这些库在 Rust 编程中非常实用，为进一步开发更复杂的应用程序打下了基础。
