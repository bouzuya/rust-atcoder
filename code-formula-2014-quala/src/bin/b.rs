use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        p: [usize; a],
        q: [usize; b],
    };
    let mut pin = vec!['x'; 10];
    for p_i in p {
        pin[(p_i + 10 - 1) % 10] = '.';
    }
    for q_i in q {
        pin[(q_i + 10 - 1) % 10] = 'o';
    }

    println!("{} {} {} {}", pin[6], pin[7], pin[8], pin[9]);
    println!(" {} {} {}", pin[3], pin[4], pin[5]);
    println!("  {} {}", pin[1], pin[2]);
    println!("   {}", pin[0]);
}
