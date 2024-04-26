fn foo() {
    println!("foo")
}

fn main() {
    stupid_macros::repeat!(foo(), 3);
    stupid_macros::repeat!({ println!("hello world") }, 2);
}
