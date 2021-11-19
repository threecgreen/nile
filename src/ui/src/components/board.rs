use nile::{Cell, Coordinates, TilePlacement, BOARD_SIZE};
use yew::prelude::*;
use yewdux::prelude::{DispatchPropsMut, Dispatcher, Reducer};
use yewdux::{component::WithDispatch, prelude::DispatchProps};

use crate::components::tile::tile_cell;
use crate::state::{GameStore, SelectedTile};

use super::tile::empty_cell::EmptyCell;
use super::tile::tile_cell::{Selection, TileCell, TileCellType};
use super::utils::update_if_changed;
use crate::state::Action;

struct BoardImpl {
    props: DispatchProps<GameStore>,
}
pub type Board = WithDispatch<BoardImpl>;

impl Component for BoardImpl {
    type Properties = DispatchProps<GameStore>;
    type Message = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let bridge = self.props.bridge();
        let state = self.props.state();
        let board = state.nile.board();
        let current_turn_tiles = &state.current_turn_tiles;
        let selection = state.selected_tile;
        let cells = (0..BOARD_SIZE as i8)
            .map(|i| {
                (0..BOARD_SIZE as i8).map(|j| {
                    let coordinates = Coordinates(i, j);
                    let cell = board.cell(coordinates).unwrap();
                    let is_seleted = match selection {
                        Some(SelectedTile::Board(c)) => c == coordinates,
                        _ => false,
                    };
                    let on_select = {
                        Callback::from(|_| {
                            self.props.send( Action::SelectBoardTile(coordinates));
                        })
                    };

                    html! {
                        <td key={ format!("{}", j) }>
                            { Self::view_cell(cell, TileCellType::from((cell, board.is_end_game_cell(coordinates))), Selection::from((is_seleted, current_turn_tiles.contains(&coordinates))), on_select) }
                        </td>
                    }
                })
            })
            .collect::<Html>();

        html! {
            <div class="outer">
                <span class="start">{ "Start" }</span>
                <span class="arrow">{ "→" }</span>
                <table class="board">
                    // {
                    // }
                </table>
            </div>
        }
    }
}

impl From<(&Cell, bool)> for TileCellType {
    fn from((cell, is_end_game): (&Cell, bool)) -> Self {
        if is_end_game {
            Self::EndGame
        } else {
            match cell.bonus() {
                b if b > 0 => Self::Bonus,
                b if b < 0 => Self::Penalty,
                _ => Self::Normal,
            }
        }
    }
}

impl From<(bool, bool)> for Selection {
    fn from((is_selected, is_from_current_turn): (bool, bool)) -> Self {
        if is_selected {
            Self::Selected
        } else if is_from_current_turn {
            Self::Selectable
        } else {
            Self::Locked
        }
    }
}

impl BoardImpl {
    fn view_cell(
        cell: Cell,
        tile_cell_type: TileCellType,
        selection: Selection,
        on_select: Callback<()>,
        on_drop: Callback<()>,
    ) -> Html {
        match cell.tile() {
            Some(tp) => {
                html! {
                    <TileCell tile_path_type={ tp.tile_path_type().clone() }
                        rotation={ tp.rotation() }
                        tile_cell_type={ tile_cell_type }
                        selection={ selection }
                        on_select={ on_select }
                    />
                }
            }
            None => {
                html! {
                    <EmptyCell bonus={ cell.bonus() }
                        is_end_game={ tile_cell_type == TileCellType::EndGame }
                        on_drop={ on_drop }
                    />
                }
            }
        }
    }
}
