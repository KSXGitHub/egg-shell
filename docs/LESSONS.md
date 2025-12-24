# Lessons learned from others' failings

> [!NOTE]
> None of the syntax shown below are finalized. Treat them as pseudo-language.

## Hygiene of documentation's intra links

### Prior Art: Rust

Rust documentation has a feature called ["intra-doc link"](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html). It allows documentation to refer and link to items in scope by names. For example:

```rust
/// Link to [`Bar`].
pub struct Foo;

pub struct Bar;
```

The documentation for `Foo` above would contain a link to `Bar`. Both the documentation generator (`rustdoc`) and the language server (`rust-analyzer`) would recognize them.

#### Rough edge: Public docs refer to private items

The behavior of the tools diverge when `Bar` becomes private:
* `rust-analyzer` would still work like normal.
* `rustdoc` would warn about private items.

Whilst the behavior of `rustdoc` is still correct, `rust-analyzer` would fail to warn the user about invalid intra-doc links.

#### Rough edge: Doc-only imports require a different compilation context

Sometimes the user wants to refer to an item not currently in scope. The true solution is importing such an item with `#[cfg(doc)]`, for example:

```rust
#[cfg(doc)]
use std::collections::HashMap; // this item is only usable in documentation

/// Link to [`HashMap`].
pub struct Foo;
```

The problem, however, is that `rust-analyzer` doesn't recognize the `HashMap` intra-doc link as valid. It does not highlight it as it normally does for valid intra-doc links.

The problem was caused by `cfg` fundamentally being a different compilation context. So `rust-analyzer` and its users face a dilemma:
* To enable `cfg(doc)` by default would have allowed the code to wrongly use doc-only imports.
* To disable `cfg(doc)` by default would lessen the utility of documentation related IDE operations.
* Another option is to compile for both contexts at once, but this would increase the cost of memory and computation.

#### Rough edge: False "unused imports"

This is a problem shared by all things relating to `#[cfg(...)]` in general, not just `#[cfg(doc)]`.

The true solution is to add similar `#[cfg(...)]` to the import items. However, the compiler was unable to teach it to the user via hints due to it being in a different compilation context.

Furthermore, with test-only imports, the user can put everything inside a `#[cfg(test)] mod tests`. But with doc-only imports, the user has to write `#[cfg(doc)]` on every doc-only import items, making the code prone to documentation inconsistency down the line.

A good and quality codebase should have tests for documentation consistency (`RUSTDOCFLAGS='-D warnings' cargo doc`), but this is more of a workaround than a proper solution:
* It is not as convenient as immediate LSP feedback.
* When there are many features or compilation flags, the tests for documentation consistency would run into the same problem as regular tests and compilation: Combinatorial explosion of feature flags.

#### Rough edge: Inconsistent treatments of private items

This code would rightfully not work with `RUSTDOCFLAGS='-D warnings' cargo doc`:

```rust
pub struct Bar;

/// Link to [`Bar`].
pub struct Foo;
```

This code, despite the `use` statement being private however, would work:

```rust
use std::collections::HashMap;

/// Link to [`HashMap`].
pub struct Foo;
```

This code would either work or not work depending on the ultimate visibility of the linked item:

```rust
// lib/foo.rs

use super::bar::Bar; // `Bar` could either be fully visible to the public or not

/// Link to [`Bar`].
pub struct Foo;
```

The above demonstration shows a special case of the `use` item which is treated differently from the other items. Whilst this behavior isn't too complicated and quite intuitive to be fair, it still increases complexity and makes it more error-prone.

### Prior Art: TypeScript

TSDoc is the de-facto documentation format for TypeScript.

TSDoc uses a [`{@link}`](https://tsdoc.org/pages/tags/link/) tag to refer to other items.

```typescript
/** Link to {@link Bar} */
export class Foo {}

export class Bar {}
```

The above code is simple enough to work similarly in different document generation tools and TypeScript TSServer.

Unlike Rust which has different compilation contexts to deal with, TypeScript has only one compilation context. However, TypeScript still has rough edges of its own.

#### Rough Edge: Inconsistent tooling

Different tools have subtle differences in handling intra-doc links. TypeScript TSServer allows documentation of an exported or public item to link to a non-exported or private item. But a documentation generation tool cannot do that or it would create broken links.

#### Rough Edge: No doc-only import aliases for a long time

For a long time, doc-only import aliases didn't exist. The user has to either:
* Write an actual `import` statement which isn't used by the code nor the type checker.
* Repeat `import('path/to/module').ItemName` ad infinitum.

After a while, TSDoc finally introduced `@import` attribute, and the various tools race to support it. But since it was added very late,
* The problem of inconsistent tooling persists.
* Older TypeScript versions and tooling versions don't support it. This means that code that uses the new syntax would sacrifice backward-compatibility.

### Lessons and Ideas

#### Valid intra link targets

Documentation intra links should refer only to items whose visibility is the same or greater.
* Intra links from a private item can refer to any private item of the same scope, internal items, or public items.
* Intra links from an internal item can refer to any other internal or public items.
* Intra links from a public item can only refer to other public items.

#### Intra link forms

Documentation intra links should have the following forms:
* Absolute path: The absolute address of an item.
* Relative path: The address of an item, relative to the current file.
* Name: A special case of relative path, referring to an item of the same scope.

#### Doc-only import aliases

Doc-only import aliases allow the user to write good documentation with less effort, less text, less clutter.

Doc-only import aliases should be meta-attributes of the same item or containing item.

Intra links can use an alias declared by the same item.

```
@@import { Bar } from './bar.egg'
@@desc Link to [`Bar`].
pub struct Foo
```

Intra links can use an alias declared by the containing item.
* Intra links of an item can use an alias declared by the namespace containing that item, directly or indirectly.
* Intra links of a struct field can use an alias declared by the struct or the namespace containing the struct.
* Intra links of an enum variant can use an alias declared by the enum or the namespace containing the enum.
* Intra links of a trait item can use an alias declared by the trait or the namespace containing the trait.
* Intra links of a trait instance can use an alias declared by the instantiation  or the namespace containing the instantiation, not the trait itself though.
* and so on...

```
mod outer with
    @@!import { Bar } from './bar.egg'

    @@desc Link to [`Bar`].
    pub struct Foo with
        @@desc Another link to [`Bar`].
        pub foo: u64
```

#### Type-checked documentation's intra links

The compiler should check the validity of the intra links according to the rules laid out in regular compilation context. No special compilation flag or context should be necessary.
