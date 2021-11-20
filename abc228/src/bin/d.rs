use proconio::input;

fn leader(n: usize, next: &mut [usize], x: usize) -> usize {
    if next[x] == x {
        return x;
    }

    next[x] = leader(n, next, next[x]);
    next[x]
}

fn merge(n: usize, next: &mut [usize], a: usize, b: usize) {
    let (x, y) = (leader(n, next, a), leader(n, next, b));
    if x == y {
        return;
    }

    next[x] = y;
}

fn main() {
    input! {
        q: usize,
        tx: [(usize, usize); q],
    };
    let n = 1 << 20;

    let mut a = vec![-1_i64; n];
    let mut next = (0..n).collect::<Vec<usize>>();
    for (t, x) in tx {
        match t {
            1 => {
                let i = leader(n, &mut next, x % n);
                merge(n, &mut next, i, (i + 1) % n);
                next[i] = next[(i + 1) % n];
                a[i] = x as i64;
            }
            2 => {
                println!("{}", a[x % n]);
            }
            _ => unreachable!(),
        }
    }
}
