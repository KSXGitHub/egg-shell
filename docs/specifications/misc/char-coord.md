# Coordinate of Characters in Source Code

## Requirements

Character coordinates must match that of VS Code, so that when user types the `filepath:ln:col` that they see into the Quick File Picker, VS Code takes them right to the correct line and column.

VS Code numbers the first line as 1 and the first column as 1.

VS Code counts columns by [unicode scalar values][unicode scalar value], as a consequence, "❤️" is counted as 2 columns instead of just 1.

Although VS Code counts TAB by tab size in the status bar, it's still 1 when <kbd>Alt+Click</kbd> on links.

## Specification

* Each line in a document may be separated by `LF` or `CRLF`.
* The first line is numbered 1.
* Subsequent lines are numbered `prev + 1` where `prev` is the number of the line right before.
* Each column (character) in a line is determined by a [unicode scalar value].
* The first column is numbered 1.
* Subsequent columns are numbered `prev + 1` where `prev` is the number of the column right before.

[unicode scalar value]: https://www.unicode.org/glossary/#unicode_scalar_value
