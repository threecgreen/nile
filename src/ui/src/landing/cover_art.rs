use nile::{Rotation, TilePath, TilePathType};

use yew::prelude::*;

use crate::components::DisplayTile;

pub struct CoverArt {}

impl Component for CoverArt {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="cover-art">
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Center90) }
                    rotation={ Rotation::Clockwise270 }
                />
                <DisplayTile tile_path_type={ TilePathType::Universal(TilePath::Right45) }
                    rotation={ Rotation::Clockwise90 }
                    classes={ classes!("negative-margin") }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Corner90) }
                    rotation={ Rotation::Clockwise180 }
                    classes={ classes!("down-right") }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Corner90) }
                    rotation={ Rotation::None }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Right135) }
                    rotation={ Rotation::Clockwise180 }
                    classes={ classes!("down-right") }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Center90) }
                    rotation={ Rotation::Clockwise270 }
                    classes={ classes!("up") }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Straight) }
                    rotation={ Rotation::None }
                    classes={ classes!("negative-margin") }
                />
                <DisplayTile tile_path_type={ TilePathType::Normal(TilePath::Straight) }
                    rotation={ Rotation::None }
                    classes={ classes!("negative-margin") }
                />
            </div>
        }
    }
}
