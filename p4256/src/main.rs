use std::io::{self, BufWriter, Stdout, Write};

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<i32>().unwrap();
}

fn input_number_vector() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|v| v.parse::<i32>().unwrap())
        .collect()
}

fn find_value(value: i32, numbers: &[i32]) -> usize {
    return (numbers).iter().position(|&v| v == value).unwrap();
}

fn recur(pre_order: &[i32], in_order: &[i32], sout: &mut BufWriter<Stdout>) {
    let len = in_order.len();

    if len == 0 {
        return;
    }
    if len == 1 {
        write!(sout, "{} ", pre_order[0]).unwrap();
        return;
    }

    let root_value = pre_order[0];
    let root_idx = find_value(root_value, in_order);

    recur(&pre_order[1..(root_idx + 1)], &in_order[0..root_idx], sout);
    recur(
        &pre_order[(root_idx + 1)..],
        &in_order[root_idx + 1..len],
        sout,
    );
    write!(sout, "{} ", root_value).unwrap();
}

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    let count = input_number();

    for _ in 0..count {
        let node_length = input_number() as usize;
        let pre_order = input_number_vector();
        let in_order = input_number_vector();

        let root_value = pre_order[0];
        let root_idx = find_value(root_value, &in_order);

        recur(
            &pre_order[1..(root_idx + 1)],
            &in_order[0..root_idx],
            &mut sout,
        );

        recur(
            &pre_order[(root_idx + 1)..],
            &in_order[root_idx + 1..node_length],
            &mut sout,
        );
        writeln!(sout, "{}", root_value).unwrap();
    }
}

// fn recur1(
//     pre_start: usize,
//     pre_end: usize,
//     in_start: usize,
//     in_end: usize,
//     map: &HashMap<i32, usize>,
//     pre_order: &Vec<i32>,
//     sout: &mut BufWriter<Stdout>,
// ) {
//     if pre_start > pre_end {
//         return;
//     }
//     if pre_start == pre_end {
//         write!(sout, "{} ", pre_order[pre_start]).unwrap();
//         return;
//     }

//     let root_value = pre_order[pre_start];
//     let root_idx = map[&root_value];
//     let left_dis = root_idx - in_start;

//     if root_idx > 0 {
//         recur1(
//             pre_start + 1,
//             pre_start + left_dis,
//             in_start,
//             root_idx - 1,
//             map,
//             pre_order,
//             sout,
//         );
//     }
//     if pre_start + left_dis + 1 <= pre_end {
//         recur1(
//             pre_start + left_dis + 1,
//             pre_end,
//             root_idx + 1,
//             in_end,
//             map,
//             pre_order,
//             sout,
//         );
//     }
//     write!(sout, "{} ", root_value).unwrap();
// }

// fn main1() {
//     let mut sout = io::BufWriter::new(io::stdout());

//     let count = input_number();

//     for _ in 0..count {
//         let node_length = input_number() as usize;
//         let pre_order = input_number_vector();
//         let in_order = input_number_vector();

//         let mut in_order_num_map: HashMap<i32, usize> = HashMap::new();
//         for (idx, &value) in in_order.iter().enumerate() {
//             in_order_num_map.insert(value, idx);
//         }

//         let root_value = pre_order[0];
//         let root_idx = in_order_num_map[&root_value];

//         let left_dis = root_idx - 0;

//         recur1(
//             1,
//             1 + left_dis - 1,
//             0,
//             root_idx - 1,
//             &in_order_num_map,
//             &pre_order,
//             &mut sout,
//         );

//         recur1(
//             1 + left_dis,
//             node_length - 1,
//             root_idx + 1,
//             node_length - 1,
//             &in_order_num_map,
//             &pre_order,
//             &mut sout,
//         );
//         writeln!(sout, "{}", root_value).unwrap();
//     }
// }
