use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let n = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    let k = n.len();
    let mut ans = k + 1;
    for bits in 1_usize..1 << k {
        let mut sum = 0_usize;
        for i in 0..k {
            if (bits >> i) & 1 == 1 {
                sum += n[i];
                sum %= 3;
            }
        }
        if sum == 0 {
            ans = ans.min(k - bits.count_ones() as usize);
        }
    }
    if ans >= k {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
