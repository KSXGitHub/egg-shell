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
