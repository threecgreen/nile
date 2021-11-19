// use nile::Cell;
// use yew::prelude::*;

// struct BoardCell {
//     props: Props,
// }

// #[derive(Clone, Properties)]
// struct Props {
//     cell: Cell,
//     is_end_game: bool,
//     is_selected: bool,
//     is_from_current_turn: bool,
//     on_drop_from_rack: Callback<()>,
//     on_select: Callback<()>,
// }

// impl Component for BoardCell {
//     type Message = ();
//     type Properties = Props;

//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         todo!()
//     }

//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         todo!()
//     }

//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//     }

//     fn view(&self) -> Html {
//         match self.props.cell.tile {
//             Some(tile_placement)
//         }
//         html! {
//             <td>
//             </td>
//         }
//     }
// }
