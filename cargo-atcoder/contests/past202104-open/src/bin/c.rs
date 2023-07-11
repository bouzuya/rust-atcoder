use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut a = vec![];
    for _ in 0..n {
        input! {
            k_i: usize,
            a_i: [Usize1; k_i],
        }
        a.push(a_i);
    }
    input! {
        p: usize,
        q: usize,
        b: [Usize1; p],
    }

    let mut count = 0;
    for a_i in a {
        let mut c_i = vec![false; m];
        for a_ij in a_i {
            c_i[a_ij] = true;
        }
        if b.iter().filter(|&&b_i| c_i[b_i]).count() >= q {
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
