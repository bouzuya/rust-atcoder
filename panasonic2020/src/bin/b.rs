use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    if h == 1 || w == 1 {
        println!("1");
        return;
    }

    let ans = match (h % 2, w % 2) {
        (0, 0) | (0, 1) | (1, 0) => (h * w) / 2,
        (1, 1) => (h * w) / 2 + 1,
        _ => unreachable!(),
    };
    println!("{}", ans);
}
