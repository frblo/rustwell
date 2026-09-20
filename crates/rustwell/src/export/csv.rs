use std::io::Write;

use crate::{
    export::Exporter, rich_string::RichString, screenplay::Screenplay, statistics::Statistics,
};

/// A [`Screenplay`] exporter for `CSV`
///
/// The variables configure the exporter
#[derive(Default)]
pub struct CsvExporter {}

impl Exporter for CsvExporter {
    fn file_extension(&self) -> &'static str {
        "csv"
    }

    fn export(&self, screenplay: &Screenplay, writer: &mut dyn Write) -> std::io::Result<()> {
        let statistics = Statistics::new(screenplay);
        writeln!(writer, "Name, Number of words, Number of lines",)
            .expect("Failed to write csv header");
        for (scene_idx, rich_scene_name) in statistics.scene_names.iter().enumerate() {
            writeln!(writer, "{}", escape_rich_string(rich_scene_name.clone()))
                .expect("Failed to write scene name");

            for (rich_character_name, stats) in statistics.characters_stats_in_scene(scene_idx) {
                writeln!(
                    writer,
                    "{}, {}, {}",
                    escape_rich_string(rich_character_name),
                    stats.words_count,
                    stats.lines_count
                )
                .expect("Failed to write character statistics");
            }
        }
        Ok(())
    }
}

fn escape_rich_string(rs: RichString) -> String {
    let s = rs.to_string();
    format!("\"{}\"", s.replace("\"", "\"\""))
}
