use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }
    /*
        2進数に変換して、右から検索して、最初に1が出現するまでの0の個数
        10進数  2進数     求め方	    答え
        2       10	     1[0]	    1
        8	    1000	 1[000]	    3
        100	    1100100	 11001[00]  2
    */

    print!(
        "{}",
        a.into_iter()
            .map(|num: i32| num.trailing_zeros())
            .min()
            .unwrap()
    );
}
