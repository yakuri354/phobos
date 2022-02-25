# Cooperative multitasking

phobos supports cooperative multitasking via Rust's built in async infrastructure.

Tasks are spawned into an executor using method `spawn`.

```rust,ignore
executor.spawn(Task::new(async_fn()))
```

After calling `executor.run()` the executor polls the tasks indefinetely.

Executor is implemented here: `kernel/task/executor.rs`.

Task is an individual unit of work performed by the OS. Tasks are wrapped and boxed futures with unique IDs. Tasks are implemented in file `kernel/task/mod.rs`.

### Advantages:
- No need to switch tasks, thus easier to implement
- More performant than traditional cooperative multitasking
- High level abstractions lead to understandable code
### Disadvantages:
- All tasks must run code in the kernel crate
- A faulty task can lock up the whole system

#### Also see:
- [Async book](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html)
- [Multitasking models](https://wiki.osdev.org/Multitasking_Systems)