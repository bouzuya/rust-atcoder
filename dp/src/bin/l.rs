use proconio::input;

fn f(memo: &mut Vec<Vec<Option<(i64, i64)>>>, a: &[i64], l: usize, r: usize) -> (i64, i64) {
    if let Some((x, y)) = memo[l][r] {
        return (x, y);
    }

    let (x, y) = if l + 1 == r {
        let (x, y) = (a[l], a[r]);
        if x > y {
            (x, y)
        } else {
            (y, x)
        }
    } else {
        let (x1, y1) = f(memo, a, l + 1, r);
        let (x2, y2) = f(memo, a, l, r - 1);
        if a[l] + y1 - x1 > a[r] + y2 - x1 {
            (a[l] + y1, x1)
        } else {
            (a[r] + y2, x2)
        }
    };

    memo[l][r] = Some((x, y));
    (x, y)
}

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    let mut memo = vec![vec![None; n + 1]; n + 1];
    let (x, y) = f(&mut memo, &a, 0, n - 1);
    let ans = x - y;
    println!("{}", ans);
}
