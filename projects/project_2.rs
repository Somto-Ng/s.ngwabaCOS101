fn main() {
    let sales = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

    let total: f64 = sales.iter().sum();

    let average = total / sales.len() as f64;

    println!("Sales Record (in ₦): {:?}", sales);

    println!("Total Sales: ₦{:.2}", total);

    println!("Average Sales: ₦{:.2}", average);
}
