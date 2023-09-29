use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        _m: usize,
        s: Chars,
    };

    let mut player = 0_usize;
    let mut count = vec![0_usize; n];
    let mut field = 0_usize;
    for s_i in s {
        count[player] += 1;
        match s_i {
            '+' => {
                count[player] += field;
                field = 0;
            }
            '0' => {
                // do nothing
            }
            '-' => {
                field += count[player];
                count[player] = 0;
            }
            _ => unreachable!(),
        }
        player = (player + 1) % n;
    }

    for c in count {
        println!("{}", c);
    }
}
