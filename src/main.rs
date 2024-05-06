use crate::hello_world::hello_world::hello_world;
use crate::guessing_game::guessing_game::guessing_game;

mod hello_world;
mod guessing_game;
mod common_programming_concepts;

fn main() {
    hello_world();
    guessing_game();
    common_programming_concepts::common_programming_concepts::variables();
    common_programming_concepts::common_programming_concepts::data_types();
    common_programming_concepts::common_programming_concepts::functions();

    common_programming_concepts::common_programming_concepts::control_flow();
    common_programming_concepts::common_programming_concepts::print_even_numbers(10);
    let counter = common_programming_concepts::common_programming_concepts::return_value_from_loop();
    print!("\nCounter value is = {}", counter);
    common_programming_concepts::common_programming_concepts::loop_labels();
    common_programming_concepts::common_programming_concepts::while_loop();
    common_programming_concepts::common_programming_concepts::for_loop();
}
