use proconio::input;

fn dfs(ans: &mut i64, a: &[i64], b: &mut Vec<i64>) {
    if b.len() == a.len() {
        let mut y = 1_i64;
        for &b_i in b.iter() {
            y *= b_i;
        }
        if y % 2 == 0 {
            *ans += 1;
        }
        return;
    }
    let i = b.len();
    for &d in [-1, 0, 1].iter() {
        b.push(a[i] + d);
        dfs(ans, a, b);
        b.pop();
    }
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut ans = 0;
    dfs(&mut ans, &a, &mut vec![]);
    println!("{}", ans);
}
