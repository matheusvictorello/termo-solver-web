use yew::prelude::*;
use wasm_bindgen_futures;
use web_sys::console;

use crate::termo::*;
use crate::LetterCard::*;
use crate::SquareLine::*;
use crate::Square::*;
use crate::ColorPalette::*;
use crate::BlockPalette::*;

pub const MAX_COLUMNS: usize = 4;
pub const MAX_LINES:   usize = 8;

pub enum TermoGameMsg {
    BlockNumberSelected(usize),
    StatusSelected(SquareStatus),
    CardSelected {
        column: usize,
        line:   usize,
        letter: usize,
    },
    LineFilled {
        line: usize,
    },
    SolverCompleted {
        result: Option<(f32, Word)>,
    }
}

#[derive(Properties, PartialEq)]
pub struct TermoGameProps {
    pub columns: usize
}

pub struct TermoGame {
    pub columns:               usize,
    pub lines:                 usize,
    pub selected_status:       SquareStatus,

    pub words:                 [Option<Word>; MAX_LINES],
    pub status:                [[SquarePattern; MAX_LINES]; MAX_COLUMNS],
    pub edit_line:             usize,
    pub solved:                [bool; MAX_COLUMNS],
}

impl TermoGame {
    async fn presolve(
        cb:        Callback<TermoGameMsg>,
        columns:   usize,
        lines:     usize,
        edit_line: usize,
        words:     [Option<Word>; MAX_LINES],
        status:    [[SquarePattern; MAX_LINES]; MAX_COLUMNS],
        solved:    [bool; MAX_COLUMNS],
    ) {
        console::log_1(&format!("Solver Started").into());

        // Entries for unsolved columns
        let mut multiple_entries: Vec<Vec<Entry>> = Vec::new();

        console::log_1(&format!("solved {:?}", solved).into());

        // Test and prepare data of unsolved columns
        for c in 0..columns {
            // Check is this column is solved
            if solved[c] {
                console::log_1(&format!("Skipped column {} in solver", c).into());
                continue;
            } else {
                console::log_1(&format!("Not Skipped column {} in solver", c).into());
            }

            // Get this column's status
            let status = status[c];

            // Zip it with words
            let word_status_pairs = words
                .iter()
                .zip(status.iter())
                .collect::<Vec<_>>();

            // This column's entries
            let mut entries: Vec<Entry> = Vec::new();

            // Build the entries
            for i in 0..edit_line {
                let (word, pattern) = word_status_pairs.get(i).unwrap();

                let word = word.unwrap();

                // Convert SquarePattern to Pattern
                let pattern: Option<Pattern> = (*pattern).into();
                let pattern = pattern.unwrap();

                entries.push((word, pattern));
            }

            // Save this entry
            if entries.len() > 0 {
                multiple_entries.push(entries);
            }
        }

        let best = Solver::solve_multiple(&multiple_entries);

        console::log_1(&format!("Solver Ended").into());
        
        cb.emit(TermoGameMsg::SolverCompleted {
            result: best
        });
    }
}

impl Component for TermoGame {
    type Message    = TermoGameMsg;
    type Properties = TermoGameProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let (columns, lines) = match props.columns {
            0 => (1, 6),
            1 => (1, 6),
            2 => (2, 7),
            3 => (2, 7),
            _ => (4, 8),
        };
      
        let words = { let mut v = [None; MAX_LINES]; v[0] = Some(BEST_STARTER); v };

        let status = [[SquarePattern([SquareStatus::Empty; 5]); MAX_LINES]; MAX_COLUMNS];
        
        let selected_status = SquareStatus::Empty;

        let edit_line = 0;

        let solved = [false; MAX_COLUMNS];

        Self { columns, lines, words, status, selected_status, edit_line, solved }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let props = ctx.props();

        let (columns, lines) = match props.columns {
            0 => (1, 6),
            1 => (1, 6),
            2 => (2, 7),
            3 => (2, 7),
            _ => (4, 8),
        };
      
        let words = { let mut v = [None; MAX_LINES]; v[0] = Some(BEST_STARTER); v };

        let status = [[SquarePattern([SquareStatus::Empty; 5]); MAX_LINES]; MAX_COLUMNS];
        
        let selected_status = SquareStatus::Empty;

        let edit_line = 0;

        let solved = [false; MAX_COLUMNS];

        self.columns         = columns;
        self.lines           = lines;
        self.words           = words;
        self.status          = status;
        self.selected_status = selected_status;
        self.edit_line       = edit_line;
        self.solved          = solved;

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <div class="termo_game">
                <div class="inner">
                    <BlockPalette
                        columns={self.columns}
                        onclick={
                            link.callback(|number| Self::Message::BlockNumberSelected(number))
                        }
                    />
                    <ColorPalette
                        status={self.selected_status}
                        onclick={
                            link.callback(|status| Self::Message::StatusSelected(status))
                        }
                    />
                    <div class="letter_card_wrapper">
                        {
                            for (0..self.columns).map(|i|
                                match self.status.get(i) {
                                    Some(&status) => html! {
                                        <LetterCard
                                            lines={self.lines}
                                            words={self.words}
                                            {status}
                                            onsquareclick={
                                                link.callback(move |(line, letter)| Self::Message::CardSelected{
                                                    column: i,
                                                    line,
                                                    letter,
                                                })
                                            }
                                        />
                                    },
                                    _ => html! {},
                                }
                            )
                        }
                    </div>
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        match msg {
            Self::Message::BlockNumberSelected(columns) => {
                // self.columns = columns;
            }

            Self::Message::StatusSelected(status) => {
                self.selected_status = status;
            }
            
            Self::Message::CardSelected { column, line, letter } => {
                // Check is this column is solved
                if self.solved[column] {
                    return false;
                }

                // Check if this line is editable
                if line != self.edit_line {
                    return false;
                }

                // Update square status
                self.status[column][line].0[letter] = self.selected_status;

                // Counts non Empty for LineFilled event
                let mut counter = 0;
                let mut unsolved_columns = 0;

                console::log_1(&format!("solved: {:?}", self.solved).into());

                for c in 0..self.columns {
                    // Check is the column is unsolved
                    if !self.solved[c] {
                        unsolved_columns += 1;
                        console::log_1(&format!("increment unsolved_columns {:?}", unsolved_columns).into());
                    }

                    // Counts non Empty squared for LineFilled event
                    for l in 0..5 {
                        if self.status[c][line].0[l] != SquareStatus::Empty {
                            counter += 1;
                        }
                    }
                }

                console::log_1(&format!("got  counter: {:?}", counter).into());
                console::log_1(&format!("need counter: {:?}", unsolved_columns * 5).into());

                // Check if the line is full for LineFilled event
                if counter == (unsolved_columns * 5) {
                    link.callback(|e| e).emit(Self::Message::LineFilled{
                        line,
                    });
                } else {
                    console::log_1(&format!("NO LINE FILLED").into());
                }
            }

            Self::Message::LineFilled { line } => {
                // Line was filled, increment the current line
                self.edit_line = line + 1;

                console::log_1(&format!("edit_line: {:?}", self.edit_line).into());

                // Check for solved lines
                for c in 0..self.columns {
                    if self.solved[c] {
                        continue;
                    }

                    let mut completed = true;

                    // Look for a non Right status
                    for l in 0..5 {
                        if self.status[c][line].0[l] != SquareStatus::Right {
                            completed = false;
                            console::log_1(&format!("column {:?} is not complete cuz letter {:?}", c, l).into());
                            break;
                        }
                    }

                    console::log_1(&format!("column {:?} is completed? {:?}", c, completed).into());

                    // Save the result
                    self.solved[c] = completed;
                }

                // Spawn solver
                wasm_bindgen_futures::spawn_local(
                    TermoGame::presolve(
                        link.callback(|e| e),
                        self.columns,
                        self.lines,
                        self.edit_line,
                        self.words,
                        self.status,
                        self.solved,
                    )
                );
            }

            Self::Message::SolverCompleted { result } => {
                console::log_1(&format!("Solved: {:?}", result).into());

                // TODO
                if (self.edit_line) < self.lines {
                    self.words[self.edit_line] = result.map(|(_, word)| word);
                }
            }
        }

        true
    }
}