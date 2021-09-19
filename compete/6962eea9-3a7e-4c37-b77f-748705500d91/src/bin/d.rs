use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut x = vec![0; n];
    for i in (0..n).rev() {
        let mut r = 0;
        for j in (i + i + 2 - 1..n).step_by(i + 1) {
            r += x[j];
            r %= 2;
        }
        if a[i] != r {
            x[i] = 1;
        }
    }

    for i in 0..n {
        let mut r = 0;
        for j in (i..n).step_by(i + 1) {
            r += x[j];
            r %= 2;
        }
        if r != a[i] {
            println!("-1");
            return;
        }
    }

    let (m, b) = {
        let b = x
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, x_i)| x_i > 0)
            .map(|(i, _)| i + 1)
            .collect::<Vec<usize>>();
        let m = b.len();
        (m, b)
    };

    println!("{}", m);
    for (i, b_i) in b.iter().copied().enumerate() {
        print!("{}{}", b_i, if i == m - 1 { "\n" } else { " " });
    }
}
