use std::vec;

fn sieve_of_atkin(limit: i32) {
    print!("[OUTPUT] ");
	// 2 and 3 are known to be prime
	if limit > 2 {
	    print!("{} ", 2);
	}

	if limit > 3 {
	    print!("{} ", 3);
	}

	// Initialise the sieve array with false values
  let mut sieve = vec!(0);
  for _e in 1..limit {
      sieve.push(0);
  }
    let l:usize = sieve.len() as usize;
	// Mark siev[n] is true if one of the following is true:
	// a) n = (4*x*x)+(y*y) has odd number of
	// solutions, i.e., there exist odd number of distinct pairs (x, y)
	//` that satisfy the equation and	n % 12 = 1 or n % 12 = 5.
	// b) n = (3*x*x)+(y*y) has odd number of
	// solutions and n % 12 = 7
	// c) n = (3*x*x)-(y*y) has odd number of
	// solutions, x > y and n % 12 = 11

	for x in 1..limit {
		for y in 1..limit {

			// Main part of Sieve of Atkin
			let mut n:usize = ((4 * x * x) + (y * y)) as usize;
			if n <= l && (n % 12 == 1 || n % 12 == 5) {
				sieve[n] ^= 1;
			}

			n = ((3*x*x)+(y*y)) as usize;
			if n <= l && n % 12 == 7 {
			    sieve[n] ^= 1;
            }

			n = ((3*x*x)-(y*y)) as usize;
			if x > y && n <= l && n % 12 == 11 {
				sieve[n] ^= 1;
			}
		}
	}

	// Mark all multiples of squares as non-prime
	for i in 0..sieve.len() {
	    let v = sieve[i];
	    if v == 1 {
		    let mut j = i*i;
		    while j <= l {
				sieve[j] = 0;
				j += i*i;
			}
		}
	}

	// Print primes using sieve[]
	for (i, v) in sieve.iter().enumerate() {
		if *v == 1 {
			print!("{} ", i);
		}
	}
}


// Driver program to test above functions */
fn main() {
  let i = 10;
  println!("[INPUT] {}", i);
  sieve_of_atkin(i);
}
