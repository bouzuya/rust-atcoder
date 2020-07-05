use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort_by_key(|a_i| -a_i);

    let mut ans = 0;
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(a[0]);
    for i in 1..n {
        let a_i = a[i];
        ans += pq.pop().unwrap();
        pq.push(a_i);
        pq.push(a_i);
    }
    println!("{}", ans);
}
