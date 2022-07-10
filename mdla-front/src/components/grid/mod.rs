use crate::components::grid::grid_line::GridLineComponent;
use mdla_lib::model::GuessResponse;
use stylist::{css, StyleSource, YieldStyle};
use yew::prelude::*;

mod grid_cell;
mod grid_line;

/// Grid
///
///
#[derive(Debug)]
pub struct GridComponent;

#[derive(Debug, Properties, PartialEq)]
pub struct GridProperties {
    pub past_guesses: Vec<GuessResponse>,
    pub width: usize,
    pub on_guessed_word_change: Callback<String>,
}

pub enum Msg {
    UpdateGuess(String),
}

impl Component for GridComponent {
    type Message = Msg;
    type Properties = GridProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lines = 0..ctx.props().past_guesses.len() + 1;
        let width = ctx.props().width;

        html! {
            <table class={self.style()}>{
                lines.into_iter().map(|i| {
                    let guess = ctx.props().past_guesses.get(i).cloned();

                    let editable = i == 0 && guess.is_none() || i > 0 && guess.is_none() && ctx.props().past_guesses.get(i - 1).cloned().is_some();

                    let on_guessed_word_change: Option<Callback<String>> = if editable {
                        Some(ctx.link().callback(|guessed_word: String| {
                            Msg::UpdateGuess(guessed_word)
                        }))
                    } else {
                        None
                    };

                    html! {<tr> <GridLineComponent editable={editable} guess={guess} width={width} {on_guessed_word_change}  /> </ tr>}
                }).collect::<Html>()
            }</table>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::UpdateGuess(char) => {
                _ctx.props().on_guessed_word_change.emit(char);
            }
        }
        false
    }
}

impl YieldStyle for GridComponent {
    fn style_from(&self) -> StyleSource<'static> {
        css!(
            "
            margin-left: auto;
            margin-right: auto;
            background-color: var(--color-back-grid);
            border-spacing: 0;
            background-color: var(--color-back-grid);

            td {
                width: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                height: calc(var(--cell-size) - 2 * var(--width-padding-cell));
                text-align: center;
                position: relative;
                padding: var(--width-padding-cell);
                color: var(--color-police-grid);
                border: 1px solid var(--color-border-grid);
                z-index: 0;
            }

            td.present {
                background-color: var(--color-present);
                border-radius: 50%;
            }
            
            td.correct {
                background-color: var(--color-correct);
            }
            
            td.not-in-word {
                background-color: var(--color-not-in-word);
            }

            td.editable {
                padding: 0;
            }

            td.editabe > input {
                border: none;
                height: 100%;
                outline: none;
                background-color: var(--color-back-grid);
                font-size: 30px;
                width: calc(100% - 2 * var(--width-padding-cell));
                text-align: center;
                color: var(--color-police-grid);
            }
        "
        )
    }
}
