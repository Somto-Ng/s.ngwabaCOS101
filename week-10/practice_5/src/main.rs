fn main() {

	// a list of nos
	let x = vec![100,200,300];
	borrow_vector(&x); // passing referance

	println!("Printing the value from main() x[0]={}",x[0]);
	println!(****************************);
}

fn borrow_vector(z:&Vec<132>) {

	println!(****************************);
	println!("Inside print_vector function {:?} \n",z);
	println!(----------------------------);
}