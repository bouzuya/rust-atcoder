use proconio::input;

fn main() {
    input! {
        n: usize,
        case: [u64; n],
    };
    for case_i in case {
        // N = 2^d * x
        let ans = match case_i % 2 {
            0 => match (case_i / 2) % 2 {
                0 => "Even",
                1 => "Same",
                _ => unreachable!(),
            },
            1 => "Odd",
            _ => unreachable!(),
        };
        println!("{}", ans);
    }
}
