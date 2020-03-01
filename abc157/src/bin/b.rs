use proconio::input;

fn main() {
    input! {
        avv: [[usize; 3]; 3],
        n: usize,
        bv: [usize; n]
    };
    let mut cvv = vec![vec![false; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for &b in bv.iter() {
                if avv[i][j] == b {
                    cvv[i][j] = true;
                }
            }
        }
    }
    let ans = (cvv[0][0] && cvv[0][1] && cvv[0][2])
        || (cvv[1][0] && cvv[1][1] && cvv[1][2])
        || (cvv[2][0] && cvv[2][1] && cvv[2][2])
        || (cvv[0][0] && cvv[1][0] && cvv[2][0])
        || (cvv[0][1] && cvv[1][1] && cvv[2][1])
        || (cvv[0][2] && cvv[1][2] && cvv[2][2])
        || (cvv[0][0] && cvv[1][1] && cvv[2][2])
        || (cvv[2][0] && cvv[1][1] && cvv[2][0]);
    println!("{}", if ans { "Yes" } else { "No" });
}
