use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [Usize1; q]
    };
    let mut x_to_i = (0..n).collect::<Vec<usize>>();
    let mut a = (1..=n).collect::<Vec<usize>>();
    for x_i in x {
        let i = x_to_i[x_i];
        let i = if i == n - 1 { x_to_i[a[i - 1] - 1] } else { i };
        // println!("a[i] = {} a[i + 1] = {} {:?}", a[i], a[i + 1], a);
        a.swap(i, i + 1);
        x_to_i.swap(a[i] - 1, a[i + 1] - 1);
        // println!("{:?} {:?}", x_to_i, a);
    }
    for a_i in a {
        println!("{}", a_i);
    }
}
