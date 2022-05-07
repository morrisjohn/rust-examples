pub fn name() {
    use std::io;

    println!("Please enter your name");
    let mut user = String::new();

    io::stdin()
        .read_line(&mut user)
        .expect("Please enter a valid name");

        println!("Hello Mr {} 🏋️🏋️🏋️🦸🏋️", user);
}
