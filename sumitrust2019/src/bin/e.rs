use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut xyz = vec![vec![0, 0, 0]];
    for (i, &a_i) in a.iter().enumerate() {
        let mut xyz_next = xyz[i].clone();
        xyz_next.sort_by_key(|&v| -v);
        for v in xyz_next.iter_mut() {
            if *v == a_i {
                *v += 1;
                break;
            }
        }
        xyz.push(xyz_next);
    }
    let mut count = 1_usize;
    for (&a_i, xyz_i) in a.iter().zip(xyz.iter()) {
        count *= xyz_i.iter().filter(|&&v| v == a_i).count();
        count %= 1_000_000_007;
    }
    let ans = count;
    println!("{}", ans);
}
