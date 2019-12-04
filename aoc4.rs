const INPUT: &'static str = "178416-676461";

fn main() {
    let (a, b): (u32, u32) = {
        let split = INPUT.find('-').unwrap();
        ((&INPUT[0..split]).parse().unwrap(), (&INPUT[split+1..]).parse().unwrap())
    };
    
    let mut p1_found = 0u32;
    let mut p2_found = 0u32;
    'outer:
    for mut n in a..=b {
        let mut last = 10;
        let mut run = 0;
        let mut double = false;
        let mut long = false;
        
        while n > 0 {
            let d = n % 10;
            n /= 10;
            
            if d > last {
                continue 'outer;
            } else if d == last {
                run += 1;
                long = true;
            } else {
                if run == 1 {
                    double = true;
                }
                run = 0;
            }
            
            last = d;
        }
        if run == 1 {
            double = true;
        }
        
        if long {
            p1_found += 1;
        }
        if double {
            p2_found += 1;
        }
    }
    
    println!("p1: {}", p1_found);
    println!("p2: {}", p2_found);
}
