fn main() {
    let _n = 5u8;
    let _large_number = 1_000;
    let _arr = [0; 5];
    println!("Hello world");

    statement_vs_expression();
    iterate_using_for(10, false);
    println!("Decremented value {}", decrement(10, 5))
}

fn statement_vs_expression() -> () {
    // x is of type int32 here.
    let _x = {
        let y = 5;
        y + 10 // this is an expression
    };

    // x is of type unit
    let x = {
        let y = 5;
        y + 10; // this is a statement
    };
}

fn decrement(mut value: i32, n: u32) -> i32 {
    let mut counter = n;
    // let mut val = value;
    // try breaking with while loop
    while true {
        if counter == 0 {
            break;
        }
        value -= 1;
        counter -= 1;
    }
    value
}

fn iterate_using_for(end: u32, inclusive: bool) {
    let range: std::ops::RangeInclusive<u32> = if inclusive { 0..=end } else { 0..=(end - 1) };
    for value in range {
        println!("{value}")
    }
}
