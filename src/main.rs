use std::io;
fn main() {
    fibonacci_number();
    
}

fn fibonacci_number(){
    println!("What is the current temperature?");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    

    let n: i32 = match num.trim().parse(){
        Ok(p) => p,
        Err(_) => 0,
    };
    
    let mut fib = 1;
    let mut pre_fib_1 = 1;
    let mut pre_fib_2 = 1;

    for m in (1..n+1){
        if m > 2{
            fib = pre_fib_1 + pre_fib_2;
        }
        pre_fib_2 = pre_fib_1;
        pre_fib_1 = fib;
    }
    println!("{}",fib);
}