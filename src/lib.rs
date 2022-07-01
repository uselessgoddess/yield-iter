#![feature(generator_trait)]

use std::collections::LinkedList;
use std::{
    ops::{Generator, GeneratorState},
    pin::Pin,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

/// A wrapper struct around Generators,
/// providing a safe implementation of the [`Iterator`] trait.
pub struct YieldIter<G>(G);

impl<G> Unpin for YieldIter<G> {}

impl<G: Generator + Unpin> YieldIter<G> {
    /// Creates a new `GenIter` instance from a generator.
    /// The returned instance can be iterated over,
    /// consuming the generator.
    #[inline]
    pub fn new(generator: G) -> Self {
        Self(generator)
    }
}

impl<G: Generator + Unpin> Iterator for YieldIter<G> {
    type Item = G::Yield;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        Pin::new(self).next()
    }
}

impl<G: Generator + Unpin> YieldIter<G> {
    /// Creates a new `GenIter` instance from a generator.
    ///
    /// The returned instance can be iterated over,
    /// consuming the generator.
    ///
    /// # Safety
    /// This function is marked unsafe,
    /// because the caller must ensure the generator is in a valid state.
    /// A valid state means that the generator has not been moved ever since it's creation.
    #[inline]
    pub unsafe fn new_unchecked(generator: G) -> Self {
        Self(generator)
    }
}

impl<G: Generator> Iterator for Pin<&mut YieldIter<G>> {
    type Item = G::Yield;

    fn next(&mut self) -> Option<Self::Item> {
        let this: Pin<&mut YieldIter<G>> = self.as_mut();

        // This should be safe.
        // this Iterator implementation is on a Pin<&mut GenIter<G>> where G: Generator.
        // In order to acquire such a Pin<&mut YieldIter<G>> if G does *NOT* implement Unpin,
        // the unsafe `new_unchecked` function from the Pin type must be used anyway.
        //
        // Note that if G: Unpin, the Iterator implementation of YieldIter<G> itself is used,
        // which just creates a Pin safely, and then delegates to this implementation.
        let gen = unsafe { this.map_unchecked_mut(|gen| &mut gen.0) };

        match gen.resume(()) {
            GeneratorState::Yielded(y) => Some(y),
            GeneratorState::Complete(_) => None,
        }
    }
}

#[cfg(feature = "rayon")]
impl<G> IntoParallelIterator for YieldIter<G>
where
    G: Generator + Unpin,
    G::Yield: Sync + Send,
{
    type Iter = rayon::vec::IntoIter<Self::Item>;
    type Item = G::Yield;

    fn into_par_iter(self) -> Self::Iter {
        Vec::from_iter(self).into_par_iter()
    }
}

/// Create `YieldIter` with the provided generator body
/// # Examples
/// ```
/// #![feature(generators, generator_trait)]
///
/// use yield_iter::generator;
///
/// let x = 10;
/// let iter = generator! {
///     let r = &x;
///
///     for i in 0..5u32 {
///         yield i * *r
///     }
/// };
/// ```
#[macro_export]
macro_rules! generator {
    ($($x:tt)*) => {
        // SAFETY: Generator is directly passed into new_unchecked,
        // so it has not been moved
        unsafe {
            $crate::YieldIter::new_unchecked(|| {
                $($x)*
            })
        }//.fuse()
    };
}
