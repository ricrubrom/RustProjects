use std::io;

fn main() {
    println!("Please insert the number of the fibonacci sequence you desire:");
    let mut n=String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n=n.trim().parse().expect("Please insert a number");

    let num=fibonacci(n);
    println!("The number is {num}");
}

fn fibonacci(mut n:i32)->i32{
    let mut previous=0;
    let mut current=1;
    while n>0 {
        let aux=current;
        current=current+previous;
        previous=aux;
        n=n-1;
    }
    return current;
}
