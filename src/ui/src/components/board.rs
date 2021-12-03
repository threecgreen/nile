use nile::{Cell, Coordinates, BOARD_DIM};
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux::{component::WithDispatch, prelude::DispatchProps};

use crate::state::GameStore;

use super::tile::empty_cell::EmptyCell;
use super::tile::tile_cell::{Selection, TileCell, TileCellType};
use super::utils::update_if_changed;
use crate::state::Action;

pub struct BoardImpl {
    props: DispatchProps<GameStore>,
}
pub type Board = WithDispatch<BoardImpl>;

impl Component for BoardImpl {
    type Properties = DispatchProps<GameStore>;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO: narrow to just used props
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let state = self.props.state();
        let board = state.nile.board();
        let current_turn_placements = state.nile.current_turn_placements();
        let selection = state.nile.selected_board_tile();
        let cells = (0..BOARD_DIM as i8)
            .map(|i| {
                html! {
                    <tr key={ i }>
                        { for
                            (0..BOARD_DIM as i8).map(|j| {
                                let coordinates = Coordinates(i, j);
                                let cell = board.cell(coordinates).unwrap();
                                let is_seleted = match selection {
                                    Some(c) => c == coordinates,
                                    _ => false,
                                };
                                let on_select = self.props.callback(move |_| Action::SelectBoardTile(coordinates));
                                let on_drop = self.props.callback(move |_| Action::PlaceTile(coordinates));

                                html! {
                                    <td key={ j }>
                                        { Self::view_cell(cell, TileCellType::from((cell, board.is_end_game_cell(coordinates))), Selection::from((is_seleted, current_turn_placements.contains(&coordinates))), on_select, on_drop) }
                                    </td>
                                }
                            }) }
                    </tr>
                }
            });

        html! {
            <div class="outer">
                <span class="start">{ "Start" }</span>
                <span class="arrow">{ "→" }</span>
                <table class="board">
                    <tbody>
                        { for cells }
                    </tbody>
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
        cell: &Cell,
        tile_cell_type: TileCellType,
        selection: Selection,
        on_select: Callback<()>,
        on_drop: Callback<()>,
    ) -> Html {
        match cell.tile() {
            Some(tp) => {
                html! {
                    <TileCell tile_path_type={ *tp.tile_path_type() }
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
