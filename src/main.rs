use std::u64;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            m = m % n;
        }
    n
}

fn soma(mut x: u64, mut y: u64) -> u64 {
    assert!(x != 0 && y != 0);
    let c = x + y;
    c
}

fn main(){
    println!("{}", gcd(14, 15));
    println!("{}", soma(7, 3));

}


#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
        3 * 7 * 11 * 13 * 19),
        3 * 11);
}