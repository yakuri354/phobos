# Physical memory allocation

The physical frames are stored in a stack-like data structure implemented using linked lists.

All frames are linked to each other like this:
```
Top: 0x1000

0x1000:                0x42000:              0x3000:
|---------------|   -> |--------------|   -> |------------|
| frame #1      |   |  | frame #2     |   |  | frame #3   |
| next: 0x42000 | --|  | next: 0x3000 | --|  | next: None |
|---------------|      |--------------|      |------------|
```

Frames can be popped from or pushed onto the stack in constant time, which leads to O(1) allocation and deallocation

### Advantages:
- O(1) alloc & free
- Easy to understand and implement
- Zero space overhead
### Disadvantages:
- Practically impossible to allocate adjacent blocks
- Initialization time is O(N) to memory size
- Requires all physical memory to be mapped (barely possible on 32 bit architectures)

#### Also see:
- [Alternative approaches to frame allocation](https://wiki.osdev.org/Page_Frame_Allocation)
- [Linux frame allocator](https://www.kernel.org/doc/gorman/html/understand/understand009.html)