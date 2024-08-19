---
description: 标量和复合类型
icon: binary-lock
---

# 数据类型

## 类型分类

在 Rust 中，数据类型分为两大类：**标量类型**（Scalar Types）和**复合类型**（Compound Types）。Rust 是一种静态类型语言，这意味着在编译时必须明确所有变量的类型。

#### 1. 标量类型 (Scalar Types)

标量类型代表单个值，主要包括以下四种：

**a. 整数类型 (Integer Types)**

Rust 支持多种整数类型，按符号和长度划分：

* **有符号整数**（`i8`, `i16`, `i32`, `i64`, `i128`, `isize`）：可以表示负数和正数，`isize` 根据计算机架构的不同而有所变化（32 位或 64 位）。
* **无符号整数**（`u8`, `u16`, `u32`, `u64`, `u128`, `usize`）：只能表示正数和零，`usize` 根据计算机架构的不同而有所变化（32 位或 64 位）。

示例：

```rust
let x: i32 = -42; // 有符号 32 位整数
let y: u64 = 42;  // 无符号 64 位整数
```

**b. 浮点类型 (Floating-Point Types)**

Rust 支持两种浮点类型：

* **`f32`**：32 位浮点数。
* **`f64`**：64 位浮点数（默认类型）。

示例：

```rust
let x: f32 = 3.14; // 32 位浮点数
let y: f64 = 2.718; // 64 位浮点数
```

**c. 布尔类型 (Boolean Type)**

布尔类型用于表示逻辑真值，取值为 `true` 或 `false`。

```rust
let t: bool = true;
let f: bool = false;
```

**d. 字符类型 (Character Type)**

Rust 中的字符类型是 `char`，用于表示单个 Unicode 字符。`char` 类型占用 4 个字节（32 位），能够表示从 `U+0000` 到 `U+10FFFF` 范围内的所有 Unicode 值。

```rust
let c: char = 'A';
let smiley: char = '😊';
```

#### 2. 复合类型 (Compound Types)

复合类型可以组合多个值。Rust 中的主要复合类型包括元组和数组。

**a. 元组 (Tuple)**

元组可以将不同类型的多个值组合在一起，长度固定。可以通过模式匹配解构元组。

示例：

```rust
let tuple: (i32, f64, char) = (42, 3.14, 'R');
let (x, y, z) = tuple;
println!("x: {}, y: {}, z: {}", x, y, z);
```

元组的元素也可以通过索引访问：

```rust
let first = tuple.0;
let second = tuple.1;
```

**b. 数组 (Array)**

数组是同一类型的值的集合，长度固定。数组中的元素可以通过索引访问，从 0 开始。

示例：

```rust
let arr: [i32; 3] = [1, 2, 3];
let first = arr[0];
```

Rust 还提供了切片（Slice），它是对数组的一部分的引用，可以用来动态访问数组的元素。

#### 3. 类型推断与类型转换

Rust 通常能够根据上下文推断变量的类型，因此并不总是需要显式声明类型。但在需要的情况下，可以使用 `as` 关键字进行类型转换。

示例：

```rust
let x = 42; // 推断为 i32
let y = x as f64; // 转换为 f64
```

#### 4. 自定义类型

Rust 允许用户定义自己的数据类型，包括结构体（`struct`）、枚举（`enum`）和联合体（`union`）。

**a. 结构体 (Struct)**

结构体用于定义具有命名字段的复杂数据类型。

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 10, y: 20 };
```

**b. 枚举 (Enum)**

枚举用于定义具有多个可能取值的类型，每个取值称为一个“变体”。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;
```

#### 总结

Rust 提供了丰富的原生数据类型，既有用于存储单一值的标量类型，也有用于组合多个值的复合类型。通过对这些类型的熟练掌握，可以更好地编写高效和安全的 Rust 程序。
