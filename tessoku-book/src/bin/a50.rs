use proconio::input;
use rand::{thread_rng, Rng};

// 4625995259
fn f1(n: usize, _a: &[Vec<usize>]) -> Vec<(usize, usize, usize)> {
    let mut ans = vec![];
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let i = rng.gen::<usize>() % n;
        let j = rng.gen::<usize>() % n;
        ans.push((i, j, 1));
    }
    ans
}

fn main() {
    let n = 100_usize;
    input! {
        a: [[usize; n]; n],
    };
    let ans = f1(n, &a);
    println!("{}", ans.len());
    for (x, y, h) in ans {
        println!("{} {} {}", x, y, h);
    }
}
