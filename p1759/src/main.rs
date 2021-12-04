use p1759::solution;

extern crate util;

fn main() {
    let number_vec = util::input_number_vec::<usize>();
    let mut string_vec = util::input_string_vec();
    string_vec.sort();

    let pwd_len = number_vec[0];
    let mut out_buf = String::from("");
    solution(&string_vec, 0, pwd_len, String::from(""), &mut out_buf);

    println!("{}", out_buf);
}
