use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: [usize; 2 * n],
    };
    let mut ans = vec![];
    for i in 0..2 * n - 1 {
        if i % 2 == 0 {
            if p[i] < p[i + 1] {
                continue;
            } else if i + 2 < 2 * n && p[i] < p[i + 2] {
                p.swap(i + 1, i + 2);
                ans.push(i + 1);
            } else {
                p.swap(i, i + 1);
                ans.push(i);
            }
        } else {
            assert!(i % 2 != 0);
            if p[i] > p[i + 1] {
                continue;
            } else if i + 2 < 2 * n && p[i] > p[i + 2] {
                p.swap(i + 1, i + 2);
                ans.push(i + 1);
            } else {
                p.swap(i, i + 1);
                ans.push(i);
            }
        }
    }
    println!("{}", ans.len());
    for a in ans {
        println!("{}", a + 1);
    }
}
