use err_handling::{init_logging_infrastructure,ResultExt};

fn main() {
    init_logging_infrastructure();
    println!("Hello, world!");
    let buggy : Result<i8, &str> = Err("examplery failure");
    buggy.expect_and_log("Not successful!");
}
