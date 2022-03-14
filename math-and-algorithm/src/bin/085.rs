use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };

    let mut ds = vec![];
    for i in 1..=n {
        if i * i > y {
            break;
        }
        if y % i == 0 {
            ds.push(i);
            if i != y / i && y / i <= n {
                ds.push(y / i);
            }
        }
    }
    ds.sort();

    for i in 0..ds.len() {
        for j in i..ds.len() {
            for k in j..ds.len() {
                for l in k..ds.len() {
                    let (a, b, c, d) = (ds[i], ds[j], ds[k], ds[l]);
                    if (a + b + c + d == x) && (a * b * c * d == y) {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }
    println!("No");
}
