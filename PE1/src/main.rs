
fn main() {
	println!("Project Euler Problem 1");
	let mut total = 0;
	for i in 0..1000 {
		if i%3 == 0 ||  i%5 == 0 {
			total = total +  i;
		}
	}	
	println!("total :=> {}",total);
	
}
