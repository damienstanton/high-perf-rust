# Notes

## Chapter 1

- `debug_assert!` for dev-only conditional checks
- `lto`, slow compilation but smaller final bin:

```toml
[profile.release]
lto = true

```

- `codegen-units` to match num threads to compile chunks

```toml
[profile.dev]
codegen-units = 32
```

- `panic = 'abort'` for extra (and dangerous) optimization to bail out without destructors

> three things with variables when passing them to a function: send a reference (borrow), give the new function control of the variable (own), or copy/clone the variable to send it to a function...
> ...The rule of thumb is, if it's smaller than or equal to usize, copy, always. If it's somewhere between usize and 10 times that size, it's probably better to copy. If it's bigger, it's probably better to reference. If the structure has a heap allocation (such as a Vec or a Box), it's usually better to send a reference.

- Explore `itertools` crate.

© 2018 Damien Stanton

See LICENSE for details.

[![Buy Me A Coffee](https://www.buymeacoffee.com/assets/img/custom_images/white_img.png)](https://www.buymeacoffee.com/damienstanton)
