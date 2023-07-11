use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        q: usize,
        b: [i64; q],
    };
    a.sort();

    let mut s = String::new();
    for b_i in b {
        let j = a.lower_bound_by_key(&b_i, |&a_i| a_i);
        let mut c = vec![];
        if j >= 1 {
            c.push(a[j - 1]);
        }
        if (0..n).contains(&j) {
            c.push(a[j]);
        }
        if (0..n).contains(&(j + 1)) {
            c.push(a[j + 1]);
        }
        let mut c = c.iter().map(|c_i| (c_i - b_i).abs()).collect::<Vec<i64>>();
        c.sort();
        s.push_str(&format!("{}\n", c[0]));
    }
    print!("{}", s);
}
