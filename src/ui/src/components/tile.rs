use yew::prelude::*;

use nile::{Rotation, Tile, TilePathType};

use crate::colors;

use super::tile_svg::TileSvg;
use super::utils::update_if_changed;

pub mod rack_tile {
    use super::*;

    pub struct RackTile {
        props: Props,
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub tile: Tile,
        pub is_selected: bool,
    }

    impl Component for RackTile {
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
            html! {
                <div class=classes!("cell", "tile", self.props.is_selected.then(|| "selected"))>
                    <TileSvg tile={ self.props.tile } />
              </div>
            }
        }
    }
}

pub mod tile_cell {
    use nile::{Rotation, TilePathType};

    use super::*;

    #[derive(Clone, Copy, PartialEq)]
    pub enum TileCellType {
        Normal,
        Bonus,
        Penalty,
        EndGame,
    }

    pub struct TileCell {
        props: Props,
    }

    #[derive(Clone, Copy, PartialEq)]
    #[repr(u8)]
    pub enum Selection {
        Locked,
        Selectable,
        Selected,
    }

    #[derive(Clone, Properties)]
    pub struct Props {
        pub tile_path_type: TilePathType,
        pub rotation: Rotation,
        pub tile_cell_type: TileCellType,
        pub selection: Selection,
        pub is_error: bool,
        pub on_select: Callback<()>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            // No callbacks
            self.tile_path_type == other.tile_path_type
                && self.rotation == other.rotation
                && self.tile_cell_type == other.tile_cell_type
                && self.selection == other.selection
                && self.is_error == other.is_error
        }
    }

    impl Component for TileCell {
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
            let is_selectable = self.props.selection != Selection::Locked;
            let on_click = {
                let on_select = self.props.on_select.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    if is_selectable {
                        on_select.emit(());
                    }
                })
            };
            let on_drag = Callback::from(|e: DragEvent| {
                e.prevent_default();
            });
            let on_drag_start = {
                let on_select = self.props.on_select.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    if is_selectable {
                        on_select.emit(());
                    }
                })
            };
            let selected_css_class = match self.props.selection {
                Selection::Selected => Some("selected"),
                _ => None,
            };
            let universal_css_class = match self.props.tile_path_type {
                TilePathType::Universal(_) => Some("universal"),
                _ => None,
            };
            html! {
                <div
                    class=classes!(
                        "cell", "tile", selected_css_class, universal_css_class, self.props.is_error.then(|| "has-error"),
                        tile_cell_type_to_class(self.props.tile_cell_type)
                    )
                    style={ rotation_to_css(self.props.rotation) }
                    onclick={ on_click }
                    draggable={ is_selectable.to_string() }
                    ondrag={ on_drag }
                    ondragstart={ on_drag_start }
                >
                    { view_tile_path_type(self.props.tile_path_type) }
                </div>
            }
        }
    }

    const fn tile_cell_type_to_class(tile_cell_type: TileCellType) -> Option<&'static str> {
        match tile_cell_type {
            TileCellType::Normal => None,
            TileCellType::Bonus => Some("bonus"),
            TileCellType::Penalty => Some("penalty"),
            TileCellType::EndGame => Some("end-game"),
        }
    }
}

pub mod empty_cell {
    use crate::components::tile_svg::EndOfGameDot;

    use super::*;

    pub struct EmptyCell {
        props: Props,
    }

    #[derive(Clone, Properties)]
    pub struct Props {
        pub bonus: i16,
        pub is_end_game: bool,
        pub is_error: bool,
        pub on_drop: Callback<()>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            // No callback
            self.bonus == other.bonus
                && self.is_end_game == other.is_end_game
                && self.is_error == other.is_error
        }
    }

    impl Component for EmptyCell {
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
            let on_drag_over = Callback::from(|e: DragEvent| e.prevent_default());
            let on_drop = self
                .props
                .on_drop
                .reform(move |e: DragEvent| e.prevent_default());
            let on_click = self
                .props
                .on_drop
                .reform(move |e: MouseEvent| e.prevent_default());
            html! {
                <div class=classes!(
                        "cell", bonus_to_class(self.props.bonus), self.props.is_error.then(|| "has-error"),
                        self.props.is_end_game.then(|| "end-game")
                    )
                    ondragover={ on_drag_over }
                    ondrop={ on_drop }
                    onclick={ on_click }
                >
                    { if self.props.is_end_game {
                        html! { <EndOfGameDot /> }
                    } else { html!{} } }
                    { bonus_to_html(self.props.bonus) }
                </div>
            }
        }
    }

    const fn bonus_to_class(bonus: i16) -> Option<&'static str> {
        match bonus {
            0 => None,
            b if b > 0 => Some("bonus"),
            _ => Some("penalty"),
        }
    }

    fn bonus_to_html(bonus: i16) -> Html {
        if bonus == 0 {
            html! {}
        } else {
            html! {
                <p>{ bonus.abs() }</p>
            }
        }
    }
}

pub struct HiddenTile {}

impl Component for HiddenTile {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("cell", "tile", "hidden-tile") />
        }
    }
}

pub mod display {
    use nile::{Rotation, TilePathType};

    use super::*;

    pub struct DisplayTile {
        props: Props,
    }

    #[derive(Clone, Properties, PartialEq)]
    pub struct Props {
        pub tile_path_type: TilePathType,
        pub rotation: Rotation,
        #[prop_or_default]
        pub classes: Classes,
    }

    impl Component for DisplayTile {
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
            let is_universal = matches!(self.props.tile_path_type, TilePathType::Universal(_));
            html! {
                <div
                    class=classes!(
                        self.props.classes.clone(),
                        "cell",
                        "tile",
                        "display-tile",
                        "has-tile",
                        is_universal.then(|| "universal"),
                    )
                    style={ rotation_to_css(self.props.rotation) }
                >
                    { view_tile_path_type(self.props.tile_path_type) }
                </div>
            }
        }
    }
}

const fn rotation_to_css(rotation: Rotation) -> &'static str {
    match rotation {
        Rotation::None => "",
        Rotation::Clockwise90 => "transform: rotate(90deg)",
        Rotation::Clockwise180 => "transform: rotate(180deg)",
        Rotation::Clockwise270 => "transform: rotate(270deg)",
    }
}

fn view_tile_path_type(tile_path_type: TilePathType) -> Html {
    match tile_path_type {
        TilePathType::Normal(tp) => {
            let tile = Tile::from(tp);
            html! {
                <TileSvg tile={ tile }
                    stroke_color={ colors::RIVER_PATH_STROKE }
                />
            }
        }
        TilePathType::Universal(tp) => {
            let tile = Tile::from(tp);
            html! {
                <>
                    <TileSvg tile={ Tile::Universal }
                        stroke_color={ colors::UNIVERSAL_TILE_STROKE }
                    />
                    <TileSvg tile={ tile }
                        stroke_color={ colors::RIVER_PATH_STROKE }
                    />
                </>
            }
        }
    }
}
