use nile::Tile;
use yew::prelude::*;

use self::svg_wrapper::SvgWrapper;

pub struct TileSvg {}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub tile: nile::Tile,
    #[prop_or("#000000")]
    pub stroke_color: &'static str,
}

impl Component for TileSvg {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().tile {
            Tile::Straight => self.straight(ctx),
            Tile::Diagonal => self.diagonal(ctx),
            Tile::Center90 => self.center90(ctx),
            Tile::Corner90 => self.corner90(ctx),
            Tile::Left45 => self.left45(ctx),
            Tile::Right45 => self.right45(ctx),
            Tile::Left135 => self.left135(ctx),
            Tile::Right135 => self.right135(ctx),
            Tile::Universal => self.universal(ctx),
        }
    }
}

impl TileSvg {
    fn straight(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper>
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="0" y1="20" x2="40" y2="20" />
            </SvgWrapper>
        }
    }
    fn diagonal(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper>
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="40" y1="0" x2="0" y2="40" />
            </SvgWrapper>
        }
    }
    fn center90(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper>
                <path fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" d="M20,40c0-11.055-8.945-20-20-20"/>
            </SvgWrapper>
        }
    }
    fn corner90(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper>
                <path fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" d="M40,40C28.986,28.986,11.163,28.986,0.148,40"/>
            </SvgWrapper>
        }
    }
    fn left45(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper style={ Self::reflect_to_css(true) }>
                { self.tile45(ctx) }
            </SvgWrapper>
        }
    }
    fn right45(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper style={ Self::reflect_to_css(false) }>
                { self.tile45(ctx) }
            </SvgWrapper>
        }
    }
    fn tile45(&self, ctx: &Context<Self>) -> Html {
        html! {
            <path fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" d="M19.938,40.063c0-27.636,22.363-50,50-50" />
        }
    }
    fn left135(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper style={ Self::reflect_to_css(false) }>
                { self.tile135(ctx) }
            </SvgWrapper>
        }
    }
    fn right135(&self, ctx: &Context<Self>) -> Html {
        html! {
            // For 45, left is reflected, but for 135 right is
            <SvgWrapper style={ Self::reflect_to_css(true) }>
                { self.tile135(ctx) }
            </SvgWrapper>
        }
    }
    fn tile135(&self, ctx: &Context<Self>) -> Html {
        html! {
            <path fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" d="M0,40l15.725-15.725
                c0.444-0.527,1.11-0.862,1.854-0.862c1.337,0,2.422,1.084,2.422,2.422L20,40"
            />
        }
    }
    fn universal(&self, ctx: &Context<Self>) -> Html {
        html! {
            <SvgWrapper>
                <circle fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" cx="20" cy="20" r="5" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="23.535" y1="23.535" x2="40" y2="40" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="0" y1="0" x2="16.466" y2="16.466" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="16.464" y1="23.535" x2="0" y2="40" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="40" y1="0" x2="23.535" y2="16.464" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="15" y1="20" x2="0" y2="20" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="40" y1="20" x2="25" y2="20" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="20" y1="25" x2="20" y2="40" />
                <line fill="none" stroke={ ctx.props().stroke_color } stroke-width="3" stroke-miterlimit="10" x1="20" y1="0" x2="20" y2="15" />
            </SvgWrapper>
        }
    }

    const fn reflect_to_css(should_reflect: bool) -> &'static str {
        if should_reflect {
            "transform: scaleX(-1)"
        } else {
            ""
        }
    }
}

mod svg_wrapper {
    use yew::{html, Children, Component, Context, Properties};

    pub struct SvgWrapper {}

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        #[prop_or("")]
        pub style: &'static str,
        #[prop_or_default]
        pub children: Children,
    }

    impl Component for SvgWrapper {
        type Message = ();
        type Properties = Props;

        fn create(_ctx: &Context<Self>) -> Self {
            Self {}
        }

        fn view(&self, ctx: &Context<Self>) -> yew::Html {
            html! {
                <svg viewBox="0 0 40 40" style={ ctx.props().style }>
                    { ctx.props().children.clone() }
                </svg>
            }
        }
    }
}

/// Dot that indicates how the river should align in the end-of-game cells
pub struct EndOfGameDot {}

impl Component for EndOfGameDot {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <svg viewBox="0 0 40 40" class="end-game-dot">
                <circle cx="37.5" cy="50%" r="2.5" />
            </svg>
        }
    }
}
