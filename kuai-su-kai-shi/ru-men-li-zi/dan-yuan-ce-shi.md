---
icon: calendar-exclamation
---

# 单元测试

### 1. 单元测试的基础

#### 1.1 测试模块 (`#[cfg(test)]`)

Rust 中的单元测试通常放在源文件的底部，并放在一个 `#[cfg(test)]` 属性标记的模块中。这个模块中的测试代码只在执行测试时编译，而不会在生产环境中包含。

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }
}
```

#### 1.2 `#[test]` 属性

`#[test]` 属性用于标记一个函数为测试函数。当执行 `cargo test` 时，所有带有 `#[test]` 标记的函数都会被运行。

#### 1.3 使用 `assert!`、`assert_eq!` 和 `assert_ne!`

Rust 提供了一些常用的断言宏来帮助我们验证测试结果：

* `assert!(condition)`: 如果 `condition` 为 `false`，测试失败。
* `assert_eq!(left, right)`: 如果 `left` 不等于 `right`，测试失败。
* `assert_ne!(left, right)`: 如果 `left` 等于 `right`，测试失败。

```rust
#[test]
fn test_multiplication() {
    assert_eq!(3 * 3, 9);
    assert_ne!(3 * 3, 10);
    assert!(3 * 3 == 9);
}
```

### 2. 测试结构与组织

#### 2.1 测试模块的结构

单元测试通常与被测试的代码放在同一个文件中，但也可以将测试代码放在 `tests` 目录下的独立文件中进行集成测试。如下是一个典型的测试模块结构：

```rust
// src/lib.rs

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

#### 2.2 集成测试

Rust 允许将测试代码放在 `tests` 目录下进行集成测试。这些测试是完全独立的，类似于外部使用者使用你的库来进行测试。

```rust
// tests/integration_test.rs

use my_crate::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

集成测试与单元测试的主要区别在于，集成测试不会自动导入当前包的内容，需要手动导入要测试的函数或模块。

### 3. 测试用例的管理

#### 3.1 运行测试

使用 `cargo test` 命令可以运行所有测试，包括单元测试和集成测试。`cargo test` 会执行如下步骤：

1. 编译代码和测试代码。
2. 运行测试并报告结果。

```sh
cargo test
```

#### 3.2 过滤测试

如果你只想运行某些特定的测试，可以使用过滤功能。例如，运行名字中包含 `add` 的测试：

```sh
cargo test add
```

#### 3.3 忽略某些测试

有时，你可能希望暂时忽略某些测试。这可以通过 `#[ignore]` 属性来实现。

```rust
#[test]
#[ignore]
fn test_that_is_ignored() {
    assert_eq!(1 + 1, 2);
}
```

要运行被忽略的测试，可以使用 `cargo test -- --ignored` 命令。

#### 3.4 设定测试行为

你可以通过传递额外的参数来改变测试的行为，例如启用并发测试或限制输出。

```sh
cargo test -- --nocapture
```

`--nocapture` 选项会显示测试期间的所有输出，而不仅仅是测试失败时的输出。

### 4. 测试中的 `Result` 和 `panic!`

#### 4.1 使用 `Result` 进行测试

测试函数不仅可以返回 `()` 类型，也可以返回 `Result<T, E>`。这种方式允许你在测试中使用 `?` 运算符进行更复杂的错误处理。

```rust
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("Math seems broken"))
    }
}
```

#### 4.2 测试 `panic!`

如果你想测试某个函数是否会在特定条件下触发 `panic!`，可以使用 `should_panic` 属性。

```rust
#[test]
#[should_panic(expected = "division by zero")]
fn test_divide_by_zero() {
    divide(1, 0);
}
```

`expected` 参数用于指定触发 `panic!` 时的错误信息。如果触发 `panic!` 时的信息与 `expected` 不符，测试会失败。

### 5. 代码覆盖率与持续集成

#### 5.1 代码覆盖率

Rust 可以与代码覆盖率工具集成，如 `tarpaulin`，以查看你的测试覆盖了多少代码。

```sh
cargo install cargo-tarpaulin
cargo tarpaulin
```

`cargo-tarpaulin` 可以生成代码覆盖率报告，帮助你识别哪些部分的代码尚未被测试覆盖。

#### 5.2 持续集成 (CI)

Rust 项目通常会使用持续集成（CI）工具来自动化测试过程。在 CI 管道中，可以通过简单的命令 `cargo test` 来确保每次提交的代码都通过所有测试。

以下是一个基于 GitHub Actions 的示例配置：

```yaml
name: Rust CI

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v2
    - name: Install Rust
      run: rustup update stable
    - name: Run tests
      run: cargo test
```

### 6. 结论

Rust 提供了强大的单元测试功能，集成在语言工具链中，使得编写、运行和管理测试变得非常方便。通过本文介绍的单元测试基础、结构与组织、测试用例管理、`Result` 和 `panic!` 的使用，以及代码覆盖率与持续集成，你可以更好地确保 Rust 项目的代码质量。
