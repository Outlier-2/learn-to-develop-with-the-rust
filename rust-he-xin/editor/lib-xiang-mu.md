---
icon: server
---

# lib 项目

Rust 是一个现代化的系统编程语言，拥有强大的工具链和社区支持。创建一个 `lib` 项目（即库）可以使你的代码被其他项目重用。Rust 的包管理工具 `Cargo` 提供了简洁的命令来初始化、编译和发布库。

**1. 初始化项目**

首先，我们需要使用 `cargo` 命令初始化一个新的库项目。

```sh
cargo new my_library --lib
```

这将创建一个名为 `my_library` 的目录，内部包含基础项目结构：

```css
my_library
├── Cargo.toml
└── src
    └── lib.rs
```

* `Cargo.toml`：项目的配置文件，包含项目名称、版本、依赖项等信息。
* `src/lib.rs`：库的入口文件，所有库的公共 API 从这里开始定义。

**2. 编辑 `Cargo.toml`**

`Cargo.toml` 是 Rust 项目的核心配置文件。在这里，你可以定义项目的元数据、依赖项、特性（features）以及其他重要配置。

```toml
[package]
name = "my_library"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
description = "A brief description of my library"
license = "MIT OR Apache-2.0"
repository = "https://github.com/your_username/my_library"
documentation = "https://docs.rs/my_library"
```

* **`edition`**：Rust 的语言版本，建议使用最新的 `2021` 版。
* **`license`**：项目的许可证类型。可以使用 `MIT`、`Apache-2.0` 或其他符合开源要求的许可证。
* **`repository`** 和 **`documentation`**：填写项目的 GitHub 地址和在线文档地址。

**3. 编写库代码**

在 `src/lib.rs` 文件中定义你的库的主要功能。你可以从定义模块开始，将不同的功能分组到独立的模块中，以保持代码的组织性和可读性。

````rust
pub mod math_utils {
    /// Adds two numbers together.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sum = my_library::math_utils::add(2, 3);
    /// assert_eq!(sum, 5);
    /// ```
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Subtracts the second number from the first.
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
````

* **模块化**：使用 `mod` 关键字将功能分组到模块中，使得代码更具结构性和可维护性。
* **文档注释**：通过 `///` 注释为每个公开函数编写文档，并附上代码示例，便于用户理解和使用。

**4. 测试你的库**

测试是确保库质量的重要环节。Rust 提供了内置的测试框架，可以在同一个项目中编写测试代码。

在 `src/lib.rs` 文件中添加测试模块：

```rust
#[cfg(test)]
mod tests {
    use super::math_utils;

    #[test]
    fn test_add() {
        assert_eq!(math_utils::add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(math_utils::subtract(5, 2), 3);
    }
}
```

* **测试模块**：使用 `#[cfg(test)]` 标记测试模块，`#[test]` 标记单个测试函数。
* **运行测试**：使用 `cargo test` 命令运行所有测试。

```sh
cargo test
```

**5. 添加 `features`**

在库中使用 `features` 可以为用户提供额外的功能，而不强制启用所有功能。可以在 `Cargo.toml` 中定义 `features`：

```toml
[features]
default = ["serde_support"]
serde_support = ["serde"]

[dependencies]
serde = { version = "1.0", optional = true }
```

然后在代码中使用 `cfg` 属性来根据 `feature` 启用或禁用特定功能：

```rust
#[cfg(feature = "serde_support")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
pub struct MyStruct {
    pub id: u32,
    pub name: String,
}
```

**6. 构建和文档生成**

为了确保库可以正常构建并生成文档，使用以下命令：

* **构建库**：`cargo build` 会生成 `target/debug` 文件夹，里面包含编译好的库。

```sh
cargo build
```

* **生成文档**：`cargo doc --open` 生成并打开库的文档。

```sh
cargo doc --open
```

**7. 发布你的库**

在你确认库的功能、测试和文档都准备就绪后，可以将库发布到 `crates.io`，这是 Rust 的包管理平台。

```sh
cargo publish
```

确保你在 `Cargo.toml` 中正确配置了所有元数据，特别是 `version`、`license` 和 `repository` 信息。

**8. 版本管理**

发布后，你可能需要更新和维护库的版本。请遵循语义化版本号（Semantic Versioning）的规则：

* **主版本号**：有重大变化且不兼容之前版本时更新。
* **次版本号**：增加新功能且向后兼容时更新。
* **修订版本号**：修复 bug 且向后兼容时更新。

每次发布新版本时，确保在 `Cargo.toml` 中更新 `version` 字段，并为新功能或变化添加详细的文档说明
