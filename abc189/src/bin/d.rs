use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let ao = s.iter().map(|s_i| s_i[0] == 'A').collect::<Vec<bool>>();
    let mut tf = (1_u64, 1_u64);
    for is_and in ao {
        let next = if is_and {
            (tf.0, tf.1 * 2 + tf.0)
        } else {
            (tf.0 * 2 + tf.1, tf.1)
        };
        tf = next;
    }
    let ans = tf.0;
    println!("{}", ans);
}
