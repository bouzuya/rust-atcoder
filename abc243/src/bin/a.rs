use proconio::input;

fn main() {
    input! {
        mut v: i64,
        abc: [i64; 3],
    };
    let mut i = 0;
    loop {
        v -= abc[i];
        if v < 0 {
            break;
        }
        i += 1;
        i %= 3;
    }
    let ans = vec!["F", "M", "T"][i];
    println!("{}", ans);
}
