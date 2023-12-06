// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM DONE

// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` ordc use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Remove the unnecessary unwrap()
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new(); // Create an empty vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;

    // Let's swap these two!
    std::mem::swap(&mut value_a, &mut value_b); // Added a semicolon here
    println!("value a: {}; value b: {}", value_a, value_b);
}


