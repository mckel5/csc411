/// A two-dimensional array of type `T`. `T` must implement the `Clone` trait.
#[derive(Clone)]
pub struct Array2<T> {
    pub width: usize,
    pub height: usize,
    array: Vec<Vec<T>>,
}

/// A row-major iterator over an `Array2`.
pub struct Array2IterRowMajor<'a, T> where T: Clone {
    row: usize,
    column: usize,
    array2: &'a Array2<T>,
}

/// A column-major iterator over an `Array2.`
pub struct Array2IterColumnMajor<'a, T> where T: Clone {
    row: usize,
    column: usize,
    array2: &'a Array2<T>,
}

impl<T: Clone> Array2<T> {
    /// Return an `Array2` with a defined `width` and `height`. Each value will be set to `value`.
    pub fn from_single_value(value: T, width: usize, height: usize) -> Array2<T> {
        Array2 { width, height, array: vec![vec![value; width]; height] }
    }

    /// Return an `Array2` built from a `Vec`, where each row will be of length `width`.
    /// The height of the `Array2` is inferred.
    pub fn from_row_major(vec: Vec<T>, width: usize) -> Array2<T> {
        let array = vec.chunks(width).map(|s| s.into()).collect();
        Array2 { width, height: vec.len() / width, array }
    }

    /// Return an `Array2` built from a `Vec`, where each column will be of length `height`.
    /// The width of the `Array2` is inferred.
    pub fn from_col_major(vec: Vec<T>, height: usize) -> Array2<T> {
        let mut array: Vec<Vec<T>> = Vec::new();
        for i in 0..height {
            array.push(vec.iter().skip(i).step_by(height).cloned().collect());
        }
        Array2 { width: vec.len() / height, height, array }
    }

    /// Retrieve an immutable reference to the  element at position `(row, col)`.
    /// Returns `None` if the element does not exist.
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        match (row, column) {
            (r, c)
            if (0..self.height).contains(&r) && (0..self.width).contains(&c) => {
                Some(&self.array[row][column])
            }
            _ => None
        }
    }

    /// Retrieve a mutable reference to the element at position `(row, col)`.
    /// Returns `None` if the element does not exist.
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        match (row, column) {
            (r, c)
            if (0..self.height).contains(&r) && (0..self.width).contains(&c) => {
                Some(&mut self.array[row][column])
            }
            _ => None
        }
    }

    /// Generate a row-major iterator over the `Array2`.
    pub fn iter_row_major(&self) -> Array2IterRowMajor<'_, T> {
        Array2IterRowMajor::from(self)
    }

    /// Generate a column-major iterator over the `Array2`.
    pub fn iter_col_major(&self) -> Array2IterColumnMajor<'_, T> {
        Array2IterColumnMajor::from(self)
    }
}

impl<'a, T: Clone> Array2IterRowMajor<'a, T> {
    fn from(array2: &Array2<T>) -> Array2IterRowMajor<T> {
        Array2IterRowMajor { row: 0, column: 0, array2 }
    }
}

impl<'a, T: Clone> Array2IterColumnMajor<'a, T> {
    fn from(array2: &Array2<T>) -> Array2IterColumnMajor<T> {
        Array2IterColumnMajor { row: 0, column: 0, array2 }
    }
}

// Iterator implementations: https://aloso.github.io/2021/03/09/creating-an-iterator
impl<'a, T: Clone> Iterator for Array2IterRowMajor<'a, T> {
    type Item = T;

    /// Return a row-major iterator over the `Array2` containing tuples of the form
    /// (row, column, value)
    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.array2.get(self.row, self.column).cloned();

        if self.column == self.array2.width - 1 {
            // "Wrap around" if end of row is reached
            self.column = 0;
            self.row += 1;
        } else {
            self.column += 1;
        }

        return_value
    }
}

impl<'a, T: Clone> Iterator for Array2IterColumnMajor<'a, T> {
    type Item = T;

    /// Return a column-major iterator over the `Array2` containing tuples of the form
    /// (row, column, value)
    fn next(&mut self) -> Option<Self::Item> {
        let return_value = self.array2.get(self.row, self.column).cloned();

        if self.row == self.array2.height - 1 {
            // "Wrap around" if end of column is reached
            self.row = 0;
            self.column += 1;
        } else {
            self.row += 1;
        }

        return_value
    }
}

#[cfg(test)]
mod tests {
    use crate::Array2;

    #[test]
    fn normal_access() {
        let array2 = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6], 3);
        assert_eq!(array2.get(0, 1), Some(&2_i32));
    }

    #[test]
    fn out_of_bounds_access() {
        let array2 = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6], 3);
        assert_eq!(array2.get(2, 0), None);
    }

    #[test]
    fn row_major_iter() {
        let array2 = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6], 3);
        let collect: Vec<i32> = array2.iter_row_major().collect();
        assert_eq!(collect, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn col_major_iter() {
        let array2 = Array2::from_row_major(vec![1, 2, 3, 4, 5, 6], 3);
        let collect: Vec<i32> = array2.iter_col_major().collect();
        assert_eq!(collect, vec![1, 4, 2, 5, 3, 6]);
    }

    #[test]
    fn from_single_value() {
        let array2 = Array2::from_single_value(0, 3, 3);
        assert_eq!(array2.array,
                   vec![
                       vec![0, 0, 0],
                       vec![0, 0, 0],
                       vec![0, 0, 0],
                   ]
        )
    }
}