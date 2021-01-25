use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 1 << n],
    };
    let mut p = (0..1 << n).collect::<Vec<usize>>();
    for i in 1..n {
        let mut next = vec![];
        for j in 1..=1 << (n - i) {
            let x = if a[p[2 * j - 2]] < a[p[2 * j - 1]] {
                p[2 * j - 1]
            } else {
                p[2 * j - 2]
            };
            next.push(x);
        }
        p = next;
    }
    let ans = if a[p[0]] < a[p[1]] { p[0] } else { p[1] };
    println!("{}", ans + 1);
}
