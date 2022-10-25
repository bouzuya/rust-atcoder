use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let ds = n
        .to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let mut sum = (0, 0);
    let mut count = (1, 0);
    for d in ds {
        let mut next_sum = (0, 0);
        let mut next_count = (0, 0);
        for x in 0..=9 {
            match x.cmp(&d) {
                std::cmp::Ordering::Less => {
                    next_count.1 += count.0;
                    next_count.1 += count.1;
                    next_sum.1 += sum.0 + count.0 * x;
                    next_sum.1 += sum.1 + count.1 * x;
                }
                std::cmp::Ordering::Equal => {
                    next_count.0 += count.0;
                    next_count.1 += count.1;
                    next_sum.0 += sum.0 + count.0 * x;
                    next_sum.1 += sum.1 + count.1 * x;
                }
                std::cmp::Ordering::Greater => {
                    next_count.1 += count.1;
                    next_sum.1 += sum.1 + count.1 * x;
                }
            }
        }
        sum = next_sum;
        count = next_count;
    }

    let ans = sum.0 + sum.1;
    println!("{}", ans);
}
