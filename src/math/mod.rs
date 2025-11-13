pub mod matrix;

pub fn is_prime(n: isize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn gcd(mut a: isize, mut b: isize) -> isize {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

pub fn lcm(a: isize, b: isize) -> isize {
    (a / gcd(a, b)) * b
}

pub fn factorial(n: isize) -> isize {
    assert!(n >= 0, "factorial only defined for n >= 0");
    (1..=n).product()
}

pub fn n_choose_k(n: isize, k: isize) -> isize {
    if k < 0 || k > n {
        return 0;
    }
    (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i)
}

pub fn pow(mut base: isize, mut exp: isize) -> isize {
    assert!(exp >= 0, "negative exponent not supported");
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    result
}
