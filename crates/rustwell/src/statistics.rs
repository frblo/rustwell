use std::collections::HashMap;

use crate::{
    Screenplay,
    rich_string::RichString,
    screenplay::{Dialogue, Element},
};

pub struct Statistics {
    characters: HashMap<RichString, usize>,
    scenes: Vec<HashMap<usize, CharacterStats>>,
}

#[derive(Default)]
pub struct CharacterStats {
    pub lines_count: usize,
    pub words_count: usize,
}

impl Statistics {
    pub fn new(screenplay: &Screenplay) -> Self {
        let mut characters = HashMap::new();
        let mut scenes = vec![HashMap::new()];
        let mut scene_idx = 0;

        for e in &screenplay.elements {
            match &**e {
                Element::Heading { slug: _, number: _ } => {
                    scenes.push(HashMap::new());
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

        Statistics { characters, scenes }
    }

    pub fn scene_count(&self) -> usize {
        self.scenes.len()
    }

    pub fn character_count(&self) -> usize {
        self.characters.len()
    }

    pub fn total_character_stats(&self, name: &RichString) -> Option<CharacterStats> {
        if let Some(character_idx) = self.characters.get(name) {
            let mut character_stats = CharacterStats {
                lines_count: 0,
                words_count: 0,
            };
            for scene in self.scenes.iter() {
                match scene.get(character_idx) {
                    Some(CharacterStats {
                        lines_count,
                        words_count,
                    }) => {
                        character_stats.lines_count = *lines_count;
                        character_stats.words_count = *words_count;
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
            scene.insert(i, CharacterStats::default());

            i
        }
    };

    let stats = scene.get_mut(&character_idx).unwrap();
    stats.lines_count += 1;
}
