fn main() {
    println!("Hello, world!");
}

/*
    function_name: gcd
    parameters: {
        m: mutable unsigned 64bit integer,
        n: mutable unsigned 64bit integer,
        return: unsigned 64 bit integer
    }
    author: Pratik Raj Srivastava,
    date: 30/08/2024
*/

/// GCD function used for finding the greatest common divisor
fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m !=0 && n!=0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m%n;
    }
    n
}

/*
    unit_test_case_name: test_gcd
    author: Pratik Raj Srivastava,
    date: 30/08/2024,
    we are using [test] i.e. known as marker in Rust, similar to Annotation for Java
    which tell compiler that it is the unit test case, which needs to run when "cargo test" executes
*/
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}


