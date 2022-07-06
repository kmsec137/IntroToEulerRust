use std::env;

fn main() {
	let mut fibo = vec![0,1];
	let args: Vec<String> = env::args().collect();	
	let limit = args[1].parse::<i32>().unwrap();
	let mut total = 0;
	while fibo[1] <  limit {
		let temp = fibo[1] + fibo[0];
		fibo[0] = fibo[1];
		fibo[1] = temp;
		if fibo[1] % 2 == 0 {
			total = total + fibo[1];	
		}
	}
	println!(" fibo:{:#?}, total:{}",fibo,total);
}
