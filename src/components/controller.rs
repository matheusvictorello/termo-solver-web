use termo_solver::BEST_STARTER;
use termo_solver::Best    as TSBest;
use termo_solver::Solver  as TSSolver;
use termo_solver::Status  as TSStatus;

use yew::prelude::*;
use wasm_bindgen_futures;

use crate::components::data::block::Block;
use crate::components::data::entry::Entry;
use crate::components::generic::center::Center;
use crate::components::block_card::BlockCard;
use crate::components::column_pallete::ColumnPallete;
use crate::components::status_pallete::StatusPallete;

pub const MAX_COLUMNS: usize = 4;
pub const MAX_LINES:   usize = 8;

pub enum Message {
    ColumnsSelected(usize),
    StatusSelected(Option<TSStatus>),
    SquareSelected(usize, usize, usize),
    SolverFinished(TSBest),
}

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub columns: usize,
}

pub struct Controller {
    columns: usize,
    status:  Option<TSStatus>,
    blocks:  [Block; MAX_COLUMNS],
}

impl Controller {
    fn new(columns: usize) -> Self {
        let (columns, lines) = match columns {
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

        Self { columns, status, blocks }
    }

    async fn solve(cb: Callback<TSBest>, columns: usize, blocks: [Block; MAX_COLUMNS]) {
        let multiple_entries = blocks
            .iter()
            .take(columns)
            .filter(|block| !block.solved)
            .map(|block| {
                block.entries
                    .iter()
                    .map(|entry| {
                        match entry {
                            Entry::Fixed(v) => Some(v),
                            _               => None,
                        }
                    })
                    .filter(|entry| *entry != None)
                    .map(|entry| *entry.unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if multiple_entries.len() > 0 {
            cb.emit(TSSolver::solve_multiple(&multiple_entries));
        }
    }
}

impl Component for Controller {
    type Message    = Message;
    type Properties = Properties;
    
    fn create(ctx: &Context<Self>) -> Self {
        Self::new(ctx.props().columns)
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        *self = Self::new(ctx.props().columns);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let select_columns = link.callback(|v| {
            Self::Message::ColumnsSelected(v)
        });
        let select_status  = link.callback(|v| {
            Self::Message::StatusSelected(v)
        });

        html! {
            <Center>
                <div class="controller">
                    <ColumnPallete columns={self.columns} onclick={select_columns} />
                    <StatusPallete status={self.status} onclick={select_status} />
                    <div class="controller_blocks">
                        {
                            for self.blocks
                                .into_iter()
                                .take(self.columns)
                                .enumerate()
                                .map(|(i, block)| {
                                    let select_square = link.callback(move |(j, k)| {
                                        Self::Message::SquareSelected(i, j, k)
                                    });

                                    html! {
                                        <BlockCard key={i} {block} onclick={select_square} />
                                    }
                                })
                        }
                    </div>
                </div>
            </Center>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ColumnsSelected(columns) => {
                *self = Self::new(columns);
            }
            
            Self::Message::StatusSelected(status) => {
                self.status = status;
            }
            
            Self::Message::SquareSelected(block, entry, letter) => {
                self.blocks[block].update_status(entry, letter, self.status);

                let filled = self.blocks
                    .iter()
                    .take(self.columns)
                    .map(|block| block.is_filled(entry))
                    .filter(|&v| v)
                    .count();

                if self.columns == filled {
                    for block in self.blocks
                        .iter_mut()
                        .take(self.columns)
                    {
                        block.lock(entry);
                    }

                    wasm_bindgen_futures::spawn_local(
                        Controller::solve(
                            ctx.link().callback(|best| Self::Message::SolverFinished(best)),
                            self.columns,
                            self.blocks
                        )
                    );
                }
            }
            
            Self::Message::SolverFinished(best) => {
                if let Some((_, word)) = best {
                    let _ = self.blocks
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