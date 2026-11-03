fn main() {
    let mut string = String::from("Hello");

    let len = calculate_length(&string);

    println!("len: {len}");

    change(&mut string);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what it refers to, the String is not dropped.

// fn change(some_string: &String) { this function doesn't work because reference is imutable
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
