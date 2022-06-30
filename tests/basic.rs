#![feature(generators)]

use std::any::Any;
use yield_iter::generator;

#[test]
fn range() {
    let a = generator! {
        for i in 0..10 {
            yield i
        }
    };
    let b = 0..10;

    assert!(a.eq(b));
}

#[test]
fn after_completion() {
    let mut iter = generator! {
        yield 1;
        yield 2;
    };
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    // ...
    assert_eq!(iter.next(), None);
}
