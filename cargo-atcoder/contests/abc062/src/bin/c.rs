use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let f = |r: usize, c: usize| -> usize {
        let mut min = r * c;
        for i in 1..c {
            let a1 = r * i;
            let a2 = r / 2 * (c - i);
            let a3 = r * c - a1 - a2;
            if a1 == 0 || a2 == 0 || a3 == 0 {
                continue;
            }
            min = min.min(a1.max(a2).max(a3) - a1.min(a2).min(a3));
        }
        min
    };
    let g = |r: usize, c: usize| -> usize {
        let mut min = r * c;
        for i in 1..=c {
            let a1 = r * i;
            let a2 = (c - i) / 2 * r;
            let a3 = ((c - i) - ((c - i) / 2)) * r;
            if a1 == 0 || a2 == 0 || a3 == 0 {
                continue;
            }
            min = min.min(a1.max(a2).max(a3) - a1.min(a2).min(a3));
        }
        min
    };
    let ans = f(h, w).min(f(w, h)).min(g(h, w)).min(g(w, h));
    println!("{}", ans);
}
