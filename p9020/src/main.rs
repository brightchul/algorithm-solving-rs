use std::io::{self, Write};

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    let mut num_arr = [true; 10001];
    let mut prime_vec: Vec<usize> = vec![];

    num_arr[0] = false;
    num_arr[1] = false;

    for i in 2..101 {
        let mut j = i * i;
        while j < 10001 {
            num_arr[j] = false;
            j += i;
        }
    }

    for (idx, &is_prime) in num_arr.iter().enumerate() {
        if is_prime {
            prime_vec.push(idx);
        }
    }

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let len = buf.trim().parse::<usize>().unwrap();

    for _ in 0..len {
        buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
        let target = buf.trim().parse::<usize>().unwrap();

        let idx = prime_vec.iter().position(|v| *v >= target / 2).unwrap();
        let mut left = idx;
        let mut right = idx;

        loop {
            let total = prime_vec[left] + prime_vec[right];
            if total == target {
                break;
            }
            if total < target {
                right += 1;
            } else {
                left -= 1;
            }
        }

        writeln!(sout, "{} {}", prime_vec[left], prime_vec[right]).unwrap();
    }
}
