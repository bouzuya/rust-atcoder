use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut x_0 = 0;
    let mut s = true;
    for &a_i in a.iter() {
        x_0 += if s { 1 } else { -1 } * a_i;
        s = !s;
    }

    print!("{} ", x_0);
    let mut p = x_0;
    for (i, &a_i) in a.iter().enumerate().take(n - 1) {
        let x_i = (a_i - p / 2) * 2;
        print!("{}{}", x_i, if i == n - 1 { "\n" } else { " " });
        p = x_i;
    }
}
