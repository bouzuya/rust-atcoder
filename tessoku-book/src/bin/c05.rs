use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let n = n - 1;
    let mut ans = vec!['4'; 10];
    for i in 0..10 {
        if ((n >> i) & 1) == 1 {
            ans[10 - 1 - i] = '7';
        }
    }
    let ans = ans.iter().collect::<String>();
    println!("{}", ans);
}
