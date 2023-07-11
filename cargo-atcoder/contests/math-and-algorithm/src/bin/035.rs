use proconio::input;

fn main() {
    input! {
        xyr: [(i64, i64, i64); 2]
    };
    let ((x1, y1, r1), (x2, y2, r2)) = if xyr[0].2 > xyr[1].2 {
        (xyr[0], xyr[1])
    } else {
        (xyr[1], xyr[0])
    };

    let dpow2 = (x1 - x2).pow(2) + (y1 - y2).pow(2);
    let ans = if dpow2 > (r1 + r2).pow(2) {
        5
    } else if dpow2 == (r1 + r2).pow(2) {
        4
    } else if dpow2 > (r1 - r2).pow(2) {
        3
    } else if dpow2 == (r1 - r2).pow(2) {
        2
    } else {
        1
    };
    println!("{}", ans);
}
