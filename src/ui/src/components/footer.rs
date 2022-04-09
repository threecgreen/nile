use super::container::Container;
use super::tile_svg::TileSvg;
use nile::Tile;
use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        html! {
            <div class={ classes!("footer-background") }>
                <Container>
                    <div class="footer-flex">
                        <div>
                        <p class="copyright">{ "© 2020–2021 Carter Green" }</p>
                        </div>
                        <div>
                            <div class="logo">
                                <TileSvg tile={ Tile::Universal} />
                            </div>
                        </div>
                        <div>
                        <p class="version">
                            { format!("Version: {}", VERSION) }
                        </p>
                        </div>
                    </div>
                </Container>
            </div>
        }
    }
}
