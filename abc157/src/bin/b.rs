use proconio::input;

fn main() {
    input! {
        avv: [[usize; 3]; 3],
        n: usize,
        bv: [usize; n]
    };
    let mut ans = false;
    for i in 0..3 {
        ans |= (0..3).all(|j| bv.contains(&avv[i][j]));
        ans |= (0..3).all(|j| bv.contains(&avv[j][i]));
    }
    ans |= (0..3).all(|i| bv.contains(&avv[i][i]));
    ans |= (0..3).all(|i| bv.contains(&avv[3 - (i + 1)][i]));
    println!("{}", if ans { "Yes" } else { "No" });
}
