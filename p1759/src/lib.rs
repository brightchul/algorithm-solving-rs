pub fn solution(
    vec: &Vec<String>,
    start_idx: usize,
    pwd_len: usize,
    result: String,
    out_buf: &mut String,
) {
    if result.len() == pwd_len {
        if check_condition(&result) {
            out_buf.push_str(&result);
            out_buf.push_str("\n");
        }
        return;
    }

    let end_idx = vec.len() - (pwd_len - result.len() - 1);

    for idx in start_idx..end_idx {
        let cur = result.to_string() + &vec[idx];
        solution(vec, idx + 1, pwd_len, cur, out_buf);
    }
}

pub fn check_condition(target: &str) -> bool {
    let vowel_list = vec!['a', 'e', 'i', 'o', 'u'];
    let (mut vowel_count, mut con_count) = (0, 0);

    for char_value in target.chars() {
        if vowel_list.contains(&char_value) {
            vowel_count += 1;
        } else {
            con_count += 1;
        }
    }

    vowel_count >= 1 && con_count >= 2
}
