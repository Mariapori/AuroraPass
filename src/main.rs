use rand::Rng;
const PASSWORD_LENGHT: i32 = 12;
const CHARS: &str = "abcdefghijklmnopqrstuvwxyABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!#$";
fn main() {
    println!("***AuroraPass**************");
    print!("Your password: ");
    let mut rng = rand::thread_rng();
    let mut i = 0;
    for i in 0..PASSWORD_LENGHT {
        let mut rng = rand::thread_rng();
        let random = rng.gen_range(0..CHARS.len());
        let char = CHARS.chars().nth(random).unwrap();
        print!("{}", char);
    }
    println!("\n***************************");
}
