use rand::Rng;
use std::io;
fn main() {
    // let message=2;
    let primes=generate_two_primes();
    let modulous=get_product_of_the_two_primes(primes);
    let euler_phi_m=euler_phi(primes);
    let encryption_key=generate_encryption_key(euler_phi_m);
    let mut decryption_key=generate_decryption_key(encryption_key,euler_phi_m).expect("Failed to generate decryption key");
    while decryption_key == encryption_key {
        decryption_key = generate_decryption_key(encryption_key, euler_phi_m).expect("Failed to generate decryption key");
    }
    // Get the message from the user
    println!("Enter the message you want to send (as an integer):");
    let message = get_user_input().parse::<u32>().expect("Invalid input");
    let encrypt_message=encrypt_data(message,encryption_key,modulous);
    println!("The prime numbers are: {:?}",primes);
    println!("The product(m):modulo of the prime numbers is: {:?}",modulous);
    println!("The  φ(n) of the primes is: {:?}",euler_phi_m);
    println!("The encryption key is:{:?}",encryption_key);
    println!("The decryption key is:{:?}",decryption_key);
    println!("The message I want to send is:{message}");
    println!("The encrypted data is: {:?}",encrypt_message);
    println!("The decrypted message is:{:?}",decrypt_data(encrypt_message,decryption_key,modulous) );
    
}

// Function to read user input
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
// This function will call the generate_prime_number() function
// It will generate two primes p and q and return the as a tuple
fn generate_two_primes()->(u32,u32){
    let mut prime1;
    let mut prime2;

    loop {
        prime1 = generate_prime_number();
        prime2 = generate_prime_number();
        if prime1 != prime2 {
            return (prime1, prime2);
        }
    }
    
}
// This function gets the product of the two prime numbers generated
fn get_product_of_the_two_primes(primes:(u32,u32))->u32{
  primes.0*primes.1
}
// This function gets the euler phi of m which is the product of p*q
fn euler_phi(primes:(u32,u32))->u32{
    (primes.0-1)*(primes.1-1)
}
// This function will generate the encryption key such that
// 1 < e < euler_phi of (p-1)(q-1) &&
//  gcd (e,euler_phi)==1 meaning they are coprime
fn generate_encryption_key(euler_m:u32)->u32{
    let mut rng = rand::thread_rng();
    loop{
        let e=rng.gen_range(2..euler_m);
        if gcd(e,euler_m)==1{
            return e;
        }
    }
   
}
// This generates the decryption key
fn generate_decryption_key(encryption_key:u32,euler_phi_m:u32)->Option<u32>{
     mod_inverse(encryption_key,euler_phi_m)
}

fn encrypt_data(message:u32,encryption_key:u32,m:u32)->u32{
    let ciphered_message=fast_powering(message,encryption_key,m);
    ciphered_message
}

fn decrypt_data(ciphered_message:u32,decryption_key:u32,m:u32)->u32{
    let decrypt_message=fast_powering(ciphered_message,decryption_key,m);
    decrypt_message
}
// This functions generates random prime numbers within a certain range
fn generate_prime_number() -> u32 {
    let mut rng = rand::thread_rng();
    loop {
        let p = rng.gen_range(2..=30);
        if is_prime(p) {
            return p;
        }
    }
}


// This function will check if a number is prime
// Meaning that it cannot divide any other number except for 1 and itself
fn is_prime(n:u32)->bool{
    match n{
        0|1=>false,
        2=>true,
        _=>!(2..n).any(|i| n%i==0),
    }
}

fn gcd(mut a:u32,mut b:u32)->u32{
    while b !=0{
        let temp=b;
        b=a%b;
        a=temp;
    }
    a
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn mod_inverse(a: u32, m: u32) -> Option<u32> {
    let (g, x, _) = extended_gcd(a as i32, m as i32);
    if g != 1 {
        None
    } else {
        Some(((x % m as i32 + m as i32) % m as i32) as u32)
    }
}

fn fast_powering(a: u32, mut b: u32, modulo: u32) -> u32 {
    let mut res = 1;
    let mut base = a % modulo;
    while b > 0 {
        if b % 2 == 1 {
            res = (res * base) % modulo;
        }
        base = (base * base) % modulo;
        b /= 2;
    }
    res
}
