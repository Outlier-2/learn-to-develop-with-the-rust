# 🧚 面向对象编程

### 1. 结构体（Struct）与枚举（Enum）

#### 1.1 结构体（Struct）

结构体是Rust中定义复杂数据类型的基本方式。它们类似于其他语言中的类，用于组合相关的数据字段。

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 10, y: 20 };
```

#### 1.2 枚举（Enum）

枚举允许定义一个类型，该类型可以是几种不同类型之一。每种可能的类型称为变体，枚举在Rust中通常用于处理多种状态或选项。

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let direction = Direction::Up;
```

### 2. 方法的实现

#### 2.1 实现`impl`块

在Rust中，通过`impl`块为结构体和枚举定义方法。`impl`块允许将行为附加到数据类型上，类似于OOP中的成员函数。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数，用于构造一个矩形
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // 实例方法，用于计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect = Rectangle::new(30, 50);
let area = rect.area();
```

#### 2.2 方法的类型

Rust中方法分为两类：关联函数和实例方法。

* **关联函数**：类似于静态方法，通过类型调用。通常用作构造函数。
* **实例方法**：与特定实例关联，通过实例调用。需要`&self`参数。

### 3. Trait 与多态

#### 3.1 Trait 的定义与实现

Trait类似于其他语言中的接口，用于定义一组可以在多种类型上实现的行为。Trait使得Rust可以实现多态——不同的类型可以表现出相同的行为。

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
}

let circle = Circle { radius: 5.0 };
let rectangle = Rectangle { width: 3, height: 4 };

println!("Circle area: {}", circle.area());
println!("Rectangle area: {}", rectangle.area());
```

#### 3.2 动态分发与Trait对象

Rust中，通过使用`dyn`关键字，可以创建指向不同实现相同Trait的类型的Trait对象。这允许在运行时根据对象类型调用适当的方法，实现动态分发。

```rust
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 3.0 }),
    Box::new(Rectangle { width: 3, height: 4 }),
];

for shape in shapes {
    println!("Area: {}", shape.area());
}
```

#### 3.3 解析赋值与模式匹配

Rust中可以使用模式匹配（Pattern Matching）进行解析赋值，方便提取结构体或枚举中的字段。

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 10, y: 20 };

let Point { x, y } = point;  // 解析赋值

println!("x: {}, y: {}", x, y);
```

对于枚举，也可以使用类似的方式进行模式匹配和解析赋值：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 5, y: 10 };

match msg {
    Message::Quit => println!("Quit message"),
    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
}
```

### 4. 综合示例

下面的示例展示了如何结合结构体、Trait和`impl`块来创建一个简单的面向对象风格的程序。

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

struct Square {
    side: f64,
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side {}", self.side);
    }
}

let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];

for shape in shapes {
    shape.draw();
}
```

这个程序中，我们定义了一个`Drawable`的Trait，表示一个可以绘制的形状，然后为`Circle`和`Square`结构体实现了这个Trait。通过使用Trait对象，我们可以在一个集合中存储和操作不同类型的形状。
