# Orphan Rule

## Purpose

1. Prevent conflict where different modules provide overlapping `impl`s.
2. Enable lazy loading and partial type check.

## Specification

### Definitions

CURRENT MODULE is the module that defines the `impl`.

LOCAL: A trait or a type is called LOCAL to a module if it was defined by that module.

### The rule

`impl Trait(P0...Pn)` is allowed if either (1) or (2) is true.

(1): `Trait` is LOCAL to the CURRENT MODULE.

(2): At least one type `Pi` is LOCAL to the CURRENT MODULE.
