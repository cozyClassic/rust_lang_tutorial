use std::io; // 라이브러리 import

/* 모든 프로그램의 스코프에 가져오는, 스표준 라이브러리의 Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude
* 
*/
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String = String::new(); 

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
