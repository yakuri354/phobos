# Font rendering

One of the main goals of phobos is to focus on modern practices. While using Linux, I've noticed that the traditional 8x8 bitmap fonts frequently used for debug output are very hard to read, especially on screens with high DPI. I've decided to use modern anti-aliased fonts for debug output in phobos

The first text rendering system used `fontdue` crate to rasterize glyphs on the fly.

Font rasterization has proven to be ***highly inefficient*** due to its heavy reliance on floating point operations, which are currently simulated using LLVM's `soft-float` flag (because the FPU cannot be used in kernel code). It has led to inacceptably slow boots.

The final approach was to pre-render a font into bitmaps of several sizes and include them into the binary. I've used crate `noto-mono-bitmaps` which includes font Noto Mono rasterized with grayscale antialiasing.

### Advantages:
- Almost free in terms of performance
- Extremely complex TTF parsing and rasterizing code is avoided
- Easy to implement
### Disadvantages:
- Only a fixed set of font sizes can possibly be supported
- Requires a lot of space and bloats the binary
- Rasterizing parameters cannot be changed in runtime

#### Also see:
- [noto-sans-mono-bitmap crate](https://github.com/phip1611/noto-sans-mono-bitmap-rs)