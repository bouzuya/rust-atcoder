use proconio::input;

fn main() {
    input! {
        n: usize,
        av: [usize; n]
    };
    let mut ok = true;
    for a in av {
        if a % 2 == 0 && (a % 3 != 0 && a % 5 != 0) {
            ok = false;
        }
    }
    let ans = if ok { "APPROVED" } else { "DENIED" };
    println!("{}", ans);
}
