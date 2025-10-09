fn main() {
    let principal: f64 = 520_000_000.0;

    let rate: f64 = 10.0;

    let years: f64 = 5.0;

    let amount = principal * (1.0 + (rate / 100.0)).powf(years);

    let compound_interest = amount - principal;

    println!("Principal: ₦{:.2}", principal);
    println!("Rate: {}%", rate);
    println!("Time: {} years", years);
    println!("Total Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}
