// primality
// naive primality checker

// this is a trait
// much like Java's interfaces
pub trait PrimeChecker {
    fn is_prime(self) -> bool;
}

// but we can implement those traits for almost anything
// except a foreign trait for foreign types
impl PrimeChecker for u64 {
    // since u64 is a primitive, we can refer to it as 'self' exactly
    // if it was a non-primitive, then we would refer to it as self, but then
    // to get the fields, we would do smth like self.value...easy :)
    fn is_prime(self) -> bool {
        for i in 2..self {
            if self % i == 0 {
                return false
            }
        }
        true
    }
}

fn display_primality() {
    for i in 0..1000 {
        let result = i.is_prime();
        println!("{} is {}", i, if result {"a prime"} else {"not a prime"})
    }
}

fn main() {
    display_primality();
}
