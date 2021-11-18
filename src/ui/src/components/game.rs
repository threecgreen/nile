use std::collections::HashSet;
use std::rc::Rc;

use nile::{CPUTurnUpdate, Coordinates, EndTurnUpdate, TilePath, TilePlacement, TurnScore};
use yew::prelude::*;
use yew::services::KeyboardService;

struct State {
    /// Main game state
    nile: nile::Nile,
    /// Used for determining if placed tile is movable, rotatable, etc.
    current_turn_tiles: HashSet<Coordinates>,
}

pub struct Game {
    state: Rc<State>,
    link: ComponentLink<Self>,
    keyboard_service: KeyboardService,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
}

pub enum Msg {
    SelectRackTile(SelectRackTile),
    SelectBoardTile(Coordinates),
    PlaceTile(PlaceTile),
    RotateTile(TilePlacement),
    RemoveTile(RemoveTile),
    UpdateUniversalPath(UpdateUniversalPath),
    MoveTile(MoveTile),
    Undo,
    Redo,
    EndTurn(EndTurnUpdate),
    CpuTurn(CPUTurnUpdate),
    SetError(String),
    SetEndOfGame(String),
    Dismiss,
}

struct SelectRackTile {
    tile_path: TilePath,
    is_universal: bool,
    rack_idx: u8,
}

struct PlaceTile {
    dragged_tile: SelectRackTile,
    tile_placement: TilePlacement,
    score: TurnScore,
}

struct RemoveTile {
    coordinates: Coordinates,
    score: TurnScore,
}

struct UpdateUniversalPath {
    coordinates: Coordinates,
    tile_placement: TilePlacement,
}

struct MoveTile {
    old_coordinates: Coordinates,
    new_coordinates: Coordinates,
    tile_placement: TilePlacement,
    score: TurnScore,
}

impl Component for Game {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            state: Rc::new(State {
                nile: nile::Nile::new(props.player_names, props.cpu_player_count as usize)
                    // TODO: remove expect
                    .expect("nile"),
                current_turn_tiles: HashSet::new(),
            }),
            link,
            keyboard_service: KeyboardService {},
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <section>
                </section>
                <section>
                </section>
            </>
        }
    }
}
