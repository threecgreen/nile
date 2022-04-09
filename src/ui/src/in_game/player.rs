use std::rc::Rc;

use self::player::Player;
use self::rack::TileRack;
use yew::prelude::*;

use crate::components::{
    carbon_icon::{CarbonIcon, Size},
    utils::update_if_changed,
    Button,
};

use super::state::{Dispatch, State, UpdateStateMsg};

pub struct Players {
    _dispatch: Dispatch,
    state: Rc<State>,
    are_scores_expanded: bool,
}

pub enum Msg {
    Expand,
    Collapse,
    UpdateState(Rc<State>),
}

impl Component for Players {
    type Properties = ();
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(Msg::UpdateState);
        let dispatch = Dispatch::subscribe(callback);
        let state = dispatch.get();
        Self {
            _dispatch: dispatch,
            state,
            are_scores_expanded: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let desired_are_scores_expanded = matches!(msg, Msg::Expand);
        update_if_changed(&mut self.are_scores_expanded, desired_are_scores_expanded)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.state;
        let players = state.nile.players();
        let on_click = {
            let are_scores_expanded = self.are_scores_expanded;
            ctx.link().callback(move |_| {
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
                <Button class={ classes!("expand-collapse", "nile-blue-bg") }
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

impl Players {
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

    use super::super::state::{Action, SelectRackTile};
    use crate::components::utils::{if_render, if_render_html};

    use super::*;

    pub struct Player {
        dispatch: Dispatch,
        state: Rc<State>,
    }

    #[derive(Properties, Clone, PartialEq)]
    pub struct Props {
        pub id: u8,
        pub are_scores_expanded: bool,
    }

    // impl PartialEq for Props {
    //     fn eq(&self, other: &Self) -> bool {
    //         let nile = &self.dispatch.state().nile;
    //         let other_nile = &other.dispatch.state().nile;
    //         self.id == other.id
    //             && self.are_scores_expanded == other.are_scores_expanded
    //             && nile.current_turn() == other_nile.current_turn()
    //             && nile.players()[self.id as usize] == other_nile.players()[self.id as usize]
    //             && nile.selected_rack_tile() == other_nile.selected_rack_tile()
    //     }
    // }

    impl Component for Player {
        type Properties = Props;
        type Message = UpdateStateMsg;

        fn create(ctx: &Context<Self>) -> Self {
            let callback = ctx.link().callback(UpdateStateMsg);
            let dispatch = Dispatch::subscribe(callback);
            let state = dispatch.get();
            Self { dispatch, state }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            // TODO: update only if specific player changes
            UpdateStateMsg(self.state) = msg;
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let state = &self.state;
            let is_current_turn = state.nile.current_turn() == ctx.props().id as usize;
            let player = &state.nile.players()[ctx.props().id as usize];
            let current_turn_score_fwd = Self::sum_turn_scores(player.scores());
            let current_turn_score = player.current_turn_score();
            let mut score_fwd = 0;
            let selected_tile_idx = state.nile.selected_rack_tile();
            let on_select = self.dispatch.apply_callback(move |rack_idx| {
                Action::SelectRackTile(SelectRackTile { rack_idx })
            });
            html! {
                // grid columns start at 1
                <section style={ format!("grid-column: {}", ctx.props().id + 1) }>
                    <h2 class={ classes!(if is_current_turn { "current" } else { "other" }) }>
                        // nbsp keeps grid if empty player name
                        { format!("{}\u{00a0}", player.name()) }
                    </h2>
                    <TileRack tiles={ player.tiles().to_owned() }
                        show_tiles={ is_current_turn }
                        selected_tile_idx={ selected_tile_idx }
                        on_select={ on_select }
                    />
                    <table class={ classes!("scores", is_current_turn.then(|| "current")) }>
                        <thead>
                            <tr>
                                <th>{ "Score Fwd" }</th>
                                <th>{ "+" }</th>
                                <th>{ "-" }</th>
                                <th>{ "Net" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { if_render_html(ctx.props().are_scores_expanded, player.scores().iter().enumerate().map(|(i, score)| {
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

    impl Player {
        fn sum_turn_scores(turn_scores: &[TurnScore]) -> i16 {
            turn_scores.iter().fold(0, |acc, ts| acc + ts.add - ts.sub)
        }
    }
}

mod rack {
    use nile::TileArray;

    use crate::components::{utils::if_render, HiddenTile, RackTile};

    use super::*;

    pub struct TileRack {}

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

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            let on_drag = Callback::from(move |e: DragEvent| {
                e.prevent_default();
            });
            html! {
                <table>
                    <tbody>
                        <tr>
                            { for ctx.props().tiles.iter().enumerate().map(|(i, tile)| {
                                let i = i as u8;
                                let on_drag_start = ctx.props().on_select.reform(move |e: DragEvent| {
                                    e.prevent_default();
                                    i
                                });
                                let on_touch_start = ctx.props().on_select.reform(move |e: TouchEvent| {
                                    e.prevent_default();
                                    i
                                });
                                let on_click = ctx.props().on_select.reform(move |e: MouseEvent| {
                                    e.prevent_default();
                                    i
                                });
                                html! {
                                    <td key={ format!("${:?} - ${}", tile, i) }>
                                        <div draggable={ if ctx.props().show_tiles { "on" } else { "off" }  }
                                            ondrag={ on_drag.clone() }
                                            ondragstart={ on_drag_start.clone() }
                                            ontouchstart={ on_touch_start.clone() }
                                            onclick={ on_click.clone() }
                                        >
                                            { if ctx.props().show_tiles {
                                                html! {
                                                    <RackTile tile={ *tile } is_selected={ matches!(ctx.props().selected_tile_idx, Some(idx) if idx == i) } />
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
                            { for ctx.props().tiles.iter().enumerate().map(|(i, tile)| { html! {
                                <td key={ format!("{:?} - {}", tile, i) }>
                                    { if_render(ctx.props().show_tiles, tile.score()) }
                                </td>
                            } }) }
                        </tr>
                    </tbody>
                </table>
            }
        }
    }
}
