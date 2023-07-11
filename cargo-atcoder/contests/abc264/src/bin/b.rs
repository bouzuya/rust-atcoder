use proconio::{input, marker::Usize1};

fn main() {
    input! {
        r: Usize1,
        c: Usize1,
    };
    let board = vec![
        "###############",
        "#             #",
        "# ########### #",
        "# #         # #",
        "# # ####### # #",
        "# # #     # # #",
        "# # # ### # # #",
        "# # # # # # # #",
        "# # # ### # # #",
        "# # #     # # #",
        "# # ####### # #",
        "# #         # #",
        "# ########### #",
        "#             #",
        "###############",
    ];
    let ans = board[r].chars().collect::<Vec<char>>()[c] == '#';
    println!("{}", if ans { "black" } else { "white" });
}
