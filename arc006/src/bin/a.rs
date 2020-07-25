use proconio::input;

fn main() {
    input! {
        e: [i64; 6],
        b: i64,
        l: [i64; 6],
    };
    let mut count = 0;
    for &e_i in e.iter() {
        if l.contains(&e_i) {
            count += 1;
        }
    }
    let ans = match (count, l.contains(&b)) {
        (6, _) => 1,
        (5, true) => 2,
        (5, false) => 3,
        (4, _) => 4,
        (3, _) => 5,
        _ => 0,
    };
    println!("{}", ans);
}
