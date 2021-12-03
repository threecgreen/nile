use yew::prelude::*;

use nile::Tile;

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
            let mut classes = vec!["tile"];
            if self.props.is_selected {
                classes.push("selected");
            }
            html! {
                <div class={ classes!("tile", if self.props.is_selected { "selected" } else { "" }) }>
                    <TileSvg tile={ self.props.tile } />
              </div>
            }
        }
    }
}

pub mod tile_cell {
    use nile::{console, Rotation, TilePathType};

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

    #[derive(Clone, PartialEq)]
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
        pub on_select: Callback<()>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            // No callbacks
            self.tile_path_type == other.tile_path_type
                && self.rotation == other.rotation
                && self.tile_cell_type == other.tile_cell_type
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
            console::debug("Rendering tile cell");
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
                Selection::Selected => "selected",
                _ => "",
            };
            let universal_css_class = match self.props.tile_path_type {
                TilePathType::Universal(_) => "universal",
                _ => "",
            };
            html! {
                <div
                    class={ classes!("tile", selected_css_class, universal_css_class, tile_cell_type_to_class(self.props.tile_cell_type)) }
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

    fn view_tile_path_type(tile_path_type: TilePathType) -> Html {
        match tile_path_type {
            TilePathType::Normal(tp) => {
                let tile = Tile::from(tp);
                html! {
                    <TileSvg tile={ tile }
                        stroke_color={ "royalblue" }
                    />
                }
            }
            TilePathType::Universal(tp) => {
                let tile = Tile::from(tp);
                html! {
                    <>
                        <TileSvg tile={ Tile::Universal }
                            stroke_color="#aaaaaa"
                        />
                        <TileSvg tile={ tile }
                            stroke_color={ "royalblue" }
                        />
                    </>
                }
            }
        }
    }

    const fn tile_cell_type_to_class(tile_cell_type: TileCellType) -> &'static str {
        match tile_cell_type {
            TileCellType::Normal => "",
            TileCellType::Bonus => "bonus",
            TileCellType::Penalty => "penalty",
            TileCellType::EndGame => "end-game",
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
}

pub mod empty_cell {
    use nile::console;

    use super::*;

    pub struct EmptyCell {
        props: Props,
    }

    #[derive(Clone, Properties)]
    pub struct Props {
        pub bonus: i16,
        pub is_end_game: bool,
        pub on_drop: Callback<()>,
    }

    impl PartialEq for Props {
        fn eq(&self, other: &Self) -> bool {
            // No callback
            self.bonus == other.bonus && self.is_end_game == other.is_end_game
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
            console::debug("Rendering tile cell");
            let end_game_class = if self.props.is_end_game {
                "end-game"
            } else {
                ""
            };
            let on_drag_over = Callback::from(|e: DragEvent| e.prevent_default());
            let on_drop = {
                let on_drop = self.props.on_drop.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    on_drop.emit(());
                })
            };
            let on_click = {
                let on_drop = self.props.on_drop.clone();
                Callback::from(move |e: MouseEvent| {
                    e.prevent_default();
                    on_drop.emit(());
                })
            };
            html! {
                <div class={ classes!("tile", bonus_to_class(self.props.bonus), end_game_class) }
                    ondragover={ on_drag_over }
                    ondrop={ on_drop }
                    onclick={ on_click }
                >
                    { bonus_to_html(self.props.bonus) }
                </div>
            }
        }
    }

    const fn bonus_to_class(bonus: i16) -> &'static str {
        match bonus {
            0 => "",
            b if b > 0 => "bonus",
            _ => "penalty",
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
            <div class={ classes!("tile", "hidden-tile") } />
        }
    }
}
