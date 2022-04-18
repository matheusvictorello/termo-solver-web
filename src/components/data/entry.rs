use termo_solver::Entry   as TSEntry;
use termo_solver::Pattern as TSPattern;
use termo_solver::Status  as TSStatus;
use termo_solver::Word    as TSWord;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Entry {
    Unset,
    Editable(TSWord, [Option<TSStatus>; 5]),
    Fixed(TSEntry),
}

impl Entry {
    pub fn lock(&mut self) {
        match self {
            Self::Editable(word, pattern) => {
                if let [Some(v0), Some(v1), Some(v2), Some(v3), Some(v4)] = pattern {
                    *self = Entry::Fixed(
                        (
                            *word,
                            TSPattern([*v0, *v1, *v2, *v3, *v4])
                        )
                    );
                }
            }
            _ => {}
        }
    }
}