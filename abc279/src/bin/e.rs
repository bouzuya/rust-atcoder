use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    };

    let mut pos1 = vec![0];
    for a_i in a.iter().copied().take(m - 1) {
        let x = *pos1.last().unwrap();
        let next = if x == a_i {
            a_i + 1
        } else if x == a_i + 1 {
            a_i
        } else {
            x
        };
        pos1.push(next);
    }

    let mut ans = vec![0_usize; m];
    let mut s = (0..n).collect::<Vec<usize>>();
    for (i, a_i) in a.into_iter().enumerate().rev() {
        ans[i] = s[pos1[i]];
        s.swap(a_i, a_i + 1);
    }

    for a in ans {
        println!("{}", a + 1);
    }
}
