mod common_programming_concepts_3;
mod common_collections_8;
mod generic_types_traits_and_lifetimes_10;
mod an_io_project_building_a_command_line_program_12;
mod functional_language_features_iterators_and_closures_13;
mod more_about_cargo_and_crates_io_14;
mod object_oriented_programming_features_of_rust_17;
mod final_project_building_a_multithreaded_web_server_20;

use final_project_building_a_multithreaded_web_server_20::final_project::web_server;


fn main() {
    web_server::run_server()
}
