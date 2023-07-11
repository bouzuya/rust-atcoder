use proconio::{input, marker::Chars};

fn main() {
    input! {
        abcd: Chars,
    };
    let a = (abcd[0] as u8 - b'0') as i64;
    let b = (abcd[1] as u8 - b'0') as i64;
    let c = (abcd[2] as u8 - b'0') as i64;
    let d = (abcd[3] as u8 - b'0') as i64;

    for op_1 in vec![1, -1] {
        for op_2 in vec![1, -1] {
            for op_3 in vec![1, -1] {
                if (((a + op_1 * b) + op_2 * c) + op_3 * d) == 7 {
                    println!(
                        "{}{}{}{}{}{}{}=7",
                        a,
                        if op_1 == 1 { '+' } else { '-' },
                        b,
                        if op_2 == 1 { '+' } else { '-' },
                        c,
                        if op_3 == 1 { '+' } else { '-' },
                        d
                    );
                    return;
                }
            }
        }
    }
}
