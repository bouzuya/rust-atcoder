use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n],
    };
    let mut b = vec![0; n];
    for (i, &a_i) in a.iter().enumerate().rev() {
        let k = i + 1;
        let s_i = {
            let mut sum = 0;
            for j in (k + k..n + 1).step_by(k) {
                sum += b[j - 1];
            }
            sum % 2 == 0
        };
        match (a_i == 0, s_i) {
            (false, false) | (true, true) => {}
            (false, true) | (true, false) => b[i] = 1,
        }
    }

    let b = b
        .iter()
        .enumerate()
        .filter(|&(_, &b_i)| b_i == 1)
        .map(|(i, _)| i + 1)
        .collect::<Vec<usize>>();
    println!("{}", b.len());
    for (i, &b_i) in b.iter().enumerate() {
        print!("{}{}", b_i, if i == b.len() - 1 { "\n" } else { " " });
    }
}
