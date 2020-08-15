use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    };
    let mut ans = 0_i64;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if l[i] != l[j]
                    && l[i] != l[k]
                    && l[j] != l[k]
                    && std::cmp::max(std::cmp::max(l[i], l[j]), l[k])
                        < (l[i] + l[j] + l[k] - std::cmp::max(std::cmp::max(l[i], l[j]), l[k]))
                {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
