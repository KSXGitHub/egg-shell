# Ideas

> [!NOTE]
> None of the syntax shown below are finalized. Treat them as pseudo-language.

## Reproducible Build

The same set of source code (including the lockfile), compiler parameters, toolchain version, and target platform should always result in the exact same set of output files.

## Compile-time Parameters for Whole Project or Library

Some projects and libraries would like to produce different outputs (e.g. different programs) based on different parametric configurations or compiler environments. However, allowing them to access the system directly from the source code would violate both the security sandbox and [reproducibility guarantee](#reproducible-build). Therefore, they may be accessed indirectly via either the compiler's CLI arguments or configuration files.

If a dependency requires compile-time parameters, they must be passed down from its dependant via the manifest file. The compile-time parameters shall be part of the identity of the dependency, similar to how generic works.

Compile-time parameters in the source tree may accept variables and expressions. The variables are the compile-time parameters. This would allow one to pass parameters from the CLI all the way down to indirect dependency should the configuration of the dependency chain allows it.

To allow the language server to work correctly without having to specify compile-time parameters, the types of compile-time parameters must always be declared as part of the source tree such as configuration files, source files, etc. If the compile-time parameters are set in the source tree, their types may be omitted.

Compile-time parameters set in the source tree are only default values. For a project, they can be overridden by compiler's CLI arguments. For a dependency, they can be overridden by having the dependant passing different values than the default.

Initially, the types of the compile-time parameters must be built-in primitives. More thought and design is required for the use of ecosystem types (custom types defined by users via library code).

### Compile-time Environment Variables

Compile-time Environment Variables are just string typed compile-time parameters. Nothing is special about them.

> [!NOTE]
> To ensure determinism and reproducibility, the compiler **must not** inherit environment variables **automatically**.
> The user must explicitly specify the variable names (and optionally values) via CLI or configuration files.

## Type-level Certifications

> [!TIP]
> Perhaps type-theoretical proofs may prove useful here?

<details><summary>(without type-theoretical proofs)</summary>

### Domains

Verifier functions may assign bounds to their return values. For example, a non-zero verifier will assign a `(!= 0)` certificate to its returning number.

~~One domain may be a subset of another. For example, `(> 3)` is a subset of `(> 2)`. Therefore, a `(> 3)` domain should imply `(> 2)`.~~

~~One domain may intersect with another. For example, `(> 3)` and `(< 5)` intersects at `(> 3) and (< 5)`.~~

Combining two domains with no intersection should result in a `never` type. For example, `(!= 0) and (== 0)` should collapse into `never`.

### Nominal certifications

Verifier functions may assign certificates to their return values. For example, UTF-8 verifier will assign a `utf8` certificate to its returning byte stream.

</details>

## Subtypes

A subtype `B` of `A` is just `A` but with potentially fewer variants.

A value of type `A` is incompatible with `B` but a value of type `B` is compatible with `A`.

If the inner of `B` does not have any special requirements than that of `A` (such as subtyped struct fields, narrower sum type, or type-level certifications), a value of type `A` can be explicitly cast into type `B`.

A subtype can inherit from multiple subtypes of the same origin type. For example, if `B1` and `B2` are both subtypes of `A`, then it is possible to declare a `C` that subtypes both `B1` and `B2`.

A subtype must have the same size as the base type. Thus, the subtyping mechanism does not permit OOP-style inheritance.

If the base type is a struct, the subtype can be a struct with stricter fields (i.e. fields whose types are subtypes of the fields in the base type). If the base type is a sum type, the subtype can include fewer variants than and from the base type, and/or can have stricter variant value types.

Orphan rule: If at least one of the direct or indirect base types is foreign, a foreign trait cannot be implemented on the subtype.

## Embedded text verifier

Verify embedded string literal at type-checker level. For example, verifying embedded JavaScript code to have correct syntax.

### Multi-line string

Multi-line string shares the same syntax as embedded text verifier, but with text verifier being no-op.

User may optionally specifies newline type (LF or CRLF). The default newline type is LF (even if the source code uses CRLF).

All raw newlines, be it LF or CRLF, would be converted to the specified newline type. This is to ensure consistency between coding platforms.

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
* Every function has a unique type, except function pointers.

### First-Class Types

* A concrete type is a value.
* A generic type is a type constructor.
* A type constructor is a const function that return a concrete type.
* There is a type of all types.
* The type of all types also includes itself ([no type universes](#no-type-universes)).

### First-Class Kinds

* A kind of a type or a kind is also a type.
* The type of all types also includes all kinds.

### First-Class Traits

* A trait is a type constructor.
* Most traits are record type constructors.
* Traits that conflicted with each other are sum type constructors.
* The parameters of a trait is the parameters of the type constructor.
* The parameters of a trait can be any const value, including: primitives, types, kinds, traits, etc.
* Every set of trait and parameters can only have at most 1 "canonical" instance.
* Since a trait is a type constructor, it is also a const function.
* Since a trait is a const function, it has a unique type.

### First-Class Modules

* A concrete module is a value.
* A parameterized module is a function that return a concrete module.
* Every module has a unique type.

## No Type Universes

We are making a programming language, not a proof assistant.

The type system of a programming language only needs to make sure that the type represents the data. A paradox that cannot terminate can cause neither type unsafety, undefined behavior, nor memory corruption.

Maybe one day, we would add dependent proof to enable the users to make more static assertions and the optimizer to be more aggressive, but such proof would have been required to be total, eagerly evaluated, strongly normalized, and maybe other properties to be used as a proof. There should still be no needs type universes.

## Trait Instance Targets

* Concrete types.
* Type constructors.

## Trait bounds

### Implied trait bounds

If trait bound `Foo(...)` implies trait bound `Bar(...)`,
1. Expression `Foo(...)` is equivalent to expression `Foo(...) and Bar(...)`.
2. Constraint `Foo(...)` is a subset of constraint `Bar(...)`.
3. When one writes the expression `Foo(...) and Bar(...)`, the linter may warn of unnecessary trait bound in `Bar(...)`.

Some traits may imply another trait (for example: `Convert(A, B)` implies `TryConvert(A, B, never)`), but a blanket instance would take away the ability to customize them. Therefore, trait instance writer should still explicitly specify the implied trait instance.

Some types may also imply a trait (for example: `HashMap(K, V)` implies `Hash(K)` and `Eq(K)`).

~~Implied trait bounds may be syntactically omitted when writing trait instances for generic types.~~

Both adding and removing implied trait bounds are backward incompatible change (semver major).

### Trait bounds by necessity

Unlike implied trait bounds where one trait bound is a subset of another, trait bounds by necessity are only required because of implementation requirements. For example, a generic struct may require a trait bound because one of its internal field has a type that has trait bounds.

Even if trait bound `Foo(...)` requires trait bound `Bar(...)` by necessity, `Foo(...)` does not imply `Bar(...)`. Therefore, in a generic declaration, adding `Foo(...)` does not provide the capabilities of `Bar(...)`.

The linter may warn of unused trait bounds by necessity if their presence are found to be unnecessary.

Adding trait bounds by necessity is a backward incompatible change (semver major), but removing them is backward compatible (semver minor).

## Variances of generic type parameters

Generic types are _invariant_ by default.

~~_Covariance_ and _contravariance_ can be expressed by higher order generic type parameters (types of types) with trait bounds related to subtyping.~~

(Can it really be expressed this way?)

## WASM-based plugins

> [!WARNING]
> Beware of [non-determinism](https://github.com/WebAssembly/design/blob/main/Nondeterminism.md). The non-deterministic behavior are to be disabled by default for plugins that may affect reproducibility. Plugins that require non-deterministic capabilities must request permission and should guarantee that the end result is unaffected by non-determinism. The end-user will assume that all plugins are deterministic regardless of whether they make use of non-deterministic capabilities. Failure of a plugin to guarantee determinism of its result is considered a bug of the plugin.

### WASM-based type aliases

Type aliases may be a WASM module.

> [!NOTE]
> **Why not WASM-based type constructors?**
>
> The compiler would need consistent and inferable information regarding memory layout and sizes, so there is no point in using opaque plugins to define them.

> [!NOTE]
> **What about Decidability?**
>
> The only reason this programming language would care about Decidability and Guaranteed Termination is to guarantee correctness, soundness, and consistency in proof uses (if the proof idea is ever implemented).
>
> Type aliases are purely computational entities for they would be resolved into canonical forms that use type constructors which are decidable. As a result, type aliases would be invisible to proofs.

### WASM-based const functions

Const functions may be a WASM module.

### WASM-based macros

Macros may be a WASM module.

## Notebook-like inline snapshots

* After `>>>` is an inquiry.
* After `<<<` is an answer.
* Before both `>>>` and `<<<` is the start of a line.
* An inquiry must be a constant expression.
* The answer is normalized.
* Under every inquiry must be an answer.
* The toolchain provides tools to auto-generate answers.

Example:

```
>>> 1 * 2 * 3 * 4 * 5
<<< 120
```

## Effect System

Compile-time check for possible effects on a function.

Examples of effects include, but are not limited to:
* `const`-ness: Whether the function can be called at compile-time.
* `static`-ness: Whether the function can be called in `static` context.
* Memory safety: Whether the function is memory-safe.
* Total/partial, divergent/convergent, and whether a function would crash.
* The various types of side-effects and capabilities.

> [!TIP]
> One may consider `unsafe` as a synonym for "all capabilities". This is critical from the point of view of [capability-based security](#capability-based-security).

### Generic effects

Passing effects as generic parameters.

### Capability-based security

The ability to annotate side-effects and capabilities is a sound base for capability-base security.

The programmer wants to use the various third-party libraries in the ecosystem. However, carelessly adding them to the project creates risks of supply-chain attacks or malware injections.

Letting the programmer specify capabilities for each third-party library certainly cannot completely replace manual code auditing, but it does make the job easier.

The programmer would specify what capabilities each library is allowed to have, both at runtime (to prevent malware injections) and at compile-time (to prevent supply-chain attacks). If the library code requires more capabilities than were given, the compiler would fail.

## Costs tracking (optional)

There are 4 types of costs: (space, time) ✕ (optimistic, pessimistic)
* "Time" means how many operations a function would perform.
* "Space" means how much memory a function would require.
* "Optimistic" means the best-case scenario: Shortest possible codepath for "time", or least memory allocated for "space".
* "Pessimistic" means the worst-case scenario. Longest possible codepath for "time", or most memory allocated for "space".

Time and Space may be independent of each other, but Optimistic should be cheaper than Pessimistic.

If the Pessimistic Time and/or Pessimistic Space aren't specified, the compiler should assume them to be Unbounded.

If the Optimistic Time and/or the Optimistic Space aren't specified, the compiler should assume them to be the same as their Pessimistic counterparts.

The cheaper cost is a subtype of the more expensive cost.

> [!IMPORTANT]
> The subtyping rule applies to the Optimistic class the exact same way as to the Pessimistic class, in the exact same direction, not opposite.
>
> This is because, contrary to human intuition, optimistic cost doesn't work like the lower-bound of a range type.

A cost is a kind of [effect](#effect-system).

### Level 0: Existence of Side-Effects

This is just a subset of [the Effect System](#effect-system).

This level should be trivially inferred by the compiler.

### Level 1: Absolute bounds

Only track the absolute maximum cost a function requires.

There are generally 3 classes of absolute bounds:
* Constant of Zero: The function does absolutely nothing, i.e. a no-op.
* Constant of Non-Zero: The function has a finite maximum cost.
* Unbounded: The cost of the function is not bounded.

This level should be trivially inferred by the compiler.

### Level 2: Bounds of Polynomial Expressions

> [!TIP]
> This type of bound depends on input, and as such, requires dependent-type.

This is a bound described by [polynomial expressions](https://en.wikipedia.org/wiki/Polynomial), not to be confused with [polynomial bounds](#level-3-polynomial-bounds).

In a non-generic or resolved function signature:
* The inputs and their sizes serve as indeterminates.
* The multiplier constants (a.k.a. factors) serve as coefficients.
* The operations or side-effects serve as units (a.k.a. dimensions).

This bound is described by polynomial expressions only. Sub-polynomial expressions (such as `log`, `sqrt`, etc) are not legal and must be over-approximated as a polynomial that contains it. As a consequence, this level would equate a binary-search and a linear-search as having almost the same cost.

Super-polynomials (such as `2**n`) are not legal and must be over-approximated into Unbounded.

The terms are independent of each other, despite how entangled they may have been in the actual code.

This level should be trivially inferred by the compiler.

### Level 3: Polynomial Bounds

> [!TIP]
> This type of bound depends on input, and as such, requires dependent-type.

This is a bound described by a mix of [polynomials](#level-2-bounds-of-polynomial-expressions) and sub-polynomials.

Super-polynomials (such as `2**n`) are not legal and must be "upgraded" into Unbounded.

Unlike [Level 2](#level-2-bounds-of-polynomial-expressions), this bound accept sub-polynomials.

The terms remain independent of each other, just like [Level 2](#level-2-bounds-of-polynomial-expressions).

This level cannot be trivially inferred by the compiler and thus require manually written proofs.

### Level 4: Exact bound expressions

> [!TIP]
> This type of bound depends on input, and as such, requires dependent-type.

This is a bound described by the granular cost expressions in exact details. This includes the entangled relationships between operations, super-polynomials, etc.

Such expressions are not necessarily linear. They may include structures such as branching (`if`, `match`) and loops.

This level cannot be trivially inferred by the compiler and thus requires manually written proofs.

This level cannot be trivially subtyped and thus requires manually written proofs.

## Type-theoretical proofs (optional)

**Goals:**
* Enhance correctness of programs, reduce logic bugs.
* Enable compiler optimization to be more aggressive, improving performance.

**Non-goals:**
* Proving Mathematical theorems.

**Potential applications:**
* Statically-checked pre- and post-assertions.
* Subtyping.
* Branch narrowing.

**Observed properties:**
* Propositions are just types. Proven propositions resolve to "true". Disproven propositions resolve to "false". Unproven propositions are unresolved.
* "For All" and "Exists" look like closures/templates. They could be implemented as wrapper of closures or templates. For example (non-final): `ForAll((x: nat) => GreaterThanOrEqual(x, 0))`, `Exists((a: nat, b: nat) => Equal(a * b, 12))`.

> [!TIP]
> Inspired by [Beyond Booleans](https://overreacted.io/beyond-booleans/).
