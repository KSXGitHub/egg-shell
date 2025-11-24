# Complications

> [!NOTE]
> None of the syntax shown below are finalized. Treat them as pseudo-language.

## Vocabularies for field names for unordered anonymous tuples

### Problems

#### Translation

Field names for different anonymous tuples can have entirely different meaning (homographs) and thus cannot be translated consistently across different vocabularies.

#### Type representation

Field names cannot be represented by a single literal value, making its type representation involving field names (such as `Pick`, `Omit`, `HasField`, etc) difficult to design.

#### Spread and merge

Related to _Type representation_ above. Merging homographs would lead to surprising results.

#### ABI consistency

Field names are sorted differently across different vocabularies and thus cannot have a consistent stable ABI if implemented naively.

### Potential solutions

#### Word set

Word set is a special type whose statically-known values (e.g. compile-time constants) can be inserted in struct and anonymous tuples as field names. A named struct would also have an associated word set.

#### Wrapper type with combination properties

Combine value wrappers instead of field names. The meaning of the "field names" can be derived from the name of the wrapper types.

#### Anonymous product of resolved types

Combine resolved type directly. If some types are found to be duplicated or too generic (such as plain numbers and strings), define a wrapper type.

Generic type cannot be combined this way.

## Hidden trait bound doesn't mix well with first-class trait bound as expression

### Problems

Hidden trait bounds are trait bounds that are automatically assumed by default. Their existence necessitate a syntax to remove it (i.e. `?Trait(Types)`). This syntax however does not make sense in a normal expression.

### Potential solutions

#### Syntax to disable hidden trait bounds is only valid in trait declarations and trait instances

Expressions do not contain hidden trait bounds. The result of such expressions do not assume hidden trait bounds. Instead, they will be assumed once these expressions are invoked in trait declarations and trait instances by appending hidden trait bounds to the result of the invoked expressions.

## "Forall" trait bounds

### Problems

Some trait bound expressions may require the ability to specify "forall" of generic parameters (e.g. lifetime parameters for function parameters of a function trait bounds, similar to `for<>` in Rust).

### Potential solutions

#### Template syntax

Template syntax can function as a "forall" expression.

It requires an additional ability to seamlessly type-cast between template types of similar set of template parameters but in different order in order to mitigate/eliminate the needs of "wrapper" template closure.

The template parameters must be bound the lowest-level signature (to either an argument or the return type) either directly or indirectly (via other template parameters) to enable type-cast. Unbounded template parameters (essentially arguments in disguise) cannot be type-casted.

## Trait instantiations on higher-kinded types with varying number of parameters

### Problems

Higher-kinded types may have varying number of parameters (e.g. `Option(X)` requires 1 whilst `Result(X, E)` requires 2). This makes fitting them into a common trait (such as `Functor` which is required for both `Option` and `Result`) difficult.

### Potential solutions

#### Allow trait instantiations to be on partially applied type aliases

Trait instantiations on some classes of type aliases must be allowed.

These type aliases can either be named (`type F(X) = G(A, X, C)`) or not named. The legal not-named type aliases are so-called "type closures" (shorthand: `G(A, ?, C)`, long-form hasn't been thought out).

These type aliases must have a single identifiable [type root](#type-root). Therefore, [type aliases with branching](#type-aliases-with-branching) are forbidden.

The orphan rules must forbid trait instantiations of one trait on multiple type aliases which share the same type root. For example: `inst(E) Functor(Result(?, E))` must conflict with `inst(X) Functor(Result(X, ?))` because `Result(?, E)` and `Result(X, ?)` share `Result` as the type root.

Trait instantiations on [functions that return types](#functions-that-return-types) are forbidden.

Trait instantiations on [opaque type expressions](#opaque-type-expressions) are forbidden.

##### Type Root

The type root is the canonical, originally-defined type constructor that serves as the foundational element in a non-branching type expression or type alias.
It represents the ultimate source type from which all aliases derive their identity for trait resolution purposes.

A type root must be:
* The original type name (for named types) or structure (for structural types) as defined in its declaration.
* Not itself an alias of another type.
* Unambiguously determinable through syntactic inspection.

For example:
* The type root of `enum Result` is `Result`.
* The type root of `type FsResult(X) = Result(X, FsError)` is `Result`.
* The type root of `type FallibleString(E) = Result(String, E)` is `Result`.
* The type root of `type ReversedResult(E, X) = Result(X, E)` is `Result`.
* The type root of `type ComplexForNoReason(X) = Either(Option(X), Result(X, DynError))` is `Either`.
* The type root of `struct Foo` (no generic parameter) is `Foo`.
* The type root of `(u32, String, char)` would be either `Tuple` or `Tuple3` once the design is finalized.
* The type root of `[u32; 5]` and `[Foo; 1]` is `Array`.
* The type root of `[u32]` and `[Foo]` is `Slice`.
* The type root of `&str`, `&Foo`, and `&&Foo` is `Reference`.

##### Type aliases with branching

A type alias with branching is a type-level expression that conditionally selects between two or more different type expressions based on compile-time conditions, preventing the identification of a single type root.

For example: `type Choose(choice: bool, T, F) = if choice then T else F`.

##### Functions that return types

All type aliases are type functions, but not all type functions qualified as type aliases.

##### Opaque type expressions

An opaque type expression is a type-level computation where the relationship between input types and output type is not transparently visible to the trait resolution system, preventing identification of a type root

For example:
* `type Computed = some_const_function_that_return_type()`
* `type Mapped(types: List(type), f: fn(type) -> type) = map(types, f)`
* `type Reduced(Init, types: List(types), f: fn(type, type) -> type) = fold(Init, types, f)`

## Location agnostic compilation cache

### Problems

We would like to store cache based on the content hash of the files. The problem is, a module file may have dependencies, and the relative positions between the importer and the imported also change the way the program work.

### Potential solutions

#### Make relative path a part of a dependency identity

Cache the relative path as part of a file's dependencies tree. Together with a content hash, make up a pair that is the identity of a dependency.

## Subtypes and traits

### Problems

Trait methods may (and often does) have return type being the same as the type parameters in of the trait. This makes it possible to return a value belongs to the type but not a subtype, leading to wrong subtype guarantees should the subtype inherit traits from the its supertypes.

For example, the number type `i32` has trait instance `Add(i32, i32)` and a subtype `1i32 | 2i32` (the syntax of subtyping is not final). Logically, `Add(i32, i32)` should be possible on `1i32 | 2i32`. However, `2i32 + 2i32` evaluates to `4i32` which is outside the subtype.

### Potential solutions

#### Subtypes are not considered the same as their supertypes

Subtypes should not be considered the same as their supertypes, and they do not inherit their trait instances. Instead, their values would be casted to the closet supertype that satisfy the trait instance.

For example, calling `Add(i32, i32)::add` on 2 values of subtype `1i32 | 2i32` would cast the values to `i32`, compute the addition, then return an `i32`.

## Subtypes and blanket traits

### Unresolved questions

Is it possible to make unsound method with blanket traits?

## Turing-complete const expressions and proof-system

> [!NOTE]
> This problem is only relevant when the Type-theoretical Proofs is implemented.

### Problems

Const expressions are allowed to be used in type expressions.

Type expressions are the mechanism of type-theoretical proofs.

In Type-theoretical Proofs, proofs and types cannot be cleanly separated.

Turing-completeness introduces undecidability and non-termination, which could enable writing false proofs such as `eq(0, 1)`. Such false proofs could be used to subtly or accidentally introduce unsoundness, undefined behavior, and logic bugs to the program without the presence of a single escape hatch (such as `unsafe`).

### Potential solutions

#### Introduce an effect that specifies totality

Const expressions without termination guarantee cannot be invoked from within termination-guaranteed contexts.

The const expressions used in a type context must guarantee to terminate.

The problems with this solution:
1. It restricts all legitimate uses of Turing-complete type expressions in non-proof type signatures.

#### Introduce a tier purer than `const` called `type`

All type-level expressions can be invoked from within const expressions.

The reverse isn't true.

The problems with this solution:
1. Too restrictive.
2. It either overloads the meaning of an existing keyword (`type`) or adds a new one, making the language more complex.

#### Identify what kinds of proof operations that actually need totality

Surely the paradoxes only arise when certain operations are enabled! Only such operations would require the `total` effect to be used in type context.

The problems with this solution:
1. It costs time to research.
2. A bottom-up approach may leave holes.

Alternatively, all new operations required by advanced proofs (when Type-theoretical Proofs is implemented) should require `total`. If future research finds out that the requirement was unnecessary, lifting the `total` requirement would not be a breaking change.

#### Introduce a special proof-irrelevant universe of propositions

Introduce a special type universe called `Prop` (like Lean). This universe is lower than that of the data types.

Inhabitants of the types in this universe (called "propositions") cannot be used in most computational contexts. Most of standard library functions aren't polymorphic over this universe. This renders the problem of Turing-completeness irrelevant.

The problems with this solution:
1. It contradicts uniformity and symmetry, defeating the entire point.
2. It creates a separate world of proofs, which speaks a language different from the computational world.
3. It creates a need for a separate standard library and a separate package ecosystem only for proofs.
4. It sucks.
