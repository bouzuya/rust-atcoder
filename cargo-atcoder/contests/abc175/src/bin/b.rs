use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    };
    l.sort();
    let mut count = 0_usize;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if l[i] == l[j] || l[i] == l[k] || l[j] == l[k] {
                    continue;
                }

                if l[i] + l[j] > l[k] {
                    count += 1;
                }
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
