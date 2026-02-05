use std::io::Write;

use typst_pdf::PdfOptions;

use crate::{Exporter, export::typst::TypstExporter, screenplay::Screenplay};

#[derive(Default)]
pub struct PdfExporter {
    pub synopses: bool,
}

impl Exporter for PdfExporter {
    fn file_extension(&self) -> &'static str {
        "pdf"
    }

    /// Exports a `pdf` file and writes it to the provided writer. This is done by first constructing
    /// a [typst] document and then, using [typst], exporting that to an actual `pdf` document.
    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        let compiled_doc = TypstExporter {
            synopses: self.synopses,
        }
        .compile_document(screenplay)?;
        let pdf = typst_pdf::pdf(&compiled_doc, &PdfOptions::default())
            .map_err(|_| std::io::Error::other("failed to create typst pdf"))?;
        writer.write_all(&pdf)
    }
}
