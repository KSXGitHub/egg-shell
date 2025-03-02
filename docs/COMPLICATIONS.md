# Complications

## Namespaces or locales for field names for unordered anonymous tuples

### Problems

#### Translation

Field names for different anonymous tuples can have entirely different meaning (homographs) and thus cannot be translated consistently across different locales.

#### Type representation

Field names cannot be represented by a single literal value, making its type representation involving field names (such as `Pick`, `Omit`, `HasField`, etc) difficult to design.

#### Spread and merge

Related to _Type representation_ above. Merging homographs would lead to surprising results.

#### ABI consistency

Field names are sorted differently across different namespaces and thus cannot have a consistent stable ABI if implemented naively.

### Potential solutions

#### Vocabulary

Vocabulary is a special type that can be inserted in struct and anonymous tuples as field names. A named struct would also have an associated vocabulary.

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

## Trait instantiations on higher-kinded types with varying number of parameters

### Problems

Higher-kinded types may have varying number of parameters (e.g. `Option(X)` requires 1 whilst `Result(X, E)` requires 2). This makes fitting them into a common trait (such as `Functor` which is required for both `Option` and `Result`) difficult.

### Potential solutions

#### Allow trait instantiations to be on partially applied type aliases

Trait instantiations on some classes of type aliases must be allowed.

These type aliases can either be named (`type F(X) = G(A, X, C)`) or not named. The legal not-named type aliases are so-called "type closures" (shorthand: `G(A, ?, C)`, long-form hasn't been thought out).

These type aliases must have a single identifiable type template (such as `Result` for `type FsResult(X) = Result(X, FsError)`). Therefore, type aliases with branching (such as `type Choose(choice: bool, T, F) = if choice then T else F`) are forbidden.

The orphan rules must forbid trait instantiations of one trait on multiple type aliases which share the same type template. For example: `inst(E) Functor(Result(?, E))` must conflict with `inst(X) Functor(Result(X, ?))` because `Result(?, E)` and `Result(X, ?)` share `Result` as the type template.

Trait instantiations on functions that return types are still forbidden.

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
