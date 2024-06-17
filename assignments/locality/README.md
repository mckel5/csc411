# A3: locality

Marceline Kelly -- CSC 411

---

## Collaboration

I collaborated with Nicolas Leffray to complete this assignment, mostly in the planning stages. We have separate code, however.

## Implementation

All parts of this program have been correctly implemented, including the "extra" image operations.

## Architecture

Each image transformation is, under the hood, a variant of the `transform` function. This function takes a function as an argument which specifies how to map each input pixel to its corresponding output pixel. This allows for a substantial reduction in code duplication by consolidating the iteration code shared between each transformation.

The result of each transformation is written to a single, mutable `Array2`. This way, an arbitrarily large number of transformations can be applied to a single image without needlessly duplicating `Array2`s over and over.

## Benchmarks

| Operation | Row-major time (s) | Column-major time (s)
| --- | --- |
| Rotate 90 | 2.66 | 1.77
| Rotate 180 | 0.88 | 6.99

Initially, I predicted that rotate 90 row-major would be the fastest operation, rotate 180 row-major would be the next fastest, and the two column-major operations would take roughly the same amount of time. However, this is clearly not the case.

My `Array2` is implemented using row-major `Vec`s. I suspect that rotate 180 row-major is the fastest operation because elements from the input `Array2` are iterated in row-major order, then placed into the output `Array2` in row-major order, as well. Each line of pixels in the output is parallel to its corresponding input line. Similarly, this explains why rotate 180 column-major is the slowest operation -- both `Array2`s are limited by the poor locality of column-major operations with my `Array2`.

90 degree rotations, on the other hand, are "perpendicular" operations. Each line of pixels in the output is perpendicular to its input line. As such, both 90 degree rotations are somewhere in the middle in terms of performance -- either their input can be accessed or their output can be built in row-major fashion, but not both. The fact that rotate 90 column-major is slower than rotate 90 row-major is strange, however. This seems illogical considering the implementation details of my `Array2`, and I am not sure as to why this could be the case.

## Alternate Memory Layouts

I believe that splitting `Array2` into two independent data structures, `Array2RowMajor` and `Array2ColumnMajor`, would improve the cache hit rate of my program. This way, the programmer can choose which `Array2` to use based on the transformation operation being performed. As explained above, certain operations work best with row- or column-major iterables, so explicitly specifying this would likely enhance the program's performance.

## Time Usage

The design document was the most time-consuming aspect of this assignment. Nicolas and I spent roughly 5 hours completing it, which I would attribute to our poor understanding of cache locality at that point. I found the actual implementation to be a much quicker task, requiring about 1 hour of programming and 1 hour of debugging. Finally, the benchmarking took about an hour, as well.

Total time spent: 8 hours
