




// fn main() {
//     println!("Guess the number game.");

//     let secret = rand::thread_rng().gen_range(1..=100);
//     // In Rust, variables are immutable by default,
//     // meaning once we give the variable a value, the value won't change.
//     loop {
//         // For now, all you need to know is that like variables,
//         // references are immutable by default. Hence, you
//         // need to write &mut guess rather than &guess to make it mutable.
//         let mut guess = String::new();

//         println!("Enter your guess:");

//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read the input.");

//         let guessed: i32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(msg) => {
//                 println!("{msg}");
//                 continue;
//             }
//         };
//         println!("You guessed: {guessed}");

//         match guessed.cmp(&secret) {
//             Ordering::Equal => {
//                 println!("You guessed it right!!!!");
//                 break;
//             }
//             Ordering::Less => println!("Too small"),
//             Ordering::Greater => println!("Too big"),
//         }
//     }
//     // println!("You guessed {}, {}", guess, guess);
// }

fn main() {
    println!("Hello world!");
}
