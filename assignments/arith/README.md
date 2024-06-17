# A4: arith

Marceline Kelly -- CSC 411

---

## Implementation

To my knowledge, all parts of this program have been correctly implemented.
I acknowledge that some edge cases concerning bit packing may exhibit unwanted behavior, but my tests have not found any such cases.

## Architecture

This program is divided into several modules that handle the compression & decompression of images.
Here is the flow of the compression algorithm:

1. `main`: entry point of program
2. `codec`: read PPM image, trim if necessary
3. `normalize`: normalize each RGB value
4. `component`: convert RGB values to component (Y, Pb, Pr) values
5. `dct`: perform discrete cosine transform (a, b, c, d, Pb_avg, Pr_avg)
6. `quantize`: scale cosine values to integers for bit packing
7. `bitpack`: pack quantized values into 32-bit words
8. `codec`: write compressed image

These steps are performed in reverse for the decompression algorithm.

Each module is isolated from all the others, with minimal sharing of values, as appropriate.
Data is generally passed around using descriptive structs to simplify code analysis.

## Time Usage

### Analysis

I was able to grasp the underlying concepts of this assignment relatively quickly (save for some of the more advanced bit packing laws).
I'd estimate that it took me roughly 2 hours to analyze the assignment and develop a method for solving the problems presented.

### Solutions

The implementation of this assignment certainly took longer than the analysis, but I wouldn't say it was necessarily more difficult.
Because I planned my solutions ahead of time, I was able to write comprehensive unit tests for each module.
This ensured that I was always on the right track and wasn't spitting out code without knowing what I was doing.
Additionally, many of the steps were just straightforward calculations using formulas provided in the handout.
I'd say that my initial solutions for the steps in the program took about 4 hours.
After that, I spent 2 hours or so refining my unit tests and making sure as many edge cases as possible were covered.
