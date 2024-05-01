use crate::hello_world::hello_world::hello_world;
use crate::guessing_game::guessing_game::guessing_game;

mod hello_world;
mod guessing_game;
mod common_programming_concepts;

fn main() {
    hello_world();
    guessing_game();
    common_programming_concepts::common_programming_concepts::variables();
}
