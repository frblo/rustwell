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
        let mut character_idx = HashMap::new();
        let mut character_counter = 0;

        let mut lines_count = Vec::new();
        // let mut words = Vec::new();
        let mut scenes = Vec::new();

        let mut characters_in_scenes = vec![HashSet::new()];

        let mut scene = 0;
        for e in screenplay.elements {
            match e {
                Element::Heading { slug: _, number: _ } => scene += 1,
                Element::Dialogue(dialogue) => {}
                Element::DualDialogue(dialogue, dialogue1) => todo!(),
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
    character_idx: &mut HashMap<&RichString, usize>,
    character_counter: &mut usize,
    lines_count: &mut Vec<usize>,
) {
    let name = &dialogue.character;
    let idx = character_idx.get(&name).unwrap_or_else(|| {
        let i = *character_counter;
        *character_counter += 1;

        character_idx.insert(&name, i);
        lines_count.push(0);

        &i
    });

    lines_count[*idx] += 1;
}
