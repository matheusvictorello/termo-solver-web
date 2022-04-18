use termo_solver::Pattern as TSPattern;
use termo_solver::Status  as TSStatus;
use termo_solver::Word    as TSWord;

use crate::components::data::entry::Entry;
use crate::components::controller::MAX_LINES;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub lines:   usize,
    pub solved:  bool,    
    pub entries: [Entry; MAX_LINES],
}

impl Default for Block {
    fn default() -> Self {
        Self {
            lines:   0,
            solved:  false,
            entries: [Entry::Unset; MAX_LINES],
        }
    }
}

impl Block {
    pub fn update_status(&mut self, entry: usize, letter: usize, new_status: Option<TSStatus>) {
        // Update letter status
        let mut entry = &mut self.entries[entry];

        match &mut entry {
            Entry::Editable(_, pattern) => {
                pattern[letter] = new_status;
            }
            _ => {}
        }
    }

    pub fn is_filled(&self, entry: usize) -> bool {
        if self.solved {
            true
        } else {
            match self.entries[entry] {
                Entry::Unset => {
                    false
                }
                Entry::Editable(_, pattern) => {
                    0 == pattern
                        .iter()
                        .filter(|&&p| p == None)
                        .count()
                }
                Entry::Fixed(_) => {
                    true
                }
            }
        }
    }

    pub fn lock(&mut self, entry: usize) {
        self.entries[entry].lock();

        // Check for completion
        if let Entry::Fixed((_, TSPattern([
            TSStatus::Right,
            TSStatus::Right,
            TSStatus::Right,
            TSStatus::Right,
            TSStatus::Right,
        ]))) = self.entries[entry] {
            self.solved = true;
        }
    }

    pub fn push(&mut self, word: TSWord) {
        if !self.solved {
            for entry in self.entries.iter_mut() {
                match entry {
                    Entry::Unset => {
                        *entry = Entry::Editable(word, [None; 5]);
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}