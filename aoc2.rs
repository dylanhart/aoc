const INPUT: &'static str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,19,5,23,2,10,23,27,2,27,13,31,1,10,31,35,1,35,9,39,2,39,13,43,1,43,5,47,1,47,6,51,2,6,51,55,1,5,55,59,2,9,59,63,2,6,63,67,1,13,67,71,1,9,71,75,2,13,75,79,1,79,10,83,2,83,9,87,1,5,87,91,2,91,6,95,2,13,95,99,1,99,5,103,1,103,2,107,1,107,10,0,99,2,0,14,0";
// const INPUT: &'static str = "1,9,10,3,2,3,11,0,99,30,40,50";

fn run_prog(image: &Vec<u32>, noun: u8, verb: u8) -> u32 {
    let mut mem = image.clone();
    let mut pc = 0;
    
    mem[1] = noun as u32;
    mem[2] = verb as u32;
    
    loop {
        match mem[pc] {
            1 => {
                let dst = mem[pc+3] as usize;
                mem[dst] = mem[mem[pc+1] as usize] + mem[mem[pc+2] as usize];
            },
            2 => {
                let dst = mem[pc+3] as usize;
                mem[dst] = mem[mem[pc+1] as usize] * mem[mem[pc+2] as usize];
            },
            99 => break,
            _ => panic!("wtf?"),
        }
        pc += 4;
    }
    
    return mem[0];
}

fn main() {
    let image: Vec<u32> = INPUT.split(",").map(str::parse).map(Result::unwrap).collect();
    
    println!("p1: out = {}", run_prog(&image, 12, 02));
    
    for n in 0..100 {
        for v in 0..100 {
            let out = run_prog(&image, n, v);
            if out == 19690720 {
                println!("p2: n: {}, v: {}", n, v);
                break;
            }
        }
    }
}
