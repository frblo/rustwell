//! Traits for exporting [`Screenplay`]s
//!
//! This module defines the [`Exporter`] trait, which can be implented to provide
//! custom export methods. The [`Exporter`] trait is designed to be dyn-compatible
//! to allow end-application to implement dynamic pickers of exporters.
//!
//! # Implementing
//! [`Exporter::export`] returns [`std::io::Result`] and is generally expected
//! to only return Error when there is an issue with the writer.

pub mod html;
pub mod pdf;

use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use crate::Screenplay;

/// A [`Screenplay`] exporter.
///
/// Types Implementing this trait provide logic for exporting a [`Screenplay`]
/// to some format.
pub trait Exporter {
    fn file_extension(&self) -> &'static str;

    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> Result<()>;
}

/// An extension trait for [`Exporter`] providing common functionality.
///
/// There is a blanket implementation so any type implementing [`Exporter`] also
/// implements this trait.
///
/// Bring it into scope to get acess to the functions
pub trait ExporterExt: Exporter {
    fn export_to_stdout(&self, screenplay: &Screenplay) -> Result<()> {
        export_to_stdout(self, screenplay)
    }

    fn export_to_file(&self, screenplay: &Screenplay, path: impl AsRef<Path>) -> Result<()> {
        export_to_file(self, screenplay, path)
    }

    fn export_with_extension(
        &self,
        screenplay: &Screenplay,
        dir: impl AsRef<Path>,
        base: &str,
    ) -> Result<()> {
        export_with_extension(self, screenplay, dir, base)
    }
}

impl<T: Exporter + ?Sized> ExporterExt for T {}

/// Exports a [`Screenplay`] to stdout using the provided [`Exporter`]
pub fn export_to_stdout<E>(exporter: &E, screenplay: &Screenplay) -> Result<()>
where
    E: Exporter + ?Sized,
{
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    exporter.export(screenplay, &mut lock)
}

/// Exports a [`Screenplay`] to a specified file path using the provided [`Exporter`]
pub fn export_to_file<E>(
    exporter: &E,
    screenplay: &Screenplay,
    path: impl AsRef<Path>,
) -> Result<()>
where
    E: Exporter + ?Sized,
{
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    exporter.export(screenplay, &mut w)
}

/// Exports a [`Screenplay`] to the specified directory with the specified
/// base using the file extension from the provided [`Exporter`].
pub fn export_with_extension<E>(
    exporter: &E,
    screenplay: &Screenplay,
    dir: impl AsRef<Path>,
    base: &str,
) -> Result<()>
where
    E: Exporter + ?Sized,
{
    let path = dir
        .as_ref()
        .join(base)
        .with_extension(exporter.file_extension());
    export_to_file(exporter, screenplay, &path)
}
