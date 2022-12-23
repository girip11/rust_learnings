fn main() {
    let _n = 5u8;
    let _large_number = 1_000;
    let _arr = [0; 5];
    println!("Hello world");

    statement_vs_expression();
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
