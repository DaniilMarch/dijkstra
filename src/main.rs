use dijkstra::{State, CellType, SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::*;

fn main() -> BError {
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT).expect("Error creating simple console").build()?;

    let from_x = 49;
    let from_y = 50;
    let to_x = 51;
    let to_y = 50;

    let mut game_state = State::new(from_x, from_y, to_x, to_y);
    game_state.set_weight_at(from_x, from_y, 0.0);
    game_state.set_type_at(50, 48, CellType::Wall);
    game_state.set_type_at(50, 49, CellType::Wall);
    game_state.set_type_at(50, 50, CellType::Wall);
    game_state.set_type_at(50, 51, CellType::Wall);
    game_state.set_type_at(50, 52, CellType::Wall);
    game_state.set_type_at(51, 48, CellType::Wall);
    game_state.set_type_at(51, 48, CellType::Wall);
    game_state.set_type_at(52, 48, CellType::Wall);
    game_state.set_type_at(53, 48, CellType::Wall);
    game_state.set_type_at(54, 48, CellType::Wall);
    game_state.set_type_at(55, 48, CellType::Wall);
    game_state.set_type_at(51, 52, CellType::Wall);
    game_state.set_type_at(52, 52, CellType::Wall);
    game_state.set_type_at(53, 52, CellType::Wall);
    game_state.set_type_at(54, 52, CellType::Wall);
    game_state.set_type_at(55, 52, CellType::Wall);


    main_loop(context, game_state)
}
