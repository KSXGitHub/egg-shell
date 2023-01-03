use crate::Ordinal;
use derive_more::{DebugCustom, Display, From, Into};
use std::num::NonZeroUsize;

macro_rules! def_type {
    (
        $(#[$top_attrs:meta])*
        $name:ident

        $(#[$from_pred_count_attrs:meta])*
        $from_pred_count_name:ident

        $(#[$value_attrs:meta])*
        $value_name:ident

        $(#[$pred_count_attrs:meta])*
        $pred_count_name:ident

        $(#[$advanced_by_attrs:meta])*
        $advance_by_name:ident

        $(#[$try_retreat_by_attrs:meta])*
        $try_retreat_by_name:ident
    ) => {
        $(#[$top_attrs])*
        #[derive(DebugCustom, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Into)]
        #[debug(fmt = "{name} {_0}", name = stringify!($name))]
        pub struct $name(Ordinal);

        impl $name {
            $(#[$from_pred_count_attrs])*
            pub const fn $from_pred_count_name(pred_count: usize) -> Self {
                $name(Ordinal::from_pred_count(pred_count))
            }

            $(#[$value_attrs])*
            pub const fn $value_name(self) -> NonZeroUsize {
                self.0.value()
            }

            $(#[$pred_count_attrs])*
            pub const fn $pred_count_name(self) -> usize {
                self.0.pred_count()
            }

            $(#[$advanced_by_attrs])*
            pub const fn $advance_by_name(self, steps: usize) -> Self {
                let $name(ordinal) = self;
                $name(ordinal.advance_by(steps))
            }

            $(#[$try_retreat_by_attrs])*
            pub fn $try_retreat_by_name(self, steps: usize) -> Option<Self> {
                self.0.try_retreat_by(steps).map($name)
            }
        }
    };
}

def_type! {
    /// Position of a line.
    ///
    /// The position of the first line is 1.
    LnNum

    /// Create a line number from the number of preceding lines (pred_count).
    ///
    /// Line number is always equal to `pred_count + 1`, so `from_pred_count(0)` would return line 1.
    from_pred_count

    /// Get the value of the line number.
    value

    /// Number of preceding lines.
    ///
    /// This number is always equal `ln - 1`.
    pred_count

    /// Advance the line number.
    advance_by

    /// Try retreat the line number.
    ///
    /// Return `None` if overflow occurred.
    try_retreat_by
}

def_type! {
    /// Position of a character in a line.
    ///
    /// The position of the first character is 1.
    ColNum

    /// Create a column number from the number of preceding columns (pred_count).
    ///
    /// Column number is always equal to `pred_count + 1`, so `from_pred_count(0)` would return column 1.
    from_pred_count

    /// Get the value of the column number.
    value

    /// Number of preceding characters in a line.
    ///
    /// This number is always equal `col - 1`.
    pred_count

    /// Advance the column number.
    advance_by

    /// Try retreat the line number.
    ///
    /// Return `None` if overflow occurred.
    try_retreat_by
}

/// Coordinate of a character.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
#[display(fmt = "{line}:{column}")]
pub struct CharCoord {
    /// Line number of the character.
    pub line: LnNum,
    /// Column number of the character.
    pub column: ColNum,
}

impl CharCoord {
    /// Create a character coordinate.
    pub const fn new(line: LnNum, column: ColNum) -> Self {
        CharCoord { line, column }
    }

    /// Create a character coordinate from line and column predecessor counts.
    pub const fn from_pred_counts(ln_pred: usize, col_pred: usize) -> Self {
        Self::new(
            LnNum::from_pred_count(ln_pred),
            ColNum::from_pred_count(col_pred),
        )
    }

    /// Advance by a number of lines.
    pub const fn advance_line(mut self, steps: usize) -> Self {
        self.line = self.line.advance_by(steps);
        self
    }

    /// Advance by a number of columns.
    pub const fn advance_column(mut self, steps: usize) -> Self {
        self.column = self.column.advance_by(steps);
        self
    }
}
