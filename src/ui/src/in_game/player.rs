use self::player::Player;
use self::rack::TileRack;
use yew::prelude::*;
use yewdux::{component::WithDispatch, prelude::DispatchProps};

use super::state::GameStore;
use crate::components::{
    carbon_icon::{CarbonIcon, Size},
    utils::update_if_changed,
    Button,
};

pub struct PlayersImpl {
    are_scores_expanded: bool,
    props: DispatchProps<GameStore>,
    link: ComponentLink<Self>,
}
pub type Players = WithDispatch<PlayersImpl>;

pub enum Msg {
    Expand,
    Collapse,
}

impl Component for PlayersImpl {
    type Properties = DispatchProps<GameStore>;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            are_scores_expanded: false,
            props,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let desired_are_scores_expanded = matches!(msg, Msg::Expand);
        update_if_changed(&mut self.are_scores_expanded, desired_are_scores_expanded)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // TODO: narrow to used props
        update_if_changed(&mut self.props, props)
    }

    fn view(&self) -> Html {
        let state = self.props.state();
        let players = state.nile.players();
        let on_click = {
            let are_scores_expanded = self.are_scores_expanded;
            self.link.callback(move |_| {
                if are_scores_expanded {
                    Msg::Collapse
                } else {
                    Msg::Expand
                }
            })
        };
        html! {
            <div class="players">
                <div style={ format!("column-count: {}", players.len()) }>
                    { for { players.iter().enumerate().map(|(i, _player)| {
                        html! {
                            <Player key={ i } id={ i as u8 } are_scores_expanded={ self.are_scores_expanded } />
                        }
                    }) } }
                </div>
                <Button class=classes!("expand-collapse", "nile-blue-bg")
                    is_enabled={ !players[0].scores().is_empty() }
                    title={ if self.are_scores_expanded { "Collapse scores" } else { "Expand scores" } }
                    on_click={ on_click }
                >
                    { self.view_inner_collapse_expand() }
                </Button>
            </div>
        }
    }
}

impl PlayersImpl {
    fn view_inner_collapse_expand(&self) -> Html {
        html! {
            { if self.are_scores_expanded {
                html! {
                    <>
                        <CarbonIcon name="row_collapse" size={ Size::S24 } />
                        { "Collapse" }
                    </>
                }
            } else {
                html! {
                    <>
                        <CarbonIcon name="row_expand" size={ Size::S24 } />
                        { "Expand" }
                    </>
                }
            } }
        }
    }
}

mod player {
    use nile::TurnScore;
    use yewdux::prelude::{DispatchPropsMut, Dispatcher};

    use super::super::state::{Action, SelectRackTile};
    use crate::components::utils::{if_render, if_render_html};

    use super::*;

    pub struct PlayerImpl {
        props: Props,
    }

    #[derive(Properties, Clone)]
    pub struct Props {
        #[prop_or_default]
        pub dispatch: DispatchProps<GameStore>,
        pub id: u8,
        pub are_scores_expanded: bool,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            let nile = &self.dispatch.state().nile;
            let other_nile = &other.dispatch.state().nile;
            self.id == other.id
                && self.are_scores_expanded == other.are_scores_expanded
                && nile.current_turn() == other_nile.current_turn()
                && nile.players()[self.id as usize] == other_nile.players()[self.id as usize]
                && nile.selected_rack_tile() == other_nile.selected_rack_tile()
        }
    }

    impl DispatchPropsMut for Props {
        type Store = GameStore;

        fn dispatch(&mut self) -> &mut DispatchProps<Self::Store> {
            &mut self.dispatch
        }
    }

    pub type Player = WithDispatch<PlayerImpl>;

    impl Component for PlayerImpl {
        type Properties = Props;
        type Message = ();

        fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            Self { props }
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            update_if_changed(&mut self.props, props)
        }

        fn view(&self) -> Html {
            let state = self.props.dispatch.state();
            let is_current_turn = state.nile.current_turn() == self.props.id as usize;
            let player = &state.nile.players()[self.props.id as usize];
            let current_turn_score_fwd = Self::sum_turn_scores(player.scores());
            let current_turn_score = player.current_turn_score();
            let mut score_fwd = 0;
            let selected_tile_idx = state.nile.selected_rack_tile();
            let on_select = self
                .props
                .dispatch
                .callback(move |rack_idx| Action::SelectRackTile(SelectRackTile { rack_idx }));
            html! {
                // grid columns start at 1
                <section style={ format!("grid-column: {}", self.props.id + 1) }>
                    <h2 class=classes!(if is_current_turn { "current" } else { "other" })>
                        // nbsp keeps grid if empty player name
                        { format!("{}\u{00a0}", player.name()) }
                    </h2>
                    <TileRack tiles={ player.tiles().to_owned() }
                        show_tiles={ is_current_turn }
                        selected_tile_idx={ selected_tile_idx }
                        on_select={ on_select }
                    />
                    <table class=classes!("scores", is_current_turn.then(|| "current"))>
                        <thead>
                            <tr>
                                <th>{ "Score Fwd" }</th>
                                <th>{ "+" }</th>
                                <th>{ "-" }</th>
                                <th>{ "Net" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { if_render_html(self.props.are_scores_expanded, player.scores().iter().enumerate().map(|(i, score)| {
                                let row_html = html! {
                                    <tr key={ i }>
                                        <td>{ score_fwd }</td>
                                        <td>{ score.add }</td>
                                        <td>{ score.sub }</td>
                                        <td>{ score.add - score.sub }</td>
                                    </tr>
                                };
                                score_fwd = score_fwd + score.add - score.sub;
                                row_html
                            }).collect::<Html>()) }
                            <tr key={ player.scores().len() }>
                                <td>
                                    { current_turn_score_fwd }
                                </td>
                                // Only display current turn scores during turn
                                <td>{ if_render(is_current_turn, current_turn_score.add) }</td>
                                <td>{ if_render(is_current_turn, current_turn_score.sub) }</td>
                                <td>{ if_render(is_current_turn, current_turn_score.add - current_turn_score.sub) }</td>
                            </tr>
                        </tbody>
                    </table>
                </section>
            }
        }
    }

    impl PlayerImpl {
        fn sum_turn_scores(turn_scores: &[TurnScore]) -> i16 {
            turn_scores.iter().fold(0, |acc, ts| acc + ts.add - ts.sub)
        }
    }
}

mod rack {
    use nile::TileArray;

    use crate::components::{utils::if_render, HiddenTile, RackTile};

    use super::*;

    pub struct TileRack {
        props: Props,
    }

    #[derive(Clone, Properties)]
    pub struct Props {
        pub tiles: TileArray,
        pub show_tiles: bool,
        #[prop_or_default]
        pub selected_tile_idx: Option<u8>,
        pub on_select: Callback<u8>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            self.show_tiles == other.show_tiles
                && self.selected_tile_idx == other.selected_tile_idx
                && self.tiles == other.tiles
        }
    }

    impl Component for TileRack {
        type Properties = Props;
        type Message = ();

        fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
            Self { props }
        }

        fn update(&mut self, _msg: Self::Message) -> ShouldRender {
            false
        }

        fn change(&mut self, props: Self::Properties) -> ShouldRender {
            update_if_changed(&mut self.props, props)
        }

        fn view(&self) -> Html {
            let on_drag = Callback::from(move |e: DragEvent| {
                e.prevent_default();
            });
            html! {
                <table>
                    <tbody>
                        <tr>
                            { for self.props.tiles.iter().enumerate().map(|(i, tile)| {
                                let i = i as u8;
                                let on_drag_start = self.props.on_select.reform(move |e: DragEvent| {
                                    e.prevent_default();
                                    i
                                });
                                let on_touch_start = self.props.on_select.reform(move |e: TouchEvent| {
                                    e.prevent_default();
                                    i
                                });
                                let on_click = self.props.on_select.reform(move |e: MouseEvent| {
                                    e.prevent_default();
                                    i
                                });
                                html! {
                                    <td key={ format!("${:?} - ${}", tile, i) }>
                                        <div draggable={ if self.props.show_tiles { "on" } else { "off" }  }
                                            ondrag={ on_drag.clone() }
                                            ondragstart={ on_drag_start.clone() }
                                            ontouchstart={ on_touch_start.clone() }
                                            onclick={ on_click.clone() }
                                        >
                                            { if self.props.show_tiles {
                                                html! {
                                                    <RackTile tile={ *tile } is_selected={ matches!(self.props.selected_tile_idx, Some(idx) if idx == i) } />
                                                }
                                            } else {
                                                html! {
                                                    <HiddenTile />
                                                }
                                            } }
                                        </div>
                                    </td>
                                }
                            }) }
                        </tr>
                        <tr class="align-right">
                            { for self.props.tiles.iter().enumerate().map(|(i, tile)| { html! {
                                <td key={ format!("{:?} - {}", tile, i) }>
                                    { if_render(self.props.show_tiles, tile.score()) }
                                </td>
                            } }) }
                        </tr>
                    </tbody>
                </table>
            }
        }
    }
}
