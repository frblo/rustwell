use std::collections::{HashMap, HashSet};

use crate::{
    Screenplay,
    rich_string::RichString,
    screenplay::{Dialogue, Element},
};

pub struct Statistics {
    characters_in_scene: Vec<Vec<usize>>,
    characters: Vec<CharacterStats>,
}

struct CharacterStats {
    name: String,
    lines_count: usize,
    words_count: usize,
    scenes: Vec<usize>,
}

impl Statistics {
    pub fn new(screenplay: &Screenplay) -> Self {
        let mut character_idx: HashMap<RichString, usize> = HashMap::new();
        let mut character_counter = 0;

        let mut lines_count = Vec::new();
        // let mut words = Vec::new();
        // let mut scenes = Vec::new();

        // let mut characters_in_scenes = vec![HashSet::new()];

        let mut scene = 0;
        for e in &screenplay.elements {
            match e {
                Element::Heading { slug: _, number: _ } => scene += 1,
                Element::Dialogue(dialogue) => handle_dialogue(
                    dialogue,
                    &mut character_idx,
                    &mut character_counter,
                    &mut lines_count,
                ),
                Element::DualDialogue(dialogue1, dialogue2) => {
                    handle_dialogue(
                        dialogue1,
                        &mut character_idx,
                        &mut character_counter,
                        &mut lines_count,
                    );
                    handle_dialogue(
                        dialogue2,
                        &mut character_idx,
                        &mut character_counter,
                        &mut lines_count,
                    );
                }
                _ => continue,
            }
        }

        Statistics {
            characters_in_scene: Vec::new(),
            characters: Vec::new(),
        }
    }

    pub fn scene_count(&self) -> usize {
        self.characters_in_scene.len()
    }

    pub fn character_count(&self) -> usize {
        self.characters.len()
    }
}

fn handle_dialogue(
    dialogue: &Dialogue,
    character_idx: &mut HashMap<RichString, usize>,
    character_counter: &mut usize,
    lines_count: &mut Vec<usize>,
) {
    let name = dialogue.character.clone();
    let idx = character_idx.get(&name).cloned().unwrap_or_else(|| {
        let i = character_counter.clone();
        *character_counter += 1;

        character_idx.insert(name, i);
        lines_count.push(0);

        i
    });

    lines_count[idx] += 1;
}
