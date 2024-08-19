---
icon: magnifying-glass-arrows-rotate
---

# 内部可变

在 Rust 中，所有权和借用规则通过编译时检查确保了数据的不可变性和线程安全性。然而，有时我们希望在一个不可变的上下文中对数据进行修改，这就是内部可变性（Interior Mutability）概念的来源。

内部可变性允许我们在遵守 Rust 的借用规则的同时，修改数据。这种能力通常通过 `Cell<T>` 和 `RefCell<T>` 类型来实现，它们提供了一种在不可变借用中进行可变操作的机制。

**1. 内部可变性与外部可变性的区别**

* **外部可变性**：通过可变引用（`&mut`）来修改数据，通常要求借用方持有数据的唯一可变引用。
* **内部可变性**：允许在不可变引用（`&`）的情况下修改数据，通常借助于特定的类型和方法来实现。

**2. `Cell<T>` 与 `RefCell<T>`**

`Cell<T>` 和 `RefCell<T>` 是 Rust 提供的两种内部可变性类型，它们的主要区别在于处理单线程与多线程的方式。

**2.1 `Cell<T>`**

`Cell<T>` 适用于存储类型实现了 `Copy` trait 的值。`Cell<T>` 允许在共享引用下读写数据，但因为 `Cell<T>` 的操作不涉及引用，所以它不能返回内部数据的引用。

```rust
use std::cell::Cell;

fn main() {
    let data = Cell::new(5);
    data.set(10);
    println!("data: {}", data.get());
}
```

在这个例子中，我们在一个不可变的上下文中使用 `Cell<T>` 修改了数据。

**2.2 `RefCell<T>`**

`RefCell<T>` 更加强大，允许在运行时执行借用检查，而不是编译时。`RefCell<T>` 允许我们在不可变引用的情况下对数据进行可变操作，并且支持返回数据的引用。

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(vec![1, 2, 3]);

    {
        let mut data_mut = data.borrow_mut();
        data_mut.push(4);
    }

    println!("{:?}", data.borrow());
}
```

在这个例子中，我们使用 `RefCell<T>` 实现了一个内部可变的 `Vec`，并且能够在不可变上下文中修改它。

**3. 内部可变性与线程安全**

需要注意的是，`Cell<T>` 和 `RefCell<T>` 只能用于单线程环境中。如果需要在线程间共享数据，同时保持内部可变性，应该使用 `Arc<Mutex<T>>` 或 `Arc<RwLock<T>>`。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(5));

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut num = data_clone.lock().unwrap();
        *num += 10;
    });

    handle.join().unwrap();
    println!("data: {:?}", *data.lock().unwrap());
}
```

在这个例子中，我们使用 `Arc` 和 `Mutex` 实现了线程安全的内部可变性。

**4. `Rc<RefCell<T>>` 与 `Arc<Mutex<T>>`**

`Rc<RefCell<T>>` 和 `Arc<Mutex<T>>` 是 Rust 中非常常见的组合，分别用于单线程和多线程环境下的内部可变性共享。

* **`Rc<RefCell<T>>`**：用于单线程环境，提供引用计数和内部可变性。
* **`Arc<Mutex<T>>`**：用于多线程环境，提供原子引用计数和线程安全的内部可变性。

**4.1 `Rc<RefCell<T>>` 示例**

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let data = Rc::new(RefCell::new(5));

    let data_clone = Rc::clone(&data);
    *data_clone.borrow_mut() += 1;

    println!("data: {:?}", data.borrow());
}
```

在这个例子中，`Rc<RefCell<T>>` 允许多个所有者共享和修改数据。

**4.2 `Arc<Mutex<T>>` 示例**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(5));

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut num = data_clone.lock().unwrap();
        *num += 1;
    });

    handle.join().unwrap();
    println!("data: {:?}", *data.lock().unwrap());
}
```

在这个例子中，`Arc<Mutex<T>>` 确保了多线程环境下的安全数据共享和修改。

#### 结论

Rust 的内部可变性是一种强大且灵活的机制，它允许开发者在不违反所有权和借用规则的情况下修改数据。通过 `Cell<T>` 和 `RefCell<T>`，开发者可以在不可变引用的上下文中实现可变操作，而 `Rc<RefCell<T>>` 和 `Arc<Mutex<T>>` 则提供了更加复杂的共享和同步策略。这些工具不仅确保了线程安全和数据完整性，还使得 Rust 能够轻松处理复杂的并发任务。
