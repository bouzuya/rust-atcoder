use proconio::input;

fn main() {
    input! {
        k: i64,
        s: i64
    };

    let mut c = 0;
    for x in 0..=k {
        for y in 0..=k {
            let z = s - x - y;
            if (0..=k).contains(&z) {
                c += 1;
            }
        }
    }
    let ans = c;
    println!("{}", ans);
}
