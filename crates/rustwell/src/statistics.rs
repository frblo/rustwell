use std::collections::HashMap;

use crate::{
    Screenplay,
    rich_string::RichString,
    screenplay::{Dialogue, DialogueElement, Element},
};

/// Contains all tracked statistics for a [`Screenplay`].
pub struct Statistics {
    /// Keeps track of all characters present in the [`Screenplay`], with their assigned index.
    characters: HashMap<RichString, usize>,
    /// A [`HashMap`] for each scene with related [`CharacterStats`].
    scenes: Vec<HashMap<usize, CharacterStats>>,
    /// The names of scenes in the [`Screenplay`].
    pub scene_names: Vec<RichString>,
}

/// Keeps track of statistics related to a character within some scope in a [`Screenplay`].
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct CharacterStats {
    /// The number of lines (dialogues) a characters has.
    pub lines_count: usize,
    /// The number of words a characters says.
    pub words_count: usize,
}

impl Statistics {
    /// Creates a new [`Statistics`] from a [`Screenplay`].
    ///
    /// This function does `clone` some [`RichString`]s.
    pub fn new(screenplay: &Screenplay) -> Self {
        let mut characters = HashMap::new();
        let mut scenes = vec![HashMap::new()];
        let mut scene_names = Vec::new();
        let mut scene_idx = 0;

        for e in &screenplay.elements {
            match &**e {
                Element::Heading { slug, number: _ } => {
                    scenes.push(HashMap::new());
                    scene_names.push(slug.clone());
                    scene_idx += 1;
                }
                Element::Dialogue(dialogue) => handle_dialogue(
                    dialogue,
                    &mut characters,
                    scenes.get_mut(scene_idx).unwrap(),
                ),
                Element::DualDialogue(dialogue1, dialogue2) => {
                    handle_dialogue(
                        dialogue1,
                        &mut characters,
                        scenes.get_mut(scene_idx).unwrap(),
                    );
                    handle_dialogue(
                        dialogue2,
                        &mut characters,
                        scenes.get_mut(scene_idx).unwrap(),
                    );
                }
                _ => continue,
            }
        }

        Statistics {
            characters,
            scenes,
            scene_names,
        }
    }

    /// The number of scenes in the [`Screenplay`].
    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }

    /// The number of characters in the [`Screenplay`].
    pub fn character_count(&self) -> usize {
        self.characters.len()
    }

    /// The names of all characters in the [`Screenplay`], in a stochastic order
    pub fn characters(&self) -> Vec<&RichString> {
        self.characters.keys().collect()
    }

    /// Returns the [`CharacterStats`] of a certain character in a particular scene.
    ///
    /// Returns [`None`] if the character or the scene does not exist, or if the character is not
    /// present in the scene.
    pub fn character_stats_in_scene(
        &self,
        name: &RichString,
        scene_idx: usize,
    ) -> Option<CharacterStats> {
        if let Some(character_idx) = self.characters.get(name)
            && let Some(scene) = self.scenes.get(scene_idx)
            && let Some(stats) = scene.get(character_idx)
        {
            Some(*stats)
        } else {
            None
        }
    }

    /// Returns the [`CharacterStats`] of all characters present in a particular scene.
    pub fn characters_stats_in_scene(
        &self,
        scene_idx: usize,
    ) -> HashMap<RichString, CharacterStats> {
        if let Some(scene) = self.scenes.get(scene_idx) {
            let mut map = HashMap::with_capacity(scene.len());
            for (name, character_idx) in &self.characters {
                if let Some(stats) = scene.get(&character_idx) {
                    map.insert(name.clone(), *stats);
                }
            }
            map
        } else {
            HashMap::new()
        }
    }

    /// Returns the global [`CharacterStats`] of a character in the entirety of the [`Screenplay`].
    pub fn total_character_stats(&self, name: &RichString) -> Option<CharacterStats> {
        if let Some(character_idx) = self.characters.get(name) {
            let mut character_stats = CharacterStats::default();

            for scene in self.scenes.iter() {
                match scene.get(character_idx) {
                    Some(CharacterStats {
                        lines_count,
                        words_count,
                    }) => {
                        character_stats.lines_count += *lines_count;
                        character_stats.words_count += *words_count;
                    }
                    None => (),
                };
            }
            Some(character_stats)
        } else {
            None
        }
    }
}

fn handle_dialogue(
    dialogue: &Dialogue,
    characters: &mut HashMap<RichString, usize>,
    scene: &mut HashMap<usize, CharacterStats>,
) {
    let name = &dialogue.character;
    let character_idx = match characters.get(name) {
        Some(i) => *i,
        None => {
            let i = characters.len();
            characters.insert(name.clone(), i);
            i
        }
    };

    let stats = scene.entry(character_idx).or_default();
    stats.lines_count += 1;

    for element in &dialogue.elements {
        if let DialogueElement::Line(line) = &element.inner {
            stats.words_count += count_words(line);
        }
    }
}

fn count_words(text: &RichString) -> usize {
    let mut count = 0;
    let mut in_word = false;
    for c in text.iter() {
        if c.is_whitespace() {
            in_word = false;
        } else if !in_word {
            in_word = true;
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn counts_words_said_by_a_character() {
        let script = r#"
INT. ROOM - DAY

ALICE
(tired)
Hey, open up!
(angry)
I mean it!

BOB
Who's there?
"#;

        let stats = Statistics::new(&parse(script));

        let alice = stats
            .total_character_stats(&RichString::from("ALICE"))
            .unwrap();

        assert_eq!(alice.lines_count, 1);
        assert_eq!(alice.words_count, 6);

        let bob = stats
            .total_character_stats(&RichString::from("BOB"))
            .unwrap();
        assert_eq!(bob.lines_count, 1);
        assert_eq!(bob.words_count, 2);
    }

    #[test]
    fn counts_words_across_scenes() {
        let script = r#"
INT. ROOM - DAY

ALICE
Hello world.

INT. OTHER ROOM - DAY

ALICE
Hi there.

BOB
Goodbye.
"#;

        let stats = Statistics::new(&parse(script));

        let alice = stats
            .total_character_stats(&RichString::from("ALICE"))
            .unwrap();
        assert_eq!(alice.lines_count, 2);
        assert_eq!(alice.words_count, 4);
    }

    #[test]
    fn counts_words_split_across_styled_elements() {
        let text: RichString = "foo**bar**".into();
        assert_eq!(count_words(&text), 1);

        let text: RichString = "This is *styled* text.".into();
        assert_eq!(count_words(&text), 4);
    }
}
