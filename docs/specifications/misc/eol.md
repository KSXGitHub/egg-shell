# End-of-Line

## Requirements

End-of-Line (EOL) marks the end of one line and the start of another line, reset `col` to 1 and increase `line` by 1.

Character coordinates must match both VS Code and the Linux Terminal.

There are conflicting definition of EOL between VS Code and the Linux Terminal:
* VS Code considers Carriage Return (CR) that isn't followed by a Line Feed (LF) a valid EOL.
* The Terminal only sees LF and CRLF as valid EOLs.

The language parser therefore, must find a subset that is consistent with both VS Code and the Terminal.

## Specification

* End-of-Line (EOL) marks the end of one line and the start of another line, reset `col` to 1 and increase `line` by 1.
* LF and CRLF are EOLs.
* CR without being immediately followed by LF is invalid.
