use proconio::input;

fn main() {
    input! {
        mut k: usize,
    };
    let mut ans = vec![];
    while k > 0 {
        ans.push(if (k & 1) != 0 { '2' } else { '0' });
        k >>= 1;
    }
    println!("{}", ans.into_iter().rev().collect::<String>());
}
