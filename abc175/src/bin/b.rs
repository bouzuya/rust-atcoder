use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [i64; n],
    };
    let mut count = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if l[i] == l[j] || l[i] == l[k] || l[j] == l[k] {
                    continue;
                }
                if l[i] + l[j] > l[k] && l[i] + l[k] > l[j] && l[j] + l[k] > l[i] {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
