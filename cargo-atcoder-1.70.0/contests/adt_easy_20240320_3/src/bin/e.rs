use proconio::input;

fn main() {
    input! {
        mut k: usize,
    };
    let mut ans = vec![];
    while k != 0 {
        ans.push(if k % 2 == 0 { '0' } else { '2' });
        k /= 2;
    }
    println!("{}", ans.into_iter().rev().collect::<String>());
}
