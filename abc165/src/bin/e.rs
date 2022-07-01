use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    };
    let mut ans = vec![];
    let mo = m / 2;
    let me = m - mo;
    for i in 0..me {
        ans.push((i, 2 * me - 1 - i));
    }
    let offset = 2 * me;
    for i in 0..mo {
        ans.push((offset + i, offset + 2 * mo + 1 - 1 - i));
    }
    for (a, b) in ans {
        println!("{} {}", a + 1, b + 1);
    }
}
