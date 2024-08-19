---
icon: battle-net
---

# 常用数据结构

* Stack
* List
* Map
* BTree
* Heap

Rust 提供了多种内置数据结构来满足不同场景的需求。了解并掌握这些数据结构可以帮助我们编写更加高效和易于维护的代码。下面我们将详细介绍 Rust 中常用的数据结构，包括数组、元组、向量、哈希表、链表、队列、栈、双端队列等。

**1. 数组 (Array)**

数组是一种固定长度的、同构类型的集合。数组中的元素在内存中是连续存储的，因此可以通过索引快速访问。

```rust
let arr = [1, 2, 3, 4, 5]; // 长度为5的数组
let first = arr[0]; // 访问第一个元素
```

* **特点**：数组长度在编译时确定且不可变。适合存储固定数量的元素。
* **使用场景**：适用于数据量固定且不需要动态增加或减少的场景。

**2. 元组 (Tuple)**

元组是一种可以包含多种不同类型元素的集合。元组的长度是固定的，但可以包含不同类型的数据。

```rust
let tuple = (1, "hello", 3.14);
let (x, y, z) = tuple; // 解构元组
let second = tuple.1; // 访问元组的第二个元素
```

* **特点**：元组的长度和元素类型是固定的，但可以混合多种类型。
* **使用场景**：适用于需要将多种不同类型的数据组合在一起的场景。

**3. 向量 (Vector)**

向量是一种动态数组，可以在运行时动态增长。向量的所有元素类型必须相同。

```rust
let mut vec = vec![1, 2, 3];
vec.push(4); // 向量末尾添加元素
let third = vec[2]; // 访问向量的第三个元素
```

* **特点**：向量的长度在运行时可变，支持动态增长。
* **使用场景**：适用于需要动态添加或删除元素的场景。

**4. 哈希表 (HashMap)**

哈希表是一种键值对存储的数据结构，支持快速查找。Rust 提供了 `HashMap<K, V>` 类型来实现哈希表。

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("key1", 10);
map.insert("key2", 20);
let value = map.get("key1");
```

* **特点**：通过键值对存储数据，查找操作的时间复杂度为 O(1)。
* **使用场景**：适用于需要快速查找、插入、删除操作的场景。

**5. 链表 (LinkedList)**

链表是一种动态数据结构，由节点构成，每个节点包含数据和指向下一个节点的指针。Rust 提供了 `LinkedList<T>` 来实现链表。

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();
list.push_back(1);
list.push_back(2);
let front = list.pop_front();
```

* **特点**：链表支持 O(1) 的插入和删除操作，但随机访问元素的效率较低（O(n)）。
* **使用场景**：适用于频繁插入和删除操作，但不需要随机访问元素的场景。

**6. 队列 (Queue)**

队列是一种先进先出（FIFO，First In First Out）的数据结构，常用于任务调度、缓冲区等场景。Rust 中没有直接提供队列的标准库实现，但可以使用 `VecDeque<T>` 实现队列。

```rust
rust复制代码use std::collections::VecDeque;

let mut queue = VecDeque::new();
queue.push_back(1); // 入队
queue.push_back(2);
let front = queue.pop_front(); // 出队
```

* **特点**：队列的插入操作发生在队尾，删除操作发生在队首。`VecDeque` 提供了 O(1) 的插入和删除操作。
* **使用场景**：适用于需要按顺序处理任务、消息或数据的场景。

**7. 栈 (Stack)**

栈是一种后进先出（LIFO，Last In First Out）的数据结构，常用于递归调用、表达式求值、回溯算法等场景。Rust 中可以使用 `Vec<T>` 作为栈的实现。

```rust
rust复制代码let mut stack = Vec::new();
stack.push(1); // 压栈
stack.push(2);
let top = stack.pop(); // 弹栈
```

* **特点**：栈的插入和删除操作都发生在栈顶，具有 O(1) 的时间复杂度。
* **使用场景**：适用于需要在递归、解析表达式或实现回溯算法的场景。

**8. 双端队列 (Deque)**

双端队列是一种可以在两端进行插入和删除操作的队列。在 Rust 中，`VecDeque<T>` 是双端队列的标准实现。

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_front(1); // 从前端插入
deque.push_back(2);  // 从后端插入
let front = deque.pop_front(); // 从前端删除
let back = deque.pop_back(); // 从后端删除
```

* **特点**：双端队列可以在队列的两端进行高效的插入和删除操作，提供了比普通队列更大的灵活性。
* **使用场景**：适用于需要双向操作的场景，如双端缓冲区、滑动窗口算法等。

**9. 哈希集合 (HashSet)**

哈希集合是一种不允许重复元素的无序集合。前面提到过 `HashSet<T>`，它基于哈希表实现，适用于快速查找和去重。

```rust
use std::collections::HashSet;

let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(1); // 重复元素将被忽略
```

* **特点**：集合不允许重复元素，且查找操作的时间复杂度为 O(1)。
* **使用场景**：适用于需要快速查找唯一元素的场景，如去重、集合运算等。

**10. 二叉堆 (Binary Heap)**

二叉堆是一种特殊的二叉树结构，主要用于实现优先队列。前面已经介绍过 `BinaryHeap<T>`。

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(5);
heap.push(1);
let max = heap.pop(); // 获取并移除最大元素
```

* **特点**：二叉堆支持快速的插入和删除操作，适合用于实现优先级队列。
* **使用场景**：用于任务调度、排序等需要快速获取最小或最大值的场景。

**11. BTreeMap 和 BTreeSet**

除了哈希表（HashMap、HashSet），Rust 还提供了基于二叉树的映射（BTreeMap）和集合（BTreeSet），它们内部使用了自平衡的二叉树。

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "c");
map.insert(1, "a");
map.insert(2, "b");

for (key, value) in &map {
    println!("{}: {}", key, value);
}
```

* **特点**：`BTreeMap` 和 `BTreeSet` 保持了元素的有序性，适合需要按顺序存储和查找数据的场景。
* **使用场景**：适用于需要按顺序访问数据的场景，如范围查询、排序输出等。

#### 总结

Rust 提供了丰富的数据结构来满足各种开发需求。了解这些数据结构的特点和适用场景，可以帮助开发者选择合适的数据结构来解决实际问题。无论是固定长度的数组，还是动态可变的向量和哈希表，Rust 都提供了高效且安全的实现。同时，理解队列、栈和双端队列等基础数据结构的特性，也能帮助你在具体问题中找到最佳解决方案。掌握这些数据结构及其应用，将显著提高你的 Rust 编程技能。
