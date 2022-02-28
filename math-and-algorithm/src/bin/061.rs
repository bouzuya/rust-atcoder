use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    //  1: false 0
    //  2: true  1
    //  3: false 1
    //  4: true  2
    //  5: true  2
    //  6: true  3
    //  7: false 3
    //  8: true  4
    //  9: true  4
    // 10: true  5
    // 11: true  5
    // 12: true  6
    // 13: true  6
    // 14: true  7
    // 15: false 7
    // 16: true  8
    // 17: true  8
    // 18: true  9
    // 19: true  9
    // 20: true  10
    // 21: true  10
    // 22: true  11
    // 23: true  11
    // 24: true  12
    // 25: true  12
    // 26: true  13
    // 27: true  13
    // 28: true  14
    // 29: true  14
    // 30: true  15
    // 31: false 15
    // 1, 3, 7, 15, 31
    //  +2 +4 +8 +16
    let mut sum = 0;
    for r in 0.. {
        let s = 2_usize.pow(r);
        if n == sum + s {
            println!("Second");
            break;
        } else if n < sum + s {
            println!("First");
            break;
        } else {
            sum += s;
        }
    }
}
