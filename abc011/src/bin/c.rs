use proconio::input;

fn main() {
    input! {
        mut n: i64,
        ng: [i64; 3],
    };
    if ng.iter().any(|&ng_i| ng_i == n) {
        println!("NO");
        return;
    }
    for _ in 0..100 {
        let mut ok = false;
        for x in (1..=3).rev() {
            if ng.iter().any(|&ng_i| ng_i == n - x) {
                continue;
            }
            ok = true;
            n -= x;
            break;
        }
        if !ok {
            println!("NO");
            return;
        }
    }
    println!("{}", if n <= 0 { "YES" } else { "NO" });
}
