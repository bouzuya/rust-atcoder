use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; 2_usize.pow(n as u32)],
    };
    let n2 = 2_usize.pow(n as u32);
    let v0 = *a[0..n2 / 2].iter().max().unwrap();
    let v1 = *a[n2 / 2..n2].iter().max().unwrap();
    let v = v0.min(v1);
    let ans = a.iter().copied().position(|a_i| a_i == v).unwrap() + 1;
    println!("{}", ans);
}
