use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n],
        txy: [(usize, usize, usize); q],
    };
    let mut offset = 0;
    for (t_i, x_i, y_i) in txy {
        match t_i {
            1 => {
                let x = (offset + x_i - 1) % n;
                let y = (offset + y_i - 1) % n;
                let t = a[x];
                a[x] = a[y];
                a[y] = t;
            }
            2 => offset = (offset + n - 1) % n,
            3 => println!("{}", a[(offset + x_i - 1) % n]),
            _ => unreachable!(),
        }
    }
}
