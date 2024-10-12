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

## Location agnostic compilation cache

### Problems

We would like to store cache based on the content hash of the files. The problem is, a module file may have dependencies, and the relative positions between the importer and the imported also change the way the program work.

### Potential solutions

#### Make relative path a part of a dependency identity

Cache the relative path as part of a file's dependencies tree. Together with a content hash, make up a pair that is the identity of a dependency.
