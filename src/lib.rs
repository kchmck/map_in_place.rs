pub trait MapInPlace<T> {
    fn map_in_place<F>(&mut self, mut f: F) where F: FnMut(&T) -> T;
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
    #[test]
    fn test_map_in_place() {
        let mut buf = [0, 1, 2, 3];
        buf.map_in_place(|x| x + 1);

        assert_eq!(buf, [1, 2, 3, 4]);
    }
}
