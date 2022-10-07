use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let mut count = 0_usize;
    let mut r = 0;
    for l in 0..n {
        while r < n && c[r + 1] - c[l] <= k {
            r += 1;
        }
        count += r - l;
        if l == r {
            r += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
