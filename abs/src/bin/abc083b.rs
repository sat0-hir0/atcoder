use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    };

    let result = (1..n + 1)
        .filter(|num| {
            let sum = num
                .to_string()
                .chars()
                .map(|c| (c as u8 - b'0') as u32)
                .sum::<u32>();
            a <= sum && sum <= b
        })
        .sum::<u32>();
    print!("{}", result);
}
