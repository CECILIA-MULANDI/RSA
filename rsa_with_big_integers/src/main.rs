use num_bigint::{BigUint, RandBigInt, BigInt, ToBigInt};
use num_traits::{One, Zero, ToPrimitive, Signed};
use num_integer::Integer;
use rand::Rng;
use std::io;

fn main() {
    println!("Enter the message you want to send (as an integer):");
    let message = get_user_input().parse::<u32>().expect("Invalid input");

    println!("Received message: {}", message);

    let primes = generate_two_primes();
    println!("Generated primes: {:?}", primes);

    let modulus = get_product_of_the_two_primes(&primes);
    let euler_phi_m = euler_phi(&primes);
    let encryption_key = generate_encryption_key(&euler_phi_m);
    let decryption_key = generate_decryption_key(&encryption_key, &euler_phi_m).expect("Failed to generate decryption key");

    let encrypt_message = encrypt_data(message, &encryption_key, &modulus);
    println!("Encrypted message: {:?}", encrypt_message);

    let decrypt_message = decrypt_data(&encrypt_message, &decryption_key, &modulus);
    println!("Decrypted message: {:?}", decrypt_message);

    println!("The prime numbers are: {:?}", primes);
    println!("The product (n) of the prime numbers is: {:?}", modulus);
    println!("The φ(n) of the primes is: {:?}", euler_phi_m);
    println!("The encryption key is: {:?}", encryption_key);
    println!("The decryption key is: {:?}", decryption_key);
    println!("The message I want to send is: {}", message);
    println!("The encrypted data is: {:?}", encrypt_message);
    println!("The decrypted message is: {:?}", decrypt_message);
}


fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn generate_two_primes() -> (BigUint, BigUint) {
    let mut rng = rand::thread_rng();
    let prime1 = generate_prime_number(&mut rng, 1024);
    let prime2 = generate_prime_number(&mut rng, 1024);
    (prime1, prime2)
}

fn get_product_of_the_two_primes(primes: &(BigUint, BigUint)) -> BigUint {
    &primes.0 * &primes.1
}

fn euler_phi(primes: &(BigUint, BigUint)) -> BigUint {
    let phi1 = &primes.0 - BigUint::one();
    let phi2 = &primes.1 - BigUint::one();
    phi1 * phi2
}

fn generate_encryption_key(euler_phi_m: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();
    loop {
        let e = rng.gen_biguint_below(euler_phi_m);
        if e > BigUint::one() && gcd(&e, euler_phi_m) == BigUint::one() {
            return e;
        }
    }
}

fn generate_decryption_key(encryption_key: &BigUint, euler_phi_m: &BigUint) -> Option<BigUint> {
    mod_inverse(encryption_key.to_bigint().unwrap(), euler_phi_m.to_bigint().unwrap())
        .map(|x| x.to_biguint().unwrap())
}

fn encrypt_data(message: u32, encryption_key: &BigUint, n: &BigUint) -> BigUint {
    BigUint::from(message).modpow(encryption_key, n)
}

fn decrypt_data(ciphered_message: &BigUint, decryption_key: &BigUint, n: &BigUint) -> u32 {
    let decrypted = ciphered_message.modpow(decryption_key, n);
    decrypted.to_u32().expect("Failed to convert decrypted message to u32")
}

fn generate_prime_number(rng: &mut impl Rng, bits: u64) -> BigUint {
    loop {
        let n = rng.gen_biguint(bits);
        if n.is_odd() && is_prime(&n) {
            return n;
        }
    }
}

fn is_prime(n: &BigUint) -> bool {
    if n <= &BigUint::one() {
        return false;
    }
    if n == &BigUint::from(2u32) {
        return true;
    }
    if n.is_even() {
        return false;
    }
    miller_rabin(n, 20)
}

fn miller_rabin(n: &BigUint, k: u32) -> bool {
    if n <= &BigUint::from(3u32) {
        return n > &BigUint::one();
    }

    let mut d = n - BigUint::one();
    let mut s = 0;
    while d.is_even() {
        d >>= 1;
        s += 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &(n - BigUint::from(2u32)));
        let mut x = a.modpow(&d, n);

        if x == BigUint::one() || x == n - BigUint::one() {
            continue;
        }

        let mut is_composite = true;
        for _ in 0..s-1 {
            x = x.modpow(&BigUint::from(2u32), n);
            if x == n - BigUint::one() {
                is_composite = false;
                break;
            }
        }

        if is_composite {
            return false;
        }
    }

    true
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

fn mod_inverse(a: BigInt, m: BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(&a, &m);
    if g != BigInt::one() {
        None
    } else {
        Some((x % &m + &m) % &m)
    }
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a.clone(), BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = extended_gcd(b, &(a % b));
        (g, y.clone(), x - (a / b) * y)
    }
}
