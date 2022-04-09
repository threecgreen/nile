mod button;
mod cover_art;
mod game_form;
mod header;

use nile::Tile;
use yew::prelude::*;

use crate::{
    app,
    components::{Container, EmptyCell, RackTile},
};
use button::{ClickButton, LinkButton};
use game_form::GameForm;
use header::Header;

pub struct Landing {}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub player_names: Vec<String>,
    pub cpu_player_count: u8,
    pub should_show_new_game_form: bool,
    pub dispatch: Callback<app::Msg>,
}

impl Component for Landing {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let show_new_game_form = ctx
            .props()
            .dispatch
            .reform(|_| app::Msg::SetShouldShowNewGameForm(true));
        html! {
            <Container>
                <Header />
                <div class="center-content">
                    <LinkButton href="#about">
                        { "about" }
                    </LinkButton>
                    <LinkButton href="#how-to-play">
                        { "how to play" }
                    </LinkButton>
                    <ClickButton on_click={ show_new_game_form }>
                        { "new game" }
                    </ClickButton>
                </div>
                { if ctx.props().should_show_new_game_form { html! {
                    <section>
                        <h3 class="section-title">{ "new game" }</h3>
                        <GameForm player_names={ ctx.props().player_names.clone() }
                            cpu_player_count={ ctx.props().cpu_player_count }
                            dispatch={ ctx.props().dispatch.clone() }
                        />
                    </section>
                } } else { html!{} } }
                // TODO: narrow text to width like NYT website and to approximately match width of `Header`
                <section class="landing-section">
                    <h3>
                        <a id="about">{ "about" }</a>
                    </h3>
                    <p>
                        { "A web version of a 1960s tile-based board game, in nile players take
                           turns extending the course of the river, getting bonuses, and setting up
                           opponents for penalties." }
                    </p>
                    <p>
                        { "Play against other people, the AI, or a mix. Supports 2–4 players." }
                    </p>
                </section>

                <section class="landing-section">
                    <h3 class="section-title">
                        <a id="how-to-play">{ "how to play" }</a>
                    </h3>
                    <p>
                        { "The goal of nile is to outscore your opponents through skillful placement
                           of tiles to form the path of the river. All players contribute to the
                           same river, a continuous path. " }
                    </p>
                    { Self::view_starting_the_game() }
                    <h4>{ "how to place tiles" }</h4>
                    <p>
                        { "When placing a tile, the path of the tile being placed must match
                           up with the previous placed tile, to form a smooth path. Two tiles have
                           paths going from side to side, two from corner to corner, and four going
                           from side to corner."}
                    </p>

                    <h5>{ "the universal tile" }</h5>
                    <p>
                        { "The universal tile can act as any one of the other tiles. When playing
                           a universal tile, you can select with tile you want to to act as from
                           a dropdown." }
                    </p>
                    <div class="center-content">
                        <RackTile tile={ Tile::Universal } is_selected={ false } />
                    </div>

                    <h5>{ "encirclement" }</h5>
                    <p>{ "No tile can be placed if it would block all paths for the river to reach
                          the blue end-of-game squares." }</p>

                    <h4>{ "scoring" }</h4>
                    <p>{ "The following add points to your score:" }</p>
                    <ul>
                        <li>{ "point values of the tiles played during a turn" }</li>
                        <li>{ "bonus squares covered in a turn" }</li>
                        <li>{ "blue end-of-game squares covered" }</li>
                        <li>{ "playing all tiles in a turn results in a 20-point bonus" }</li>
                    </ul>
                    <p>{ "The following subtract points from your score:" }</p>
                    <ul>
                        <li>{ "penalty squares covered in a turn" }</li>
                        <li>{ "inability to play results in a deduction of the sum of the point
                               values of the tiles" }</li>
                    </ul>
                </section>
            </Container>
        }
    }
}

impl Landing {
    fn view_starting_the_game() -> Html {
        const FIRST_PLACEMENT_TILES: [Tile; 5] = [
            Tile::Straight,
            Tile::Center90,
            Tile::Left45,
            Tile::Right45,
            Tile::Universal,
        ];
        const OTHER_TILES: [Tile; 4] = [
            Tile::Diagonal,
            Tile::Left135,
            Tile::Right135,
            Tile::Corner90,
        ];
        html! {
            <>
                <h4>{ "starting the game" }</h4>
                <p>
                    { "Each player starts with five tiles. The first player begins by placing a
                       tile on the starting square adjacent to the arrow labeled “" }
                    <span class="start">{ "start" }</span>
                    { "”. Only the following tiles align with the start arrow and don’t direct
                       the river off the board. Their point values are listed below each tile."}
                </p>
                { Self::view_tile_table(FIRST_PLACEMENT_TILES) }
                <p>{ "The other types of tiles and their point values are" }</p>
                { Self::view_tile_table(OTHER_TILES) }
                <p>
                    { "If the player can place a tile correctly to start the game, they’re not
                       obligated to place any more tiles that turn. However, if they use all five
                       tiles, they receive a bonus of 20 points. "}
                </p>
                <p>
                    { "If a player does not have one of the tiles that can start the game, they
                       forfeit their turn. "}
                </p>
                <p>
                    { "If a player places a tile on a green square, they receive a bonus of the
                       number of the points listed in the square. Orange squares are the opposite
                       and players are penalized for playing there."}
                </p>
                <div class="center-content">
                    <table class="board"><tbody><tr>
                        <td>
                            <EmptyCell bonus={ 60 }
                                is_end_game={ false }
                                is_error={ false }
                                on_drop={ Callback::default() }
                            />
                        </td>
                        <td>
                            <EmptyCell bonus={ -40 }
                                is_end_game={ false }
                                is_error={ false }
                                on_drop={ Callback::default() }
                            />
                        </td>
                    </tr></tbody></table>
                </div>
            </>
        }
    }

    fn view_tile_table<const N: usize>(tiles: [Tile; N]) -> Html {
        html! {
            <div class="center-content">
                <table>
                    <tbody>
                        <tr>
                            { for tiles.map(|t| {
                                html! {
                                    <td><RackTile tile={ t } is_selected={ false } /></td>
                                }
                            }) }
                        </tr>
                        <tr>
                            { for tiles.map(|t| {
                                html! {
                                    <td class="tile-score">{ t.score() }</td>
                                }
                            }) }
                        </tr>
                    </tbody>
                </table>
            </div>
        }
    }
}
