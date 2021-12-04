use std::{io, str::FromStr};

pub fn input_number_vec<T>() -> Vec<T>
where
    T: FromStr,
{
    let input_str = input_string();
    let mut result_list = Vec::<T>::new();

    for one in input_str.split_whitespace() {
        if let Ok(parsed_value) = one.parse::<T>() {
            result_list.push(parsed_value);
        } else {
            panic!("invalid numberic string");
        }
    }

    result_list
}

pub fn input_number<T>() -> T
where
    T: FromStr,
{
    if let Ok(result) = input_string().parse::<T>() {
        return result;
    }
    panic!("invalid numberic string");
}

pub fn input_string() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let result = buf.trim();

    result.to_string()
}
