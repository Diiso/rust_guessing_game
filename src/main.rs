//Rust doesn't provide a random librairy yet, but we can use
// the rand crate provided by the Rust team
extern crate rand;
//The io library is the input/output librairy necessary to handle those
use std::io;
use std::cmp::Ordering;
//use std::io is similar to use namespace std in C++
use rand::Rng;

fn main() {
    println!("Guess the number, which is between 1 and 100.");
    //This variable is immutable and we want it to be, obviously
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("The secret number is {}", secret_number);
    println!("Please input your guess");
    loop {
        //In Rust, variables are immutable by default.
        //To be able to make them mutable, use mut before variable name
        //Reminder : an immutable object state cannot be changed after its creation

        //Here we want guess to be mutable since we'll edit its value...
        let mut guess = String::new();
        //The String::new() is just like javascript new Object syntax
        //It calls the object, here a string object, constructor
        // ---> In the end, we have just created a mutable variable bound
        // to a new and empty instance of a string object.


        //If we hadn't use std::io at the beginning, we would have had
        // to type std::io::stdin
        //The stdin function returns an object that handles inputs.
        //Therefore the .read_line calls the method read_line from this object
        //See chapter 4 for the reference &
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            //Allows us to handle exceptions

        //trim allows to get rid of white space at the beginning and at the end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Using a match instead of a .expect is how you go from
        // crashing the program to actually handle the error.
        // let guess: u32 = guess.trim().parse()
        //     .expect("Type a number.")

        // IN GENERAL : match matches results from the function
        //  called to the different options you give it in the arms of
        // answers

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small. Try Again !"),
            Ordering::Greater => println!("Too big. Try Again !"),
            Ordering::Equal => {
                println!("You win !");
                break;
            }
        }
        //read_line and cmp are both enums (enumerations)
    }

    // let x = 5;
    // let y = 10;
    // println!("x = {} and y = {}", x, y);
}
