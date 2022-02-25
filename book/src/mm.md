# Memory management
Phobos has 3 layers of memory management:
- Physical
  - Manages physical frames
- Virtual
  - Manages virtual address space and mapings to physical frames
- kmalloc
  - Manages small virtual memory allocations (less than a page in size)

#### See also:
- [Memory management](https://wiki.osdev.org/Memory_management)
- [Intel manual (see Paging chapter)](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)