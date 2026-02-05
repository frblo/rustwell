pub mod html;
pub mod pdf;
pub mod typst;

use std::{
    fs::File,
    io::{BufWriter, Result, Write},
    path::Path,
};

use crate::Screenplay;

pub trait Exporter {
    fn file_extension(&self) -> &'static str;

    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> Result<()>;
}

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

pub fn export_to_stdout<E>(exporter: &E, screenplay: &Screenplay) -> Result<()>
where
    E: Exporter + ?Sized,
{
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();
    exporter.export(screenplay, &mut lock)
}

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
