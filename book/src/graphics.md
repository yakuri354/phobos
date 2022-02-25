# Graphics

There are lots of ways to display graphics, each with its tradeoffs. While VESA is a common approach among hobby OSes due to its simplicity, it is deprecated by all major GPU vendors. A GPU driver is the most featureful approach, but its extreme complexity makes it impractical for anyone but major commercial OS developers.

### phobos uses GOP to render graphics
GOP is a relatively new standard. It is a part of the UEFI specification and functions similarly to VESA. The OS gets a framebuffer from the firmware, which the OS can use to draw anything on the display. Unfortunately, the GOP also comes with some serious drawbacks that make its usage impossible in complex scenarios.

GOP provides a lot of useful features, but they are all only accessible in UEFI Boot Services mode. The only thing that the OS can use is its framebuffer.

One of the most serious disadvantages of GOP is lack of BitBlt and modesetting in Runtime Serives. 

### Advantages:
- Simple to use
- Blitting and modesetting in Boot Services
- Modern and standardized
### Disadvantages:
- No modesetting in Runtime Services
- No blitting in Runtime Services

#### Also see:
- [GOP](https://wiki.osdev.org/GOP)
- [Drawing in a framebuffer](https://wiki.osdev.org/Drawing_In_a_Linear_Framebuffer)