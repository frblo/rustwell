use std::io::Write;

use typst_pdf::PdfOptions;

use crate::{export::typst::compile_document, screenplay::Screenplay};

pub fn export_pdf(screenplay: &Screenplay, mut writer: impl Write) {
    let compiled_doc = compile_document(screenplay);
    let pdf = typst_pdf::pdf(&compiled_doc, &PdfOptions::default()).expect("Error generating PDF");
    writer.write_all(&pdf).expect("Error writing PDF");
}
