use std::env;
use std::f32;

fn sieve(limit: i32) -> Vec<i32> {
	let mut sieve: Vec<i32>	= (0..limit).collect();
	for i in 0..sieve.len() {
		if i % 2 == 0 && i > 2 {
			sieve[i] = 1;
		} else { sieve[i] = 0; }
	}
	let s_limit: usize = (sieve.len() as f32).sqrt() as usize;
	for i in 3..s_limit {
		if sieve[i] == 0 {
			let b: usize = i*i;
			let s: usize = 2*i;
			for j in (b..sieve.len()).step_by(s) {
				sieve[j] = 1;	
   			}
		}
			
	}
	return sieve;
}
fn flip(inarr: Vec<i32>) -> Vec<i32>{
	let mut outarr: Vec<i32> = (0..(inarr.len() as i32)).collect();
	let mut out_index: usize = 0;
	for i in 0..inarr.len(){
		if inarr[i] == 0 {
			outarr[out_index] = (i as i32);
			out_index = out_index + 1;
		}	
	}
	return outarr;
}

fn sieve_print(inarr: Vec<i32>) {
	for i in 0..(inarr.len() as usize) {
		if inarr[i] == 0 {
			print!("{} ",i);
		}	
	}	
	println!();
}
fn main() {
	let args: Vec<String> = env::args().collect();
	let limit: i32 = args[1].parse::<i32>().unwrap();
	sieve_print(sieve(limit));
}
