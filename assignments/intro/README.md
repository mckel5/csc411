# Assignment 1

CSC 411

Marceline Kelly

With help from Nicolas Leffray

---

## Prompts

### `brightness`

Please write a Rust program `brightness`, which should print the average brightness of a grayscale image. Every pixel in a grayscale image has a brightness between 0 and 1, where 0 is black and 1 is as bright as possible. Print the average brightness using decimal notation with exactly one digit before the decimal point and three digits after the decimal point. Print only the brightness.

### `fgroups`

In this problem you are to write a program `fgroups` (short for “fingerprint groups”), which when given a set of names with fingerprints will identify groups of names that share a fingerprint. The real object of the exercise is to familiarize you with some parts of the standard library.

## Implementation

All parts of this assignment have been correctly implemented.

## `fgroups`: problem-solving potential

Digital fingerprints are, for better or for worse, a core feature of the modern web. These invisible pieces of data allow organizations to track users across websites, devices, and geolocations.

One prominent use for categorizing fingerprints may be advertising. Similar fingerprints among users may be indiciative of common interests or lifestyles. Advertisers can exploit this data to tailor their advertisements to specific groups of web users.

A more benevolent application may be security. Fingerprints may be used to authenticate users, though they may not be sufficient for applications with sensitive data. Users in a given fingerprint group may be given special roles or permissions that allow them to interact with the application differently than other users.

## Time usage

This assignment took approximately (3) hours of work, divided as follows:

- 1 hour: `brightness`
- 2 hours: `fgroups`