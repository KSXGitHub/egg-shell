use super::{LazyCharGridData, LoadingProgress};
use crate::{char_grid::CharGridLineTemplate, text_slice::ScanText, EndOfLine, LnCol};
use assert_cmp::debug_assert_op;
use derive_more::{Display, Error};
use pipe_trait::Pipe;

/// Success value of [`LazyCharGrid::load_char`](super::LazyCharGrid::load_char).
#[derive(Debug, Clone, Copy)]
pub enum LoadCharReport {
    /// The grid is completed.
    Document,
    /// Complete a line.
    Line(CharGridLineTemplate),
    /// Get another character.
    Char(char),
}

/// Failure value of [`LazyCharGrid::load_char`](super::LazyCharGrid::load_char).
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Error)]
pub enum LoadCharError<IterError> {
    /// Encounter an invalid character.
    #[display(fmt = "CR is poorly placed, it was before {followed_by} instead of LF")]
    IllPlacedCarriageReturn { followed_by: char },
    /// Error emitted by character iterator.
    IterError(IterError),
}

impl<IterError, CharIter: Iterator<Item = Result<char, IterError>>> LazyCharGridData<CharIter> {
    /// Add another character to the grid.
    pub(super) fn load_char(&mut self) -> Result<LoadCharReport, LoadCharError<IterError>> {
        let LazyCharGridData {
            loaded_text,
            loaded_char_list,
            loaded_eol_list,
            completion_progress,
        } = self;

        let Some(LoadingProgress {
            src_char_iter,
            prev_non_lf,
            prev_line_offset,
        }) = completion_progress else {
            return Ok(LoadCharReport::Document);
        };

        let Some(char) = src_char_iter.next() else {
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..];
            let (start, end) = ScanText::run(ScanText {
                char_list: loaded_char_list,
                src_text: line_src_text,
                first_char_coord: LnCol::from_pred_counts(loaded_eol_list.len(), 0),
                offset: line_offset,
                eol: EndOfLine::EOF
            });
            // loaded_eol_list.push((end, EndOfLine::EOF)); // TODO: should this line be removed?
            loaded_char_list.shrink_to_fit(); // The list is final (no more changes), it is safe to shrink to free some memory
            loaded_eol_list.shrink_to_fit(); // The list is final (no more changes), it is safe to shrink to free some memory
            *completion_progress = None;
            return Ok(LoadCharReport::Document);
        };

        let char = char.map_err(LoadCharError::IterError)?;

        let current_byte_offset = loaded_text.len();
        loaded_text.push(char);

        if char == '\n' {
            let last_char = *prev_non_lf;
            let (eol_offset, eol) = if last_char == Some('\r') {
                debug_assert_op!(current_byte_offset > 0);
                (current_byte_offset - 1, EndOfLine::CRLF)
            } else {
                (current_byte_offset, EndOfLine::LF)
            };
            let line_offset = *prev_line_offset;
            let line_src_text = &loaded_text[line_offset..eol_offset];
            let first_char_coord = LnCol::from_pred_counts(loaded_eol_list.len(), 0);
            let (start, end) = ScanText::run(ScanText {
                char_list: loaded_char_list,
                src_text: line_src_text,
                first_char_coord,
                offset: line_offset,
                eol,
            });
            loaded_eol_list.push((end, eol));
            *prev_non_lf = None;
            *prev_line_offset = loaded_text.len();
            let template = CharGridLineTemplate {
                first_char_coord,
                start,
                end,
                eol,
                grid: (),
            };
            template.pipe(LoadCharReport::Line).pipe(Ok)
        } else {
            if *prev_non_lf == Some('\r') {
                dbg!(loaded_text);
                return Err(LoadCharError::IllPlacedCarriageReturn { followed_by: char });
            }
            *prev_non_lf = Some(char);
            char.pipe(LoadCharReport::Char).pipe(Ok)
        }
    }
}
