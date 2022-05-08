use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        r: Usize1,
        c: Usize1,
    };
    let mut count = 0;
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (r as i64 + dr, c as i64 + dc);
        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
            continue;
        }
        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}
