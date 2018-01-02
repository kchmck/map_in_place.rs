//! Mutate a slice in place with a map function.
//!
//! Note that the map result type must be the same as the input type.
//!
//! ## Example
//!
//! ```rust
//! use slice_mip::MapInPlace;
//!
//! let mut buf = [1, 2, 3, 4];
//! buf.map_in_place(|x| x * 2);
//!
//! assert_eq!(buf, [2, 4, 6, 8]);
//! ```

/// A sequence that can be mutated in place.
pub trait MapInPlace<T> {
    /// Apply the given map function to each item in the current sequence and replace each
    /// item with its associated map result.
    fn map_in_place<F>(&mut self, f: F) where F: FnMut(&T) -> T;
}

impl<T> MapInPlace<T> for [T] {
    fn map_in_place<F>(&mut self, mut f: F) where F: FnMut(&T) -> T {
        for item in self.iter_mut() {
            *item = f(item);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_in_place() {
        let mut buf = [0, 1, 2, 3];
        buf.map_in_place(|x| x + 1);

        assert_eq!(buf, [1, 2, 3, 4]);
    }
}
