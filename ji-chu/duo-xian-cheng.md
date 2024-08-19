---
icon: reel
---

# 多线程

Rust 以其独特的所有权系统和类型安全性著称，在多线程编程方面，Rust 提供了多种工具来确保线程安全并避免常见的并发问题。本文将介绍 Rust 中的多线程编程，包括线程的创建与管理、线程间的通信与同步、以及常见的多线程编程模式。

**1. 线程的创建与管理**

Rust 提供了标准库中的 `std::thread` 模块，用于创建和管理线程。创建一个线程非常简单，使用 `thread::spawn` 函数即可。

**基本线程创建示例**

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread: {}", i);
            thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hello from the main thread: {}", i);
        thread::sleep(std::time::Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

在这个例子中，`thread::spawn` 函数创建了一个新线程，该线程运行一个闭包。在主线程中，我们可以使用 `join` 方法等待新线程的完成。

**传递数据给线程**

通过 `move` 关键字，Rust 允许将数据的所有权转移到线程中：

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

这里，`v` 的所有权被移动到了新线程中，因此主线程不能再使用 `v`。

**2. 线程间的通信与同步**

Rust 提供了多种线程间通信的方式，其中最常用的是 `mpsc`（Multiple Producer, Single Consumer）通道和 `Arc`（Atomic Reference Counting）智能指针。

**使用通道进行线程间通信**

`mpsc` 模块提供了通道，用于线程之间的消息传递：

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

在这个例子中，我们创建了一个通道，并在一个线程中发送消息，然后在主线程中接收并处理该消息。

**使用 `Arc` 和 `Mutex` 实现线程安全的共享状态**

当多个线程需要访问和修改同一个数据时，可以使用 `Arc` 和 `Mutex` 来确保线程安全。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

在这个例子中，`Arc` 提供了线程间的共享所有权，而 `Mutex` 确保了对共享数据的安全访问。

**3. 常见的多线程编程模式**

在 Rust 中，多线程编程常见的模式包括以下几种：

**生产者-消费者模式**

生产者-消费者模式通常使用 `mpsc` 通道来实现，其中一个线程（生产者）生成数据并通过通道发送到另一个线程（消费者）进行处理。

**工作线程池**

工作线程池（Thread Pool）是一种用于限制同时运行线程数量的模式。Rust 的 `rayon` 和 `tokio` 等库提供了强大的线程池实现，但也可以手动实现一个简单的线程池。

```rust
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
```

这个简单的线程池示例展示了如何管理一组线程并将任务分配给它们执行。

**4. Rust 的并发模型**

Rust 的并发模型基于所有权系统，确保了线程间的数据竞争和死锁等问题能够在编译时得到解决。这使得 Rust 的多线程编程既高效又安全。

**线程安全性与数据竞争**

Rust 编译器通过所有权和借用系统强制执行线程安全性。即使是复杂的多线程程序，Rust 也能确保它们在编译时就避免了数据竞争问题。

**5. 线程与异步编程**

Rust 提供了强大的异步编程支持，例如 `async` 和 `await` 关键字，以及 `Tokio` 和 `async-std` 等异步运行时。在某些情况下，异步编程比多线程更高效，特别是处理 I/O 密集型任务时。

**简单异步示例**

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 done");
    };

    let task2 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 done");
    };

    tokio::join!(task1, task2);
}
```

这个异步示例展示了如何使用 `Tokio` 运行时并发执行任务。

#### 总结

Rust 中的多线程编程通过所有权和借用系统提供了强大的线程安全保证，避免了常见的并发问题。无论是使用简单的线程，还是复杂的线程池和异步模型，Rust 都能够帮助开发者编写高效、安全的多线程程序。理解并掌握这些概念对于编写可靠的并发代码至关重要。
