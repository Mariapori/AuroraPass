use std::env;
use rand::Rng;
const CHARS: &str = "abcdefghijklmnopqrstuvwxyABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!#$";
const PASSWORD_LENGHT: i32 = 12;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let lenght: i32 = args[1].parse::<i32>().unwrap_or_default();
        if lenght == 0 {
            println!("Please input a number.");
            return;
        }
        let mut rng = rand::thread_rng();
        for _i in 0..lenght {
            let random = rng.gen_range(0..CHARS.len());
            let char = CHARS.chars().nth(random).unwrap();
            print!("{}", char);
        }
        print!("\n");
        return;
    }
    println!("***aurorapass**************");
    print!("Your password: ");
    let _rng = rand::thread_rng();
    for _i in 0..PASSWORD_LENGHT {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..CHARS.len());
        let char = CHARS.chars().nth(random).unwrap();
        print!("{}", char);
    }
    println!("\n***************************");
}
