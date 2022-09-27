use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let i = p.iter().position(|p_i| p_i == &1).unwrap();
    let asc = p[(i + 1) % n] == 2;

    let ans = if asc {
        i.min(1 + 1 + n - 1 - i + 1)
    } else {
        1 + (n - 1 - i).min(1 + i)
    };
    println!("{}", ans);
}
