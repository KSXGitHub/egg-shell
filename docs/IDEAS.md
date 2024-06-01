## Type-level Certifications

### Domains

Verifier functions may assign bounds to their return values. For example, a non-zero verifier will assign a `(!= 0)` certificate to its returning number.

~~One domain may be a subset of another. For example, `(> 3)` is a subset of `(> 2)`. Therefore, a `(> 3)` domain should imply `(> 2)`.~~

~~One domain may intersect with another. For example, `(> 3)` and `(< 5)` intersects at `(> 3) and (< 5)`.~~

Combining two domains with no intersection should result in a `never` type. For example, `(!= 0) and (== 0)` should collapse into `never`.

### Nominal certifications

Verifier functions may assign certificates to their return values. For example, UTF-8 verifier will assign a `utf8` certificate to its returning byte stream.

## Subtypes

> [!NOTE]
> There are similarities between Type-level Certifications and this. More experiments are required to determine their difference, usefulness, complexity, and implementation viability.

A subtype `B` of `A` is just `A` but with potentially fewer variants.

A value of type `A` is incompatible with `B` but a value of type `B` is compatible with `A`.

Only a chosen few functions can construct or verify `B`.

A subtype can inherit from multiple subtypes of the same origin type. For example, if `B1` and `B2` are both subtypes of `A`, then it is possible to declare a `C` that subtypes both `B1` and `B2`.

A subtype must have the same size as the base type. Thus, the subtyping mechanism does not permit OOP-style inheritance.

## Embedded text verifier

Verify embedded string literal at type-checker level. For example, verifying embedded JavaScript code to have correct syntax.

### Multi-line string

Multi-line string shares the same syntax as embedded text verifier, but with text verifier being no-op.

User may optionally specifies newline type (LF or CRLF). By default, the newline would be whatever the file uses.

## Macros

Macros don't merely process AST.

### Procedural macros

Const functions from an imported module can be used as a macro as long as the signature fits.

Functions tagged with the `meta` keyword may be used as a macro and should be type-checked before non-`meta` entities.

### Declarative macros

Declarative macros are declared with the `macro` keyword. All identifiers within the body of a declarative macro shall be resolved to their canonical paths, as a consequence:
* Declarative macros are bound to a scope.
* Declarative macros are incapable of referencing identifiers that don't exist.

**Differences from Rust:** In Rust, identifiers within a macro are only resolved after the macro expands, which means that identifiers within the macro will refer to whatever inside the module scope which the macro expands in. As a consequence, macro library authors are often forced to write canonical path (e.g. `::core::convert::Into::into()` instead of just `.into()`).

### Macro composition

Macros may be created by composing multiple macros.

### Eager type-checking in macros

Well-typed tokens inside macro body may be checked for type correctness before it was realized.

Macro applications with well-typed return type may be checked for type correctness without expanding the AST.

## First-Class Entities

### First-Class Functions

* A function is also a value.

### First-Class Types

* A concrete type is a value.
* A generic type is a type constructor.
* A type constructor is a const function that return a concrete type.

### First-Class Kinds

* A kind of a type or a kind is also a type.

### First-Class Traits

* A trait is a const function that returns a "constraint".
* The parameters of a trait can be any const value, including: primitives, types, kinds, traits, etc.

### First-Class Modules

* A concrete module is a value.
* A parameterized module is a function that return a concrete module.

## Trait Implementation Targets

* Concrete types.
* Type constructors.

## Implied Trait Implementation

Some trait may imply another trait (for example: `Convert(A, B)` implies `TryConvert(A, B, never)`), but a blanket implementation would take away the ability to customize them. Therefore, trait implementation writer should still explicitly specify the implied trait implementation. For convenience, a `derive` macro on top of trait implementation should be used.

## WASM-based plugins

### WASM-based type constructors

Type constructors may be a WASM module.

### WASM-based const functions

Const functions may be a WASM module.

### WASM-based macros

Macros may be a WASM module.
