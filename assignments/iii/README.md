# Assignment 2: Interfaces, Implementations, and Images

CSC 411

Marceline Kelly

---

## Design

### Abstract concept

I am trying to represent a two-dimensional array or, in other words, a container of containers of values.

### Functions and contracts

This 2D array will offer the following methods:

- `from_single_value(value: T, width: usize, height: usize)` constructs an array of two given dimensions then sets each element to a predefined value.
- `from_row_major(vec: Vec<T>, width: usize)` constructs an array from a one-dimensional, row-major vector.
- `from_col_major(vec: Vec<T>, height: usize)` constructs an array from a one-dimensional, column-major vector.
- `get(&self, row: usize, col: usize) -> T` accesses individual elements.
- `iter_row_major(&self) -> Array2Iter<'_, T>` returns a row-major iterator of the array.
- `iter_col_major(&self) -> Array2Iter<'_, T>` returns a column-major iterator of the array.

### Representation and invariants

`Array2` will be built upon a vector of vectors (i.e. `Vec<Vec<T>>`). It will satisfy the following invariants:

- Any instance of `Array2` with type `T` will have a concrete width and height, each greater than zero. Each element will be a value of type `T`.
- Row-major and column-major iterators may be requested from an `Array2` regardless of the type of `Vec` used to initialize the array (or the underlying implementation).
- Requesting the value at coordinates `(x, y)` will produce the value at the `x`th row and `y`th column of the `Array2`. `Array2`s are zero-indexed.

## Implementation

All parts of this assignment have been correctly implemented.

## Time expenditure

- 1.5 hours: Creation and revision of design document
- 0.5 hours: Planning out the inner workings of the `Array2` data structure
- 2 hours: Implementation and testing of `Array2`
- 1 hour: Implementation and testing of the Sudoku checker

**Total: 5 hours**