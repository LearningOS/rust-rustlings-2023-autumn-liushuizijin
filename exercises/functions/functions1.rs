// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


fn call_me(name: String) {
    println!("call me: {}.", name);
}

fn main() {
    let name: String = "liushuizijin".to_string();
    call_me(name);
}
