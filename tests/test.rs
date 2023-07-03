use std::mem;

struct Thing(usize);

#[test]
fn test_iter() {
    assert_eq!(0, mem::size_of::<inventory::iter<Thing>>());
    assert_eq!(1, mem::align_of::<inventory::iter<Thing>>());
}
