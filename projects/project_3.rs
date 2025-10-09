fn main() {
    let principal: f64 = 510_000.0;

    let rate: f64 = 5.0;

    let years: f64 = 3.0;

    let amount = principal * (1.0 - (rate / 100.0)).powf(years);

    let depreciation = principal - amount;

    println!("Original Value of TV: ₦{:.2}", principal);
    println!("Depreciation Rate: {}%", rate);
    println!("Time: {} years", years);
    println!("Value After Depreciation: ₦{:.2}", amount);
    println!("Total Depreciation in Value: ₦{:.2}", depreciation);
}
