use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let digits = n
        .iter()
        .map(|&c| (c as u8 - '0' as u8) as usize)
        .collect::<Vec<usize>>();
    let mut min_count = None;
    for bits in 0..1 << n.len() {
        let bs = (0..n.len())
            .map(|i| (bits >> i) & 1 == 1)
            .collect::<Vec<bool>>();
        let sum = digits
            .iter()
            .zip(bs.iter())
            .filter(|&(_, &b)| b)
            .map(|(i, _)| i)
            .sum::<usize>();
        if sum >= 3 && sum % 3 == 0 {
            let count = n.len() - bs.iter().filter(|&&b| b).count();
            min_count = match min_count {
                Some(c) => {
                    if c < count {
                        Some(c)
                    } else {
                        Some(count)
                    }
                }
                None => Some(count),
            };
        }
    }
    println!("{}", min_count.map(|c| c as i64).unwrap_or(-1));
}
