use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    for (a_i0, a_i1) in a.windows(2).map(|w| match w {
        &[a_i0, a_i1] => (a_i0, a_i1),
        _ => unreachable!(),
    }) {
        if a_i1 == a_i0 {
            println!("stay");
        } else if a_i1 < a_i0 {
            println!("down {}", a_i0 - a_i1);
        } else if a_i1 > a_i0 {
            println!("up {}", a_i1 - a_i0);
        } else {
            unreachable!();
        }
    }
}
