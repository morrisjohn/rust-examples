pub fn temp_conv() {
    use std::io;

    let mut fa_or_cel = String::new();

    println!("Press 1 to convert C to F. \nPress 2 to convert F to C");

    io::stdin()
        .read_line(&mut fa_or_cel)
        .expect("Please enter a valid number");

    let fa_or_cel: u64 = fa_or_cel.trim().parse().expect("lol");

    let temp = fa_or_cel;

    if temp == 1 {
        farenheit(1.8);
    } else if temp == 2 {
        celsius(0.56);
    } else {
        println!("Sorry you didn't choose any of the above options. Retry");
    }
}

fn farenheit(fa: f32) {
    use std::io;

    let mut cel = String::new();

    println!("Enter a tempearture in celsuis");

    io::stdin()
        .read_line(&mut cel)
        .expect("Enter a valid number");

    let cel: f32 = cel.trim().parse().expect("Enter a valid number");
    let value = cel * fa + 32.0;
    println!("{} degree celsius = {} degree farenheit", cel, value);
}

fn celsius(c: f32) {
    use std::io;

    let mut fa = String::new();

    println!("Enter a tempearture in Farenheit");

    io::stdin()
        .read_line(&mut fa)
        .expect("Enter a valid number");

    let fa: f32 = fa.trim().parse().expect("Enter a valid number");
    let value = fa * c + (-17.78);
    println!("{} degree farenheit = {} degree celsius", fa, value);
}
