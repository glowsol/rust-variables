fn main() {
    /*let _guess: u32 = "42".parse().expect("Not a number!");
    println!("{}",_guess);*/

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
