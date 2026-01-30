mod algo;
use algo::Train;

mod or_test;
use or_test::or_test;

mod chrono_test;
use chrono_test::chrono_test;

fn main() {
    let train = Train::new("moscow", 1_000_000, -0b0111_1111);
    train.hello();
    or_test();
    chrono_test();
}
