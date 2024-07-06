// Here are some more easy Clippy fixes so you can see its utility 📎
// TODO: Fix all the Clippy lints.
use std::mem::swap;

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        println!("{:?}", my_option);
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {my_arr:?}");

    vec![1, 2, 3, 4, 5].resize(0, 3);
    println!("This Vec is empty, see? {:?}",());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
