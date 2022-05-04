use std::io::{self, Write};
use rand::Rng;

fn main() {
    // Make an array with three random numbers with a minimum of 1 and a maximum of 100
    let mut wugs: [isize; 3] = [
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101)
    ];
    
    // Tries
    let mut n: i8 = 0;

    loop {
        n += 1;

        let left = read_boundary("Left");
        let right = read_boundary("Right");

        if left < 1 || left > 100 || right < 1 || right > 100 {
            println!("Boundaries must be integers greater than 0 and less than 101\n");
            continue;
        }

        if left > right {
            println!("Left boundary must be less than right boundary\n");
            continue;
        }

        // enclosed = number of wugs that fall within the interval given by the left and right boundaries
        let mut enclosed: i8 = 0;

        // loop over all three wugs
        for wug in &mut wugs {
            /*
             * (wug - left)(wug - right)
             * = 0 if wug = left or wug = right
             * < 0 if left < wug < right
             * > 0 if wug < left or wug > right
             * 
             * wug is a reference, therefore dereference by using *wug
             */
            let j = (*wug - left) * (*wug - right);

            // j = 0 --> one of the boundaries is where the wug is
            if j == 0 {       
                println!("Found wug on {} after {} tries!", wug, n);

                // Set value of wug (by dereferencing) to 0 to indicate it has been caught
                *wug = 0;
            }
            
            // J < 0 --> wug is between the left and right boundaries
            if j < 0 {
                enclosed += 1;
            }
        }

        // If all wugs are found, break out of the loop
        if wugs[0] == 0 && wugs[1] == 0 && wugs[2] == 0 {
            break;
        } else {
            println!("Enclosed {} wugs.", enclosed);
            println!();
        }
    }

    println!();
    println!("Congratulations! All wugs were found after {} tries!", n);
}

fn read_boundary(side: &str) -> isize {
    // Instatiate new String
    let mut boundary = String::new();

    // Print "[side] boundary: " to the console, flush stdout so output is printed immediately
    print!("{} boundary: ", &side);
    io::stdout().flush().unwrap();

    // Read line, store input in [boundary]. If this fails, raise error with message "Failed to read line"
    io::stdin().read_line(&mut boundary).expect("Failed to read line");

    // Try to parse input as isize (integer). If fails, set boundary to 0
    let boundary: isize = match boundary.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };

    return boundary;
}