use crate::Screenplay;

pub struct Statistics {
    characters_in_scene: Vec<Vec<usize>>,
    characters: Vec<CharacterStats>,
}

impl Statistics {
    pub fn scene_count(&self) -> usize {
        self.characters_in_scene.len()
    }

    pub fn character_count(&self) -> usize {
        self.characters.len()
    }
}

struct CharacterStats {
    name: String,
    lines_count: usize,
    words_count: usize,
    scenes: Vec<usize>,
}

pub fn gen_statistics(screenplay: &Screenplay) -> Statistics {}
