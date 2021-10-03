//! [![github]](https://github.com/dtolnay/inventory)&ensp;[![crates-io]](https://crates.io/crates/inventory)&ensp;[![docs-rs]](https://docs.rs/inventory)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! **Typed distributed plugin registration.**
//!
//! This crate provides a way to set up a plugin registry into which plugins
//! can be registered from any source file linked into your application. There
//! does not need to be a central list of all the plugins.
//!
//! # Examples
//!
//! Suppose we are writing a command line flags library and want to allow any
//! source file in the application to register command line flags that are
//! relevant to it.
//!
//! This is the flag registration style used by [gflags] and is better suited
//! for large scale development than maintaining a single central list of flags,
//! as the central list would become an endless source of merge conflicts in an
//! application developed simultaneously by thousands of developers.
//!
//! [gflags]: https://gflags.github.io/gflags/
//!
//! ## Instantiating the plugin registry
//!
//! Let's use a `struct Flag` as the plugin type, which will contain the short
//! name of the flag like `-v`, the full name like `--verbose`, and maybe other
//! information like argument type and help text. We instantiate a plugin
//! registry with an invocation of `inventory::collect!`.
//!
//! ```
//! pub struct Flag {
//!     short: char,
//!     name: &'static str,
//!     /* ... */
//! }
//!
//! impl Flag {
//!     pub fn new(short: char, name: &'static str) -> Self {
//!         Flag { short, name }
//!     }
//! }
//!
//! inventory::collect!(Flag);
//! ```
//!
//! This `collect!` call must be in the same crate that defines the plugin type.
//! This macro does not "run" anything so place it outside of any function body.
//!
//! ## Registering plugins
//!
//! Now any crate with access to the `Flag` type can register flags as a plugin.
//! Plugins can be registered by the same crate that declares the plugin type,
//! or by any downstream crate.
//!
//! ```
//! # struct Flag;
//! #
//! # impl Flag {
//! #     fn new(short: char, name: &'static str) -> Self {
//! #         Flag
//! #     }
//! # }
//! #
//! # inventory::collect!(Flag);
//! #
//! inventory::submit! {
//!     Flag::new('v', "verbose")
//! }
//! #
//! # fn main() {}
//! ```
//!
//! The `submit!` macro does not "run" anything so place it outside of any
//! function body. In particular, note that all `submit!` invocations across all
//! source files linked into your application all take effect simultaneously. A
//! `submit!` invocation is not a statement that needs to be called from `main`
//! in order to execute.
//!
//! ## Iterating over plugins
//!
//! The value `inventory::iter::<T>` is an iterator with element type `&'static
//! T` that iterates over all plugins registered of type `T`.
//!
//! ```
//! # struct Flag {
//! #     short: char,
//! #     name: &'static str,
//! # }
//! #
//! # inventory::collect!(Flag);
//! #
//! for flag in inventory::iter::<Flag> {
//!     println!("-{}, --{}", flag.short, flag.name);
//! }
//! ```
//!
//! There is no guarantee about the order that plugins of the same type are
//! visited by the iterator. They may be visited in any order.

#![doc(html_root_url = "https://docs.rs/inventory/0.1.10")]
#![cfg_attr(not(inventory_require_std), no_std)]
#![allow(
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::expl_impl_clone_on_copy,
    clippy::let_unit_value,
    clippy::must_use_candidate,
    clippy::semicolon_if_nothing_returned, // https://github.com/rust-lang/rust-clippy/issues/7324
)]

#[cfg(not(inventory_require_std))]
extern crate alloc;
#[cfg(not(inventory_require_std))]
use alloc::boxed::Box;

// Not public API.
#[doc(hidden)]
pub use ctor::ctor;

// Not public API.
#[doc(hidden)]
pub use inventory_impl as r#impl;

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry<T: 'static> {
    head: AtomicPtr<Node<T>>,
}

#[doc(hidden)]
pub struct Node<T: 'static> {
    value: T,
    next: *mut Node<T>,
}

impl<T: 'static> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: ptr::null_mut(),
        }
    }
}

#[doc(hidden)]
pub struct StaticNode<T: 'static>(UnsafeCell<MaybeUninit<Node<T>>>);

unsafe impl<T: 'static> Send for StaticNode<T> {}
unsafe impl<T: 'static> Sync for StaticNode<T> {}

impl<T: 'static> StaticNode<T> {
    pub const unsafe fn new() -> Self {
        Self(UnsafeCell::new(MaybeUninit::uninit()))
    }

    pub fn get(&self) -> *mut Node<T> {
        self.0.get() as *mut _
    }
}

/// Trait bound corresponding to types that can be iterated by inventory::iter.
///
/// This trait cannot be implemented manually. Instead use the [`collect`] macro
/// which expands to an implementation of this trait for the given type.
///
/// # Examples
///
/// ```
/// use inventory::Collect;
///
/// fn count_plugins<T: Collect>() -> usize {
///     inventory::iter::<T>.into_iter().count()
/// }
/// ```
pub trait Collect: Sized + 'static {
    #[doc(hidden)]
    fn registry() -> &'static Registry<Self>;
}

// Not public API. Used by generated code.
#[doc(hidden)]
pub fn submit<T: Collect>(value: T) {
    // TODO: Avoid allocation by storing node in a static mut Option<Node<T>>
    // after existential type is stable. See comment in inventory-impl.
    let node = Box::leak(Box::new(Node::new(value)));
    unsafe {
        T::registry().submit_raw(node);
    }
}

// Not public API. Used by generated code.
#[doc(hidden)]
#[inline]
pub unsafe fn submit_raw<T: Collect>(node: *mut Node<T>) {
    T::registry().submit_raw(node);
}

impl<T: 'static> Registry<T> {
    // Not public API. Used by generated code.
    pub const fn new() -> Self {
        Registry {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    unsafe fn submit_raw(&'static self, new: *mut Node<T>) {
        let mut head = self.head.load(Ordering::SeqCst);
        let new_next_ptr = ptr::addr_of_mut!((*new).next);
        loop {
            // `new` is always a valid Node<T>, and is not yet visible through the registry.
            // `head` is always null or valid *mut Node<T>.
            ptr::write(new_next_ptr, head);
            match self
                .head
                .compare_exchange(head, new, Ordering::SeqCst, Ordering::SeqCst)
            {
                Ok(_) => return,
                Err(prev) => head = prev,
            }
        }
    }
}

#[allow(non_camel_case_types)]
mod private {
    use ghost::phantom;

    #[phantom]
    pub struct iter<T>;
}

/// An iterator over plugins registered of a given type.
///
/// The value `inventory::iter::<T>` is an iterator with element type `&'static
/// T`.
///
/// There is no guarantee about the order that plugins of the same type are
/// visited by the iterator. They may be visited in any order.
///
/// # Examples
///
/// ```
/// # struct Flag {
/// #     short: char,
/// #     name: &'static str,
/// # }
/// #
/// # inventory::collect!(Flag);
/// #
/// # const IGNORE: &str = stringify! {
/// use my_flags::Flag;
/// # };
///
/// fn main() {
///     for flag in inventory::iter::<Flag> {
///         println!("-{}, --{}", flag.short, flag.name);
///     }
/// }
/// ```
///
/// Refer to the [crate level documentation](index.html) for a complete example
/// of instantiating a plugin registry and submitting plugins.
#[allow(non_camel_case_types)]
pub type iter<T> = private::iter<T>;

#[doc(hidden)]
pub use crate::private::*;

const ITER: () = {
    fn into_iter<T: Collect>() -> Iter<T> {
        let head = T::registry().head.load(Ordering::SeqCst);
        Iter {
            // Head pointer is always null or valid *mut Node<T>.
            node: head,
        }
    }

    impl<T: Collect> IntoIterator for iter<T> {
        type Item = &'static T;
        type IntoIter = Iter<T>;

        fn into_iter(self) -> Self::IntoIter {
            into_iter()
        }
    }

    #[doc(hidden)]
    impl<T: Collect> Deref for iter<T> {
        type Target = fn() -> Iter<T>;
        fn deref(&self) -> &Self::Target {
            &(into_iter as fn() -> Iter<T>)
        }
    }

    #[derive(Clone)]
    pub struct Iter<T: 'static> {
        node: *mut Node<T>,
    }

    impl<T: 'static> Iterator for Iter<T> {
        type Item = &'static T;

        fn next(&mut self) -> Option<Self::Item> {
            if self.node.is_null() {
                None
            } else {
                let node = unsafe { &*self.node };
                let value = &node.value;
                self.node = node.next;
                Some(value)
            }
        }
    }
};

/// Associate a plugin registry with the specified type.
///
/// This call must be in the same crate that defines the plugin type. This macro
/// does not "run" anything so place it outside of any function body.
///
/// # Examples
///
/// Suppose we are writing a command line flags library and want to allow any
/// source file in the application to register command line flags that are
/// relevant to it.
///
/// This is the flag registration style used by [gflags] and is better suited
/// for large scale development than maintaining a single central list of flags,
/// as the central list would become an endless source of merge conflicts.
///
/// [gflags]: https://gflags.github.io/gflags/
///
/// ```
/// pub struct Flag {
///     short: char,
///     name: &'static str,
///     /* ... */
/// }
///
/// inventory::collect!(Flag);
/// ```
///
/// Refer to the [crate level documentation](index.html) for a complete example
/// of submitting plugins and iterating a plugin registry.
#[macro_export]
macro_rules! collect {
    ($ty:ty) => {
        impl $crate::Collect for $ty {
            #[inline]
            fn registry() -> &'static $crate::Registry<Self> {
                static REGISTRY: $crate::Registry<$ty> = $crate::Registry::new();
                &REGISTRY
            }
        }
    };
}

/// Enter an element into the plugin registry corresponding to its type.
///
/// This call may be in the same crate that defines the type, or downstream in
/// any crate that depends on that crate.
///
/// This macro does not "run" anything so place it outside of any function body.
/// In particular, note that all `submit!` invocations across all source files
/// linked into your application all take effect simultaneously. A `submit!`
/// invocation is not a statement that needs to be called from `main` in order
/// to execute.
///
/// # Examples
///
/// Put `submit!` invocations outside of any function body.
///
/// ```
/// # struct Flag;
/// #
/// # impl Flag {
/// #     fn new(short: char, name: &'static str) -> Self {
/// #         Flag
/// #     }
/// # }
/// #
/// # inventory::collect!(Flag);
/// #
/// inventory::submit! {
///     Flag::new('v', "verbose")
/// }
/// #
/// # fn main() {}
/// ```
///
/// Do not try to invoke `submit!` from inside of a function body as it does not
/// do what you want.
///
/// ```compile_fail
/// // Do not do this.
/// fn submit_flags(has_verbose_flag: bool) {
///     if has_verbose_flag {
///         inventory::submit! {
///             Flag::new('v', "verbose")
///         }
///     }
/// }
/// ```
///
/// Specify the type to avoid allocation:
///
/// ```
/// # #[derive(Debug, PartialEq)]
/// # struct Flag { short: char, name: &'static str };
/// #
/// # impl Flag {
/// #     fn new(short: char, name: &'static str) -> Self {
/// #         Self { short, name }
/// #     }
/// # }
/// #
/// # inventory::collect!(Flag);
/// #
/// inventory::submit! {
///     #![type = Flag]
///     Flag::new('v', "verbose")
/// }
///
/// let mut iter = inventory::iter::<Flag>.into_iter();
/// assert_eq!(iter.next(), Some(&Flag::new('v', "verbose")));
/// assert_eq!(iter.next(), None);
///
/// ```
///
/// Refer to the [crate level documentation](index.html) for a complete example
/// of instantiating and iterating a plugin registry.
#[macro_export]
macro_rules! submit {
    ($($value:tt)*) => {
        $crate::r#impl::submit! {
            $($value)*
        }
    }
}

#[allow(dead_code)]
fn unused() {
    let _ = ITER;
}
