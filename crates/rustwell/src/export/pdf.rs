use std::io::Write;

use typst_pdf::PdfOptions;

use crate::{export::typst::compile_document, screenplay::Screenplay};

/// Exports a `pdf` file and writes it to the provided writer. This is done by first constructing
/// a [typst] document and then, using [typst], exporting that to an actual `pdf` document.
pub fn export_pdf(screenplay: &Screenplay, mut writer: impl Write) {
    let compiled_doc = compile_document(screenplay);
    let pdf = typst_pdf::pdf(&compiled_doc, &PdfOptions::default()).expect("Error generating PDF");
    writer.write_all(&pdf).expect("Error writing PDF");
}
