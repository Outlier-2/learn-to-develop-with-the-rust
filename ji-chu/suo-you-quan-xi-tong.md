# 💁‍♂️ 所有权系统

所有权（Ownership）是 Rust 编程语言中一组用于管理内存的规则。通过这些规则，Rust 可以在没有垃圾收集（Garbage Collection）的情况下，确保内存的安全和高效管理。这些规则在编译期间强制执行，如果违反了任何规则，程序将无法通过编译。



{% embed url="https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#what-is-ownership" %}
Ownership
{% endembed %}

所有权的基本规则

1. **每个值都有一个所有者**：Rust 中的每一个数据都有一个变量作为其所有者。
2. **一个值在同一时间只能有一个所有者**：当一个变量拥有某个值时，不能再有其他变量同时拥有该值。
3. **当所有者超出作用域时，值会被自动删除**：当一个变量的作用域结束时，它所拥有的内存会被自动释放，这意味着不再需要手动释放内存。



#### 栈（Stack）和堆（Heap）

在了解所有权之前，理解栈和堆的概念是必要的。

* **栈**：栈是一种后进先出（LIFO）的数据结构。它的特点是数据在栈上按顺序存储，删除数据的顺序与存储顺序相反。栈上存储的数据必须具有固定大小，这使得栈非常高效，但不适用于需要动态内存分配的情况。
* **堆**：堆的结构更复杂，可以存储大小不确定的数据。当需要存储数据时，程序会向堆申请内存空间。堆的分配速度比栈慢，因为需要查找合适的内存块，并且访问堆数据需要通过指针。

#### 所有权在实践中的应用

**变量的作用域**

变量的作用域是指变量在程序中有效的范围。比如，当声明一个变量 `let s = "hello";` 时，该变量在其所在的花括号内有效，一旦超出作用域，Rust 会自动清理它。

**`String` 类型与内存管理**

Rust 中的 `String` 类型是动态的，存储在堆上，适用于在编译时无法确定大小的字符串。与其他编程语言不同，Rust 不需要程序员手动释放 `String` 类型的内存。当一个 `String` 变量超出其作用域时，Rust 会自动调用 `drop` 函数来释放它的内存。

**移动（Move）**

当将一个变量赋值给另一个变量时，比如 `let s2 = s1;`，Rust 并不会复制该值，而是将所有权从 `s1` 转移到 `s2`。这意味着 `s1` 将不再有效，如果尝试使用它，编译器会报错。这种机制被称为“移动”（Move），它可以避免数据的重复释放问题。

**克隆（Clone）**

如果你需要保留原有变量，同时创建一个新变量，可以使用 `clone` 方法。比如 `let s2 = s1.clone();`，这会深度复制 `s1` 的数据，使 `s1` 和 `s2` 都拥有各自独立的数据副本。

#### 总结

所有权是 Rust 的核心概念，它确保了内存的安全管理，而不需要像 C++ 一样手动释放内存，也不需要依赖垃圾收集器。通过理解所有权规则、栈与堆的区别，以及变量如何在移动和克隆中表现，你将能够编写更加高效、安全的 Rust 代码。