use std::mem;

struct Thing(usize);

#[test]
fn test_iter() {
    // https://github.com/rust-lang/rust/issues/113941
    assert_eq!(16, mem::size_of::<inventory::iter<Thing>>()); // FIXME
    assert_eq!(8, mem::align_of::<inventory::iter<Thing>>()); // FIXME
}
