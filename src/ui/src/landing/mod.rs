mod button;
mod cover_art;
mod game_form;
mod header;

use yew::prelude::*;

use crate::{
    app,
    components::{utils::update_if_changed, Container},
};
use button::{ClickButton, LinkButton};
use game_form::GameForm;
use header::Header;

pub struct Landing {
    props: Props,
}

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
        let show_new_game_form = self
            .props
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
                { if self.props.should_show_new_game_form { html! {
                    <section>
                        <h3 class="section-title">{ "new game" }</h3>
                        <GameForm player_names={ self.props.player_names.clone() }
                            cpu_player_count={ self.props.cpu_player_count }
                            dispatch={ self.props.dispatch.clone() }
                        />
                    </section>
                } } else { html!{} } }
                // TODO: narrow text to width like NYT website and to approximately match width of `Header`
                <section>
                    <h3 class="section-title">
                        <a id="about">{ "about" }</a>
                    </h3>
                    <p>
                        { "A web version of a 1960s tile-based board game, in nile players take turns extending the course of the river, getting bonuses, and setting up opponents for penalties." }
                    </p>
                    <p>
                        { "Play against other people, the AI, or a mix. Supports 2–4 players." }
                    </p>
                </section>

                <h3 class="section-title">
                    <a id="how-to-play">{ "how to play" }</a>
                    // FIXME: write
                </h3>
                <section>

                </section>
            </Container>
        }
    }
}
