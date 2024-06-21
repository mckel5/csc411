# A6: profile

Marceline Kelly -- CSC 411

with ideas from Nicolas Leffray

---

## Performance analysis

`rumdis::get_bits` is the most time-intensive single routine (with no calls to subroutines) that I wrote. This function simply isolates groups of bits from a 32-bit instruction, akin to the `get` functions in `bitpack`. After analyzing the assembly code generated from this routine, I've concluded that there is no obvious, assembly-level way to improve this routine. I admit that there may be some striking inefficiency that I've missed, but I find this unlikely for two reasons:

1. Modern compilers are able to produce extremely efficient assembly code, often with greater reliability than a human
2. Most of the time spent in `rumrun::execute` (the main execution loop of the UM) is not spent in any subroutines, meaning the program is mostly performing arithmetic, register loads/stores, etc. If such basic operations are taking up the bulk of the program's running time, it follows that there is little room for improvement in the program's assembly routines

## Time usage

- Problem analysis: 1 hour
- Problem solutions: 4 hours