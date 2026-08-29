#![feature(sanitize)]

struct Plugin(&'static str);

inventory::collect!(Plugin);

inventory::submit! {
    Plugin("hello")
}

fn main() {
    for plugin in inventory::iter::<Plugin> {
        std::hint::black_box(plugin.0);
    }
}
