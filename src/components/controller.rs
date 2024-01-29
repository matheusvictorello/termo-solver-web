use termo_solver::BEST_STARTER;
use termo_solver::Best    as TSBest;
use termo_solver::Solver  as TSSolver;
use termo_solver::Status  as TSStatus;

use yew::prelude::*;
use yew::context::ContextHandle;
use wasm_bindgen_futures;

use crate::components::data::block::Block;
use crate::components::data::entry::Entry;
use crate::components::generic::center::Center;
use crate::components::generic::switch::Switch;
use crate::components::block_card::BlockCard;
use crate::components::column_pallete::ColumnPallete;
use crate::components::status_pallete::StatusPallete;
use crate::ctx::color_ctx::Color;
use crate::ctx::color_ctx::ColorContext;

pub const MAX_COLUMNS: usize = 4;
pub const MAX_LINES:   usize = 9;

pub enum Message {
    ColumnsSelected(usize),
    StatusSelected(Option<TSStatus>),
    SquareSelected(usize, usize, usize),
    SolverFinished(TSBest),
    ColorCtxUpdated(ColorContext),
}

#[derive(Properties, PartialEq)]
pub struct Properties {
    pub columns: usize,
}

pub struct Controller {
    columns:                 usize,
    status:                  Option<TSStatus>,
    blocks:                  [Block; MAX_COLUMNS],
    color:                   ColorContext,
    _color_context_listener: ContextHandle<ColorContext>,
}

impl Controller {
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

fn get_cols_and_lines(cols: usize) -> (usize, usize) {
    match cols {
        0 => (1, 6),
        1 => (1, 6),
        2 => (2, 7),
        3 => (2, 7),
        _ => (4, 9),
    }
}

impl Component for Controller {
    type Message    = Message;
    type Properties = Properties;
    
    fn create(ctx: &Context<Self>) -> Self {
        let (columns, lines) = get_cols_and_lines(ctx.props().columns);

        let status = None;

        let blocks = {
            let mut b = [Block::default(); MAX_COLUMNS];
            
            for i in 0..MAX_COLUMNS {
                b[i].lines = lines;
                b[i].entries[0] = Entry::Editable(BEST_STARTER.clone(), [None; 5]);
            }
            
            b
        };

        let (color, _color_context_listener) = ctx
            .link()
            .context(ctx.link().callback(Self::Message::ColorCtxUpdated))
            .expect("No Color Context Provided");

        Self { columns, status, blocks, color, _color_context_listener }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        let new_columns = ctx.props().columns;

        if self.columns == new_columns {
            return false;
        }

        let (columns, lines) = get_cols_and_lines(new_columns);

        let blocks = {
            let mut b = [Block::default(); MAX_COLUMNS];
            
            for i in 0..MAX_COLUMNS {
                b[i].lines = lines;
                b[i].entries[0] = Entry::Editable(BEST_STARTER.clone(), [None; 5]);
            }
            
            b
        };
        
        self.columns = columns;
        self.blocks  = blocks;

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

        let swap_color = {
            let color = self.color.clone();

            Callback::from(move |set| {
                if set {
                    color.dispatch(Color::Colorblind);
                } else {
                    color.dispatch(Color::Default);
                }
            })
        };

        html! {
            <Center>
                <div class="controller">
                    <Center>
                        <div class="controller_first_row">
                            <ColumnPallete columns={self.columns} onclick={select_columns} />
                            <Switch set={*self.color == Color::Colorblind} onclick={swap_color} />
                        </div>
                    </Center>
                    <StatusPallete status={self.status} onclick={select_status} />
                    <Center class="controller_blocks">
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
                    </Center>
                </div>
            </Center>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ColumnsSelected(new_columns) => {
                if self.columns == new_columns {
                    return false;
                }

                let (columns, lines) = get_cols_and_lines(new_columns);

                let blocks = {
                    let mut b = [Block::default(); MAX_COLUMNS];
                    
                    for i in 0..MAX_COLUMNS {
                        b[i].lines = lines;
                        b[i].entries[0] = Entry::Editable(BEST_STARTER.clone(), [None; 5]);
                    }
                    
                    b
                };

                self.columns = columns;
                self.blocks  = blocks;
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

            Self::Message::ColorCtxUpdated(color) => {
                self.color = color;
            }
        }

        true
    }
}