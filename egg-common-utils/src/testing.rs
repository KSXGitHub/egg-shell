use command_extra::CommandExtra;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{
    fs,
    io::{self, Write},
    process::{Command, Output},
};
use tempfile::tempdir;

/// Create a diff between two strings.
#[derive(Debug, Clone, Copy)]
pub struct UniDiff<Left, Right>(pub Left, pub Right);

/// Error returned by [`UniDiff::exec`].
#[derive(Debug, Display, Error)]
pub enum UniDiffExecError {
    #[display(fmt = "Failed to create temporary directory: {_0}")]
    Workspace(io::Error),
    #[display(fmt = "Failed to create temporary file for left: {_0}")]
    Left(io::Error),
    #[display(fmt = "Failed to create temporary file for right: {_0}")]
    Right(io::Error),
    #[display(fmt = "Failed to execute 'diff': {_0}")]
    Exec(io::Error),
}

impl<Left, Right> UniDiff<Left, Right>
where
    Left: AsRef<str>,
    Right: AsRef<str>,
{
    /// Execute the command `diff -u` on the two strings.
    pub fn exec(&self) -> Result<Option<String>, UniDiffExecError> {
        let UniDiff(left_text, right_text) = self;

        let workspace = tempdir().map_err(UniDiffExecError::Workspace)?;
        let left_file = workspace.path().join("left");
        let right_file = workspace.path().join("right");

        fs::write(&left_file, left_text.as_ref()).map_err(UniDiffExecError::Left)?;
        fs::write(&right_file, right_text.as_ref()).map_err(UniDiffExecError::Right)?;

        let output = Command::new("diff")
            .with_current_dir(&workspace)
            .with_arg("--color=always")
            .with_arg("-u")
            .with_arg("left")
            .with_arg("right")
            .output()
            .map_err(UniDiffExecError::Exec)?;
        let Output {
            status,
            stdout,
            stderr,
        } = output;

        io::stderr().write_all(&stderr).pipe(drop);

        if status.success() {
            if stdout.is_empty() {
                return Ok(None);
            }

            dbg!(&stdout);
            dbg!(String::from_utf8_lossy(&stdout));
            panic!("The diff command return success status but its stdout isn't empty");
        }

        stdout
            .pipe(String::from_utf8)
            .expect("The stdout of the diff command should be valid UTF-8")
            .pipe(Some)
            .pipe(Ok)
    }

    /// Assert that two strings are equal.
    pub fn assert_eq(&self) {
        match self.exec() {
            Ok(None) => { /* assertion passed, do nothing */ }
            Ok(Some(diff)) => panic!("assertion failed: `(left == right)`\n{diff}"),
            Err(_) => pretty_assertions::assert_str_eq!(self.0.as_ref(), self.1.as_ref()),
        }
    }
}

/// Assert that two strings are equal.
pub fn assert_eq_uni_diff<Left, Right>(left: Left, right: Right)
where
    Left: AsRef<str>,
    Right: AsRef<str>,
{
    UniDiff(left, right).assert_eq()
}
