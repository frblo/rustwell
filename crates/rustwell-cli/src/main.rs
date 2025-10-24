use clap::{Parser, ValueEnum};
use color_eyre::Result;
use color_eyre::eyre::bail;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
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
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Target {
    Typ,
    Html,
    Pdf,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let mut reader = decide_reader(&cli)?;
    let target = decide_target(&cli);
    let mut writer = decide_writer(&cli, &target)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let screenplay = rustwell::parse(&buf);
    match target {
        Target::Html => rustwell::export_html(&screenplay, &mut writer, true),
        _ => unimplemented!("These aren't in main yet"),
    }

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
        "typ" => Target::Typ,
        "html" | "htm" => Target::Html,
        "pdf" => Target::Pdf,
        _ => bail!("unkown extension '.{}'; specify -t/--target", ext),
    };

    Ok(t)
}

fn decide_writer(cli: &Cli, target: &Target) -> Result<Box<dyn Write>> {
    if cli.stdout || cli.out.as_deref().unwrap_or_default() == "-" {
        Ok(Box::new(BufWriter::new(io::stdout())))
    } else if let Some(path) = cli.out.as_deref() {
        let f = File::create(path)?;
        Ok(Box::new(BufWriter::new(f)))
    } else {
        let path = Path::new(detect_name_from_path(&cli.src)?)
            .with_extension(extension_from_target(target));
        let f = File::create(path)?;
        Ok(Box::new(BufWriter::new(f)))
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

fn extension_from_target(target: &Target) -> &'static str {
    match target {
        Target::Typ => "typ",
        Target::Html => "html",
        Target::Pdf => "pdf",
    }
}
