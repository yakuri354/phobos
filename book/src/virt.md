# Virtual memory allocation

There are 2 virtual memory allocators implemented in phobos

## Bump allocator
The first is simply a bump allocator which simply keeps track of the highest allocated address. It takes advantage of the extremely large x64 virtual address space

```
|0x0         |last                      |highest virtual address
|------------|--------------------------|
|    Used    |          Free            |
|------------|--------------------------|
```

### Advantages:
- Extremely easy to implement
- O(1) allocation, O(1) initialization
### Disadvantages:
- Cannot free memory which leads to eventual exhaustion
- Basically unusable in real-world scenarios

## VAD Tree

The second allocator is a red-black binary tree containing address ranges, also referred to as a VAD tree or an interval tree. It is not used due to its advantages being irrelevant at current stage of development. This algorithm is used in most major OSes, i.e. NT

### Advantages:
- Page fault resolution in O(log N) time. Very useful for swapping and MMIO
- O(1) initialization
### Disadvantages:
- Hard to implement
- O(N) allocation

#### Also see:
- [VAD tree in NT](https://www.sciencedirect.com/science/article/pii/S1742287607000503)