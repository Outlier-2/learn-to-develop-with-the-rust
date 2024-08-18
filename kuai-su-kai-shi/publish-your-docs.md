---
description: 描述rust对应的生态如包管理cargo，rustup，rustc等工具
---

# 🌴 生态简介

## Rustup

`rustup` 是一个 Rust 工具链管理器，用于安装和管理多个 Rust 版本、工具链以及组件。它提供了一种便捷的方式来切换不同版本的 Rust 编译器，以便开发者能够在项目中使用特定版本的 Rust 或者尝试新版本的特性。

{% code title="bash 下载命令" %}
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
{% endcode %}

**命令行接口**：`rustup` 提供了丰富的命令行接口，用户可以通过简单的命令进行工具链的安装、更新、切换以及组件管理。例如：

* `rustup install stable`：安装稳定版的 Rust 工具链。
* `rustup update`：更新所有已安装的工具链到最新版本。
* `rustup default nightly`：将默认工具链切换为夜间版。

## Rustc

`rustc` 是 Rust 编程语言的编译器，它是 Rust 生态系统的核心工具，负责将 Rust 源代码编译成可执行的二进制文件。`rustc` 可以直接使用命令行编译单个 Rust 文件或更复杂的项目，但通常通过 `Cargo` 来间接调用 `rustc`，以便更方便地管理依赖和构建过程。

#### 常用命令选项

以下是一些常用的 `rustc` 命令行选项：

* **`rustc [filename].rs`**：编译单个 Rust 源文件，生成一个可执行文件（默认名为 `filename`，无扩展名）。
* **`rustc [filename].rs -o output_name`**：编译并生成一个指定名称的可执行文件。
* **`rustc [filename].rs --crate-type=lib`**：编译生成一个库（`libfilename.rlib`），可以被其他 Rust 项目使用。
* **`rustc [filename].rs -O`**：编译时进行优化，生成优化后的可执行文件，适用于发布构建。
* **`rustc [filename].rs -g`**：编译时生成调试信息，适用于调试构建。
* **`rustc [filename].rs --emit=llvm-ir`**：编译生成 LLVM IR 文件（中间表示），用于低级优化和分析。
* **`rustc --explain E0382`**：查看 Rust 编译错误代码的详细解释（例如 `E0382` 是一个错误代码）。
* **`rustc --version`**：查看当前安装的 Rust 编译器版本信息。

## Cargo

`Cargo` 是 Rust 编程语言的包管理工具和构建系统，它简化了 Rust 项目的构建、依赖管理、测试和发布过程。作为 Rust 生态系统的重要组成部分，`Cargo` 帮助开发者更高效地管理项目，从开发到部署。

| **命令**                         | **功能说明**                                                 |
| ------------------------------ | -------------------------------------------------------- |
| `cargo new [project_name]`     | 创建一个新的 Rust 项目，生成标准的目录结构和 `Cargo.toml` 文件。               |
| `cargo init`                   | 在现有目录下初始化 Rust 项目，生成 `Cargo.toml` 文件。                    |
| `cargo build`                  | 编译项目，生成二进制文件（默认在 `target/debug/` 目录下）。                   |
| `cargo run`                    | 编译并运行项目，相当于 `cargo build` 后执行生成的二进制文件。                   |
| `cargo clean`                  | 清理项目生成的中间文件和构建输出，通常用于释放磁盘空间或解决构建问题。                      |
| `cargo update`                 | 更新 `Cargo.lock` 文件中的依赖版本为最新兼容版本（根据 `Cargo.toml` 中的版本要求）。 |
| `cargo check`                  | 快速检查代码是否能够成功编译，但不生成二进制文件。                                |
| `cargo test`                   | 运行项目中的所有测试函数，包括单元测试和集成测试。                                |
| `cargo bench`                  | 运行基准测试（benchmark），通常用于性能测试。                              |
| `cargo doc`                    | 生成项目的文档，文档文件生成在 `target/doc/` 目录下。                       |
| `cargo doc --open`             | 生成文档后自动在浏览器中打开。                                          |
| `cargo fmt`                    | 格式化项目中的代码，确保代码风格统一（需要安装 `rustfmt` 工具）。                   |
| `cargo build --release`        | 编译项目的发布版本，生成优化后的二进制文件（默认在 `target/release/` 目录下）。        |
| `cargo publish`                | 将当前项目发布到 [crates.io](https://crates.io/)，供其他开发者使用。       |
| `cargo package`                | 将项目打包成 `.crate` 文件，但不会发布到 crates.io，适合本地或内部发布。           |
| `cargo add [crate_name]`       | 添加一个新的依赖库到 `Cargo.toml` 中（需要安装 `cargo-edit` 插件）。         |
| `cargo remove [crate_name]`    | 从 `Cargo.toml` 中移除指定的依赖库（需要安装 `cargo-edit` 插件）。          |
| `cargo upgrade`                | 将 `Cargo.toml` 中的依赖更新为最新版本（需要安装 `cargo-edit` 插件）。        |
| `cargo workspace`              | 管理多项目工作空间，允许多个项目共享同一个 `Cargo.toml` 文件。                   |
| `cargo install [crate_name]`   | 安装一个 crate 的可执行工具，通常从 crates.io 安装。                      |
| `cargo uninstall [crate_name]` | 卸载通过 `cargo install` 安装的工具。                              |
| `cargo fix`                    | 自动修复编译器指出的简单问题，如过时的语法或警告。                                |

