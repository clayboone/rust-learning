fn main() {
    let sum_primes_under: u64 = 20_000_000;

    println!(
        "Sum of all primes below {} is {}",
        sum_primes_under,
        prime_summer(sum_primes_under)
    );
}

fn prime_summer(n: u64) -> u64 {
    let mut sum = 0_u64;

    for i in 0..n as u64 {
        if is_prime(i) {
            sum += i;
        }
    }

    sum
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    if n == 2 {
        return true;
    }

    for i in 2..=(n as f64).sqrt() as u64 + 1 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
