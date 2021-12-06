use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let s = "DiscoPresentsDiscoveryChannelProgrammingContest2016"
        .chars()
        .collect::<Vec<_>>();
    let mut t = vec![];
    for (i, s_i) in s.into_iter().enumerate() {
        t.push(s_i);
        if (i + 1) % n == 0 {
            t.push('\n');
        }
    }
    let ans = t.into_iter().collect::<String>();
    println!("{}", ans.trim_end());
}
