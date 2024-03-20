pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut n = n;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            // Check for potential overflow before multiplication
            if let Some(new_n) = n.checked_mul(3).and_then(|x| x.checked_add(1)) {
                n = new_n;
            } else {
                return None; // Overflow occurred
            }
        }
        count += 1;
    }
    
    Some(count)
