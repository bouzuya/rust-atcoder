use proconio::input;

fn main() {
    input! {
        b: usize,
    };
    let mut used = vec![false; b];
    used[0] = true;
    used[1] = true;
    for start in 2..b {
        if used[start] {
            continue;
        }
        used[start] = true;

        let mut count = 0;
        let mut x = start;
        while x <= b {
            x += x / 2;
            if x < b {
                used[x] = true;
            }
            count += 1;
            if x == b {
                println!("{}", count);
                return;
            }
        }
    }
    println!("0");
}
