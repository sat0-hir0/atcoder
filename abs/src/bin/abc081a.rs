use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: i32
    }

    let mut count = 0;

    let input_s: i32 = s;
    for num in input_s.to_string().chars() {
        if num == '1' {
            count += 1;
        }
    }
    print!("{}", count);
}
