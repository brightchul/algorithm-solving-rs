use std::io::{self, BufWriter, Stdout, Write};
use std::vec;

fn input_number() -> i32 {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<i32>().unwrap_or(0)
}

fn main() {
    let mut sout = io::BufWriter::new(io::stdout());

    loop {
        let x = input_number() * 10000000;
        if x == 0 {
            break;
        }

        solution(x, &mut sout);
    }
}

fn solution(x: i32, sout: &mut BufWriter<Stdout>) {
    let lego_len = input_number();
    let mut legos = vec![];

    for _ in 0..lego_len {
        legos.push(input_number());
    }
    legos.sort();

    let (mut left, mut right) = (0, lego_len - 1);

    while left < right {
        let temp = legos[left as usize] + legos[right as usize];
        if temp == x {
            writeln!(
                sout,
                "yes {} {}",
                legos[left as usize], legos[right as usize]
            )
            .unwrap();
            return;
        }
        if temp < x {
            left += 1;
        } else {
            right -= 1;
        }
    }
    writeln!(sout, "danger").unwrap();
}
