
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::fs::File;
use std::time::SystemTime;
fn read_numbers(file: &mut BufRead ) -> Result<Vec<i64>, Box<dyn Error>> {
   let mut buffer: Vec<i64> = vec![]; 
   for lines in file.lines() {
        let line = lines?;
        let num:i64= line.parse()?;
        buffer.push(num);
   }
   Ok(buffer)
} 

fn main() -> Result<(), Box<dyn Error>>{
    let mut ran = Xorshift64::new_with_seed(7348437894759);
    println!("{}", ran.geometric(0.5));

    Ok(())
}
  
#[derive(Debug)]
struct Xorshift64 {
    seed: u64,
    gen:f64

}

impl Xorshift64 {
    fn randomize(&mut self)  {
        // Seed musn't be 0.
        if self.seed == 0 {
            self.seed = 12349318591;
        }

        let mut x:u64 = self.seed; 
        x = x^(x << 13);
        x = x^(x << 17);
        x = x^(x << 5);
        self.gen = (x as f64) * 5.42101086242752217e-20; // normilization constant
    }



    fn next(&mut self) -> &mut Self {
        assert!(self.gen != 0.0);
        self.seed = (self.gen * 12389849214348391.3) as u64;
        self.randomize();
        self
    }
    
    pub fn uniform_rv(&mut self) -> f64 {
        self.next();
        self.gen
    }

    pub fn new() -> Xorshift64 {
        let t= SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let seed = t.as_secs() ^ 47381849; // random password
        let mut s = Xorshift64{seed, gen:0.0}; 
        s.randomize();
        s
    }

    pub fn new_with_seed(seed:u64) -> Xorshift64 {
        let mut  s = Xorshift64{seed, gen:0.0};
        s.randomize();
        s
    }

    // Available distributions 



    // Bernoulli, bernoulli(0.5) is a coinflip
    pub fn bernoulli(&mut self, p:f64) -> bool {
        self.uniform_rv() <= p 
    }

    // binomial is the sum of n bernoulli experiments
    pub fn binomial(&mut self, p:f64, n:i32) -> i32{
        let mut result = 0;
        for _ in 0..n {
            if self.bernoulli(p) {
                result += 1;
            }

        }
        result
    }
 
    // Is this even correct? I think so
    pub fn geometric(&mut self, p:f64) -> i32 {
        let mut res = 0;
        while self.bernoulli(p){ 
            res += 1;
        }
        res
    }
    pub fn uniform (&mut self, a:i32, b:i32) {
        panic!()
    }

    // Todo:
    // Implement uniform, normal/polar method, gamma(?), and some of the distributions derived from gamma.


}
#[test]
// Can we get around 50% coinflip accuracy?
fn test() {
    let mut s = Xorshift64::new();
    let m:Vec<bool> = (0..100000).map(|_| s.bernoulli(0.5)).collect();
    let truth = m.iter().filter(|f| **f  ).collect::<Vec<&bool>>().len();
    let percent = (truth as f64)/(m.len() as f64);
    println!("{}", percent);
    assert!(percent > 0.47)
}