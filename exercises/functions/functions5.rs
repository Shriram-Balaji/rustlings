// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)


fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

// expressions always return
// when there's a semicolon its a statement, and doesnt return anything
fn square(num: i32) -> i32 {
    num * num
}
