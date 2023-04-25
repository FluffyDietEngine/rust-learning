use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    let random_number :u32 = rand::thread_rng().gen_range(1..=10);

    println!("The random number generated is -> {random_number}");

    loop{
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read!");
        
        println!("Your guess is -> {guess}");

        // let guess: u32 = guess.trim().parse().expect("Not an integer");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Improper data type, enter an integer");
                continue;
            }
        };

        match guess.cmp(&random_number){
            Ordering::Less => println!("Guessed less"),
            Ordering::Greater => println!("Guessed higher"),
            Ordering::Equal => {
                println!("Correct guess!");
                break;
            }
        };
    };

}