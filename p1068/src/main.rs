use p1068::{get_number_io, get_vec_i32_io, solution};

fn main() {
    let _len = get_number_io();
    let list = get_vec_i32_io();
    let remove_node = get_number_io() as usize;

    println!("{}", solution(list, remove_node));
}
