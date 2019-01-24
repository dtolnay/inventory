## Typed distributed plugin registration

[![Build Status](https://api.travis-ci.org/dtolnay/inventory.svg?branch=master)](https://travis-ci.org/dtolnay/inventory)
[![Latest Version](https://img.shields.io/crates/v/inventory.svg)](https://crates.io/crates/inventory)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/inventory)

This crate provides a way to set up a plugin registry into which plugins can be
registered from any source file linked into your application. There does not
need to be a central list of all the plugins.

```toml
[dependencies]
inventory = "0.1"
```

*Supports rustc 1.31+*

<br>

# Examples

Suppose we are writing a command line flags library and want to allow any source
file in the application to register command line flags that are relevant to it.

This is the flag registration style used by [gflags] and is better suited for
large scale development than maintaining a single central list of flags, as the
central list would become an endless source of merge conflicts in an application
developed simultaneously by thousands of developers.

[gflags]: https://gflags.github.io/gflags/

### Instantiating the plugin registry

Let's use a `struct Flag` as the plugin type, which will contain the short name
of the flag like `-v`, the full name like `--verbose`, and maybe other
information like argument type and help text. We instantiate a plugin registry
with an invocation of `inventory::collect!`.

```rust
pub struct Flag {
    short: char,
    name: &'static str,
    /* ... */
}

impl Flag {
    pub fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}

inventory::collect!(Flag);
```

This `collect!` call must be in the same crate that defines the plugin type.
This macro does not "run" anything so place it outside of any function body.

### Registering plugins

Now any crate with access to the `Flag` type can register flags as a plugin.
Plugins can be registered by the same crate that declares the plugin type, or by
any downstream crate.

```rust
inventory::submit! {
    Flag::new('v', "verbose")
}
```

The `submit!` macro does not "run" anything so place it outside of any function
body. In particular, note that all `submit!` invocations across all source files
linked into your application all take effect simultaneously. A `submit!`
invocation is not a statement that needs to be called from `main` in order to
execute.

### Iterating over plugins

The value `inventory::iter::<T>` is an iterator with element type `&'static T`
that iterates over all plugins registered of type `T`.

```rust
for flag in inventory::iter::<Flag> {
    println!("-{}, --{}", flag.short, flag.name);
}
```

There is no guarantee about the order that plugins of the same type are visited
by the iterator. They may be visited in any order.

<br>

## How it works

Inventory is built on the [`ctor`] crate which provides module initialization
functions for Rust similar to `__attribute__((constructor))` in C. Each call to
`inventory::submit!` produces a shim that evaluates the given expression and
registers it into a registry of its corresponding type. This registration
happens dynamically as part of life-before-main for statically linked elements.
Elements brought in by a dynamically loaded library are registered at the time
that dlopen occurs.

[`ctor`]: https://github.com/mmastrac/rust-ctor

Platform support includes Linux, macOS and Windows. Other platforms will simply
find that no plugins have been registered. Support for other targets would need
to be added in the `ctor` crate.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
