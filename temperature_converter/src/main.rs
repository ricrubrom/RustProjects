fn main() {
    let temp=32.0;
    println!("The temperature in degrees celcius is {temp}");
    let temp=celcius_to_fahrenheit(temp);
    println!("The temperature in degrees fahrenheit is {temp}");
    let temp=fahrenheit_to_celcius(temp);
    println!("The temperature in degrees celcius is {temp}");
}

fn celcius_to_fahrenheit(temp:f32)->f32{
    temp*(9.0/5.0)+32.0
}

fn fahrenheit_to_celcius(temp:f32)->f32{
    (temp-32.0)/(9.0/5.0)
}
