use termo_solver::BEST_STARTER;
use termo_solver::Best    as TSBest;
use termo_solver::Entry   as TSEntry;
use termo_solver::Pattern as TSPattern;
use termo_solver::Solver  as TSSolver;
use termo_solver::Status  as TSStatus;
use termo_solver::Word    as TSWord;

use yew::prelude::*;
use wasm_bindgen_futures;
use web_sys::console;

use crate::components::center::Center;
use crate::components::column_pallete::ColumnPallete;
use crate::components::status_pallete::StatusPallete;
use crate::components::block_card::BlockCard;

const MAX_COLUMNS: usize = 4;
const MAX_LINES:   usize = 8;

// Data

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Entry {
    Unset,
    Editable(TSWord, [Option<TSStatus>; 5]),
    Fixed(TSEntry),
}

impl Entry {
    fn lock(&mut self) {
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
    fn update_status(&mut self, entry: usize, letter: usize, new_status: Option<TSStatus>) {
        // Update letter status
        let mut entry = &mut self.entries[entry];

        match &mut entry {
            Entry::Editable(word, pattern) => {
                pattern[letter] = new_status;
            }
            _ => {}
        }
    }

    fn is_filled(&self, entry: usize) -> bool {
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

    fn lock(&mut self, entry: usize) {
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

    fn push(&mut self, word: TSWord) {
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

// Component

pub enum Message {
    ColumnsSelected(usize),
    StatusSelected(Option<TSStatus>),
    SquareSelected(usize, usize, usize),
    SpawnSolver,
    SolverFinished(TSBest),
}

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub columns: usize,
}

pub struct Controller {
    columns: usize,
    lines:   usize,
    status:  Option<TSStatus>,
    blocks:  [Block; MAX_COLUMNS],
    next:    Option<Callback<()>>,
}

impl Controller {
    fn reset(&mut self, columns: usize) -> bool {
        let (columns, lines) = match columns {
            0 => (1, 6),
            1 => (1, 6),
            2 => (2, 7),
            3 => (2, 7),
            _ => (4, 8),
        };

        let blocks = {
            let mut b = [Block::default(); MAX_COLUMNS];
            
            for i in 0..MAX_COLUMNS {
                b[i].lines = lines;
                b[i].entries[0] = Entry::Editable(BEST_STARTER.clone(), [None; 5]);
            }
            
            b
        };

        let next = None;

        self.columns = columns;
        self.lines   = lines;
        self.blocks  = blocks;
        self.next    = next;

        true
    }

    async fn solve(cb: Callback<TSBest>, columns: usize, blocks: [Block; MAX_COLUMNS]) {
        let mut multiple_entries: Vec<Vec<TSEntry>> = Vec::new();

        for block in blocks.iter().take(columns) {
            if block.solved {
                continue;
            }

            let mut entries: Vec<TSEntry> = Vec::new();

            for entry in block.entries {
                if let Entry::Fixed(pair) = entry {
                    entries.push(pair);
                }
            }

            multiple_entries.push(entries);
        }

        if multiple_entries.len() > 0 {
            let best = TSSolver::solve_multiple(&multiple_entries);

            cb.emit(best);
        }
    }
}

impl Component for Controller {
    type Message = Message;
    type Properties = Properties;
    
    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let (columns, lines) = match props.columns {
            0 => (1, 6),
            1 => (1, 6),
            2 => (2, 7),
            3 => (2, 7),
            _ => (4, 8),
        };

        let status = None;

        let blocks = {
            let mut b = [Block::default(); MAX_COLUMNS];
            
            for i in 0..MAX_COLUMNS {
                b[i].lines = lines;
                b[i].entries[0] = Entry::Editable(BEST_STARTER.clone(), [None; 5]);
            }
            
            b
        };

        let next = None;

        Self {
            columns,
            lines,
            status,
            blocks,
            next,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        self.reset(props.columns)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(cb) = &self.next {
            cb.emit(());
        }

        let link = ctx.link();

        let selectColumns = link.callback(|v| Self::Message::ColumnsSelected(v));
        let selectStatus  = link.callback(|v| Self::Message::StatusSelected(v));

        html! {
            <Center>
                <div class="controller">
                    <ColumnPallete columns={self.columns} onclick={selectColumns} />
                    <StatusPallete status={self.status} onclick={selectStatus} />
                    <div class="controller_blocks">
                        {
                            for self.blocks
                                .into_iter()
                                .take(self.columns)
                                .enumerate()
                                .map(|(i, block)|
                            {
                                let selectSquare = link.callback(move |(j, k)| Self::Message::SquareSelected(i, j, k));

                                html! {
                                    <BlockCard {block} onclick={selectSquare} />
                                }
                            })
                        }
                    </div>
                </div>
            </Center>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.next = None;

        match msg {
            Self::Message::ColumnsSelected(columns) => {
                self.reset(columns);
            }
            Self::Message::StatusSelected(status) => {
                self.status = status;
            }
            Self::Message::SquareSelected(block, entry, letter) => {
                // Fill Square
                self.blocks[block].update_status(entry, letter, self.status);

                // Check for line completion
                let filled = self.blocks
                    .iter()
                    .take(self.columns)
                    .map(|block| block.is_filled(entry))
                    .filter(|&v| v)
                    .count();

                // Line was filled
                if self.columns == filled {
                    // Lock completed lines
                    for block in self.blocks.iter_mut().take(self.columns) {
                        block.lock(entry);
                    }

                    self.next = Some(ctx.link().callback(|_| Self::Message::SpawnSolver));
                }
            }
            Self::Message::SpawnSolver => {
                // Spawn solver
                let cb = ctx.link().callback(|best| Self::Message::SolverFinished(best));

                wasm_bindgen_futures::spawn_local(
                    Controller::solve(cb, self.columns, self.blocks)
                );
            }
            Self::Message::SolverFinished(best) => {
                // Add new word to incomplete blocks
                if let Some((_, word)) = best {
                    self.blocks
                        .iter_mut()
                        .take(self.columns)
                        .map(|block| block.push(word))
                        .collect::<Vec<()>>();
                }
            }
        }

        true
    }
}