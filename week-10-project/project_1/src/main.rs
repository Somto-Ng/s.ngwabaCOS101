struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: "HP".to_string(), price: 650_000 };
    let ibm = Laptop { brand: "IBM".to_string(), price: 755_000 };
    let toshiba = Laptop { brand: "Toshiba".to_string(), price: 550_000 };
    let dell = Laptop { brand: "Dell".to_string(), price: 850_000 };

    let qty = 3;

    let total =
        hp.total_cost(qty) +
        ibm.total_cost(qty) +
        toshiba.total_cost(qty) +
        dell.total_cost(qty);

    println!("Total cost for buying 3 from each brand is: ₦{}",total);
}