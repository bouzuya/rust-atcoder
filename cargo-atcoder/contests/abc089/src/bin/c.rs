use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let chars = "MARCH".chars().collect::<Vec<char>>();
    let mut count = vec![0_usize; chars.len()];
    for s_i in s {
        for (j, c) in chars.iter().copied().enumerate() {
            if s_i.starts_with(c) {
                count[j] += 1;
            }
        }
    }
    let mut ans = 0_usize;
    for i in 0..5 {
        for j in i + 1..5 {
            for k in j + 1..5 {
                ans += count[i] * count[j] * count[k];
            }
        }
    }

    println!("{}", ans);
}
