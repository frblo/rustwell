use clap::{Parser, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::bail;
use rustwell::Exporter;
use rustwell::ExporterExt;
use rustwell::HtmlExporter;
use rustwell::PdfExporter;
use rustwell::Screenplay;
use rustwell::TypstExporter;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rustwell", version, about = "A Fountain compiler CLI")]
struct Cli {
    /// Source file (use '-' for stdin)
    src: String,

    /// Output path (use '-' for stdout)
    #[arg(short = 'o', long = "out")]
    out: Option<String>,

    /// Explicit target format (overrides detection)
    #[arg(short = 't', long = "target", value_enum)]
    target: Option<Target>,

    /// Alias for stdout (same as `-o -`)
    #[arg(long = "stdout")]
    stdout: bool,

    /// Include synopses in output document
    #[arg(short = 's', long = "synopses", default_value_t = false)]
    synopses: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Target {
    Typst,
    Html,
    Pdf,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let exporter = decide_exporter(&cli);
    let mut reader = decide_reader(&cli)?;

    let screenplay = rustwell::parse_reader(&mut reader)?;
    export(&cli, &screenplay, exporter)?;

    Ok(())
}

fn decide_reader(cli: &Cli) -> Result<Box<dyn Read>> {
    if cli.src == "-" {
        Ok(Box::new(io::stdin()))
    } else {
        let file = File::open(&cli.src)?;
        Ok(Box::new(BufReader::new(file)))
    }
}

fn decide_exporter(cli: &Cli) -> Box<dyn Exporter> {
    let target = decide_target(cli);
    match target {
        Target::Html => Box::new(HtmlExporter {
            standalone: true,
            synopses: cli.synopses,
            include_source_positions: false,
        }),
        Target::Pdf => Box::new(PdfExporter {
            synopses: cli.synopses,
        }),
        Target::Typst => Box::new(TypstExporter {
            synopses: cli.synopses,
        }),
    }
}

fn decide_target(cli: &Cli) -> Target {
    cli.target
        .or_else(|| {
            let out_str = if cli.stdout {
                Some("-")
            } else {
                cli.out.as_deref()
            };

            out_str.and_then(|s| detect_target_from_path(s).ok())
        })
        .unwrap_or(Target::Pdf)
}

fn detect_target_from_path(path: &str) -> Result<Target> {
    if path == "-" {
        bail!("cannot detect target from stdout ('-'); specify -t/--target")
    }

    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    let t = match ext.as_str() {
        "typ" => Target::Typst,
        "html" | "htm" => Target::Html,
        "pdf" => Target::Pdf,
        _ => bail!("unkown extension '.{}'; specify -t/--target", ext),
    };

    Ok(t)
}

fn export(cli: &Cli, screenplay: &Screenplay, exporter: Box<dyn Exporter>) -> Result<()> {
    if cli.stdout || cli.out.as_deref().unwrap_or_default() == "-" {
        Ok(exporter.export_to_stdout(screenplay)?)
    } else if let Some(path) = cli.out.as_deref() {
        Ok(exporter.export_to_file(screenplay, path)?)
    } else {
        let dir = Path::new("");
        let base = detect_name_from_path(&cli.src)?;
        Ok(exporter.export_with_extension(screenplay, dir, base)?)
    }
}

fn detect_name_from_path(path: &str) -> Result<&str> {
    if path == "-" {
        bail!("cannot detect name from stdout ('-')")
    }

    Ok(Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default())
}
