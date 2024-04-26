use std::time::Duration;

fn foo() -> u8 {
    std::thread::sleep(Duration::from_secs(1));
    1 + 2
}

fn main() {
    stupid_macros::timed!(sleep, foo());
    stupid_macros::timed!({ "macro" });
}
