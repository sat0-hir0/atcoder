use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    print!(
        "{}",
        a.into_iter()
            .map(|num: i32| num.trailing_zeros())
            .min()
            .unwrap()
    );
}
