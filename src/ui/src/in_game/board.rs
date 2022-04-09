use std::rc::Rc;

use nile::{Cell, Coordinates, BOARD_DIM};
use yew::prelude::*;

use super::state::{Action, Dispatch, State, UpdateStateMsg};
use crate::components::EmptyCell;
use crate::components::{tile_cell::Selection, tile_cell::TileCellType, TileCell};

pub struct Board {
    dispatch: Dispatch,
    state: Rc<State>,
}

impl Component for Board {
    type Properties = ();
    type Message = UpdateStateMsg;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(UpdateStateMsg);
        let dispatch = Dispatch::subscribe(callback);
        let state = dispatch.get();
        Self { dispatch, state }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let old_state = &self.state;
        let UpdateStateMsg(new_state) = msg;
        if !Rc::<nile::Board>::ptr_eq(new_state.nile.rc_board(), old_state.nile.rc_board())
            || old_state.nile.current_turn_placements() != new_state.nile.current_turn_placements()
            || old_state.nile.selected_board_tile() != new_state.nile.selected_board_tile()
            || old_state.nile.error_cells() != new_state.nile.error_cells()
        {
            self.state = new_state;
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let state = &self.state;
        let board = state.nile.board();
        let current_turn_placements = state.nile.current_turn_placements();
        let selection = state.nile.selected_board_tile();

        let cells = (0..BOARD_DIM as i8)
            .map(|i| {
                html! {
                    <tr key={ i }>
                        { for
                            // Extra column for end of game tiles
                            (0..=BOARD_DIM as i8).map(|j| {
                                let coordinates = Coordinates(i, j);
                                let cell = board.cell(coordinates).unwrap();
                                let is_seleted = match selection {
                                    Some(c) => c == coordinates,
                                    _ => false,
                                };
                                let is_error = state.nile.error_cells().map_or(false, |error_cells| error_cells.contains(&coordinates));
                                let on_select =  self.dispatch.apply_callback(move |_| Action::SelectBoardTile(coordinates));
                                let on_drop = self.dispatch.apply_callback(move |_| Action::PlaceTile(coordinates));

                                html! {
                                    <td key={ j }>
                                        { Self::view_cell(cell, TileCellType::from((cell, board.is_end_game_cell(coordinates))), Selection::from((is_seleted, current_turn_placements.contains(&coordinates))), is_error, on_select, on_drop) }
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

impl Board {
    fn view_cell(
        cell: &Cell,
        tile_cell_type: TileCellType,
        selection: Selection,
        is_error: bool,
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
                        is_error={ is_error }
                        on_select={ on_select }
                    />
                }
            }
            None => {
                html! {
                    <EmptyCell bonus={ cell.bonus() }
                        is_end_game={ tile_cell_type == TileCellType::EndGame }
                        is_error={ is_error }
                        on_drop={ on_drop }
                    />
                }
            }
        }
    }
}
