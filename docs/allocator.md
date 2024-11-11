## Блочный аллокатор

### Арена

An arena is essentially a way to group up allocations that are expected to have the same lifetime.

With arenas, it’s easy enough to allocate an arena, fill it up during each frame and **clear it out once the frame is over**. This has additional benefits of cache locality: you can ensure that most of the per-frame objects (which are likely used more often than other objects) are usually in cache during the frame, since they’ve been allocated adjacently.

### Ссылки

* [Arena-based memory management](https://en.wikipedia.org/wiki/Region-based_memory_management)
* [Arenas overview in Rust](https://manishearth.github.io/blog/2021/03/15/arenas-in-rust)
* [Jemalloc](https://github.com/jemalloc/jemalloc)

## Дополнительные Ссылки

* [Курс по concurrency](https://gitlab.com/Lipovsky/concurrency-course)
* [Курс по ОСям на Rust](https://cs140e.sergio.bz/)
