use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io,
    os::linux::raw,
};

// Inventory Management: The system should allow store managers to add, edit, and delete products from the inventory. Each product should have a name, description, price, and quantity.
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

impl Product {
    pub fn is_valid(&self) -> bool {
        self.name.len() > 0 && self.price > 0.0 && self.quantity > 0
    }

    pub fn new_with_prompt() -> Self {
        let name = read_input("Enter product name:");
        let description = read_input("Enter product description:");
        let price = loop {
            let price = read_input("Enter product price:");
            match parse_float(&price) {
                Ok(price) => break price,
                Err(err) => println!("{}", err),
            }
        };
        let quantity = loop {
            let quantity = read_input("Enter product quantity:");
            match parse_uint(&quantity) {
                Ok(quantity) => break quantity as i32,
                Err(err) => println!("{}", err),
            }
        };
        Self {
            name,
            description,
            price,
            quantity,
        }
    }
}

// The system should allow store managers to record sales transactions, including the product sold, the quantity sold, and the sale price. The system should also calculate and display the total sales and profit made from each transaction.
struct SaleTransaction {
    product: Product,
    quantity: i32,
    sale_price: f64,
}

impl SaleTransaction {
    fn total_sales(&self) -> f64 {
        self.quantity as f64 * self.sale_price
    }
}

// The system should allow store managers to record purchase transactions, including the product purchased, the quantity purchased, and the purchase price. The system should also calculate and display the total cost of each purchase.

struct PurchaseTransaction {
    product: Product,
    quantity: i32,
    purchase_price: f64,
}

impl PurchaseTransaction {
    fn total_purchase(&self) -> f64 {
        self.quantity as f64 * self.purchase_price
    }
}

struct Inventory {
    security_hash: u64,
    products: Vec<Product>,
    sales: Vec<SaleTransaction>,
    purchases: Vec<PurchaseTransaction>,
}

#[derive(Hash)]
struct Auth {
    username: String,
    password: String,
}

impl Auth {
    pub fn generate_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Inventory {
    fn new(sec: Auth) -> Self {
        Self {
            security_hash: sec.generate_hash(),
            products: Vec::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }

    fn auth_with_prompt(&self) -> Result<(), String> {
        let username = read_input("Enter username:");
        let password = read_input("Enter password:");
        let auth = Auth { username, password };
        if auth.generate_hash() == self.security_hash {
            Ok(())
        } else {
            Err("Invalid username or password".to_string())
        }
    }

    fn is_product_exist(&self, name: &str) -> bool {
        self.get_product(name).is_some()
    }

    fn get_product(&self, name: &str) -> Option<&Product> {
        self.products.iter().find(|product| product.name == name)
    }

    fn add_product(&mut self, product: Product) -> Result<(), String> {
        if !product.is_valid() {
            return Err("Invalid product details".to_string());
        }

        if self.is_product_exist(&product.name) {
            return Err("Product already exists".to_string());
        }

        self.products.push(product);
        Ok(())
    }

    fn edit_product(&mut self, index: usize, product: Product) -> Result<(), String> {
        if index >= self.products.len() {
            return Err("Product index out of range".to_string());
        }

        if !product.is_valid() {
            return Err("Invalid product details".to_string());
        }

        self.products[index] = product;
        Ok(())
    }

    fn delete_product(&mut self, index: usize) -> Result<(), String> {
        if index >= self.products.len() {
            return Err("Product index out of range".to_string());
        }
        self.products.remove(index);
        Ok(())
    }

    fn record_sale(&mut self, sale: SaleTransaction) -> Result<(), String> {
        let product = self
            .get_product(&sale.product.name)
            .ok_or("Product does not exist".to_string())
            .unwrap();

        if product.quantity < sale.quantity {
            return Err("Out of stock".to_string());
        }
        self.sales.push(sale);
        Ok(())
    }

    fn record_purchase(&mut self, purchase: PurchaseTransaction) -> Result<(), String> {
        let _ = &self
            .get_product(&purchase.product.name)
            .ok_or("Product does not exist".to_string())?;

        self.purchases.push(purchase);
        Ok(())
    }

    fn total_sales(&self) -> f64 {
        self.sales.iter().map(|sale| sale.total_sales()).sum()
    }

    fn total_purchase(&self) -> f64 {
        self.purchases
            .iter()
            .map(|purchase| purchase.total_purchase())
            .sum()
    }

    fn total_profit(&self) -> f64 {
        let total_sales = self.total_sales();
        let total_purchase = self.total_purchase();
        total_sales - total_purchase
    }

    fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("Inventory Report\n");
        report.push_str("Name\t\tDescription\t\tPrice\t\tQuantity\n");
        for product in &self.products {
            report.push_str(&format!(
                "{}\t\t{}\t\t{}\t\t{}\n",
                product.name, product.description, product.price, product.quantity
            ));
        }
        report.push_str("\nSales Report\n");
        report.push_str("Name\t\tQuantity\t\tSale Price\t\tTotal Sales\n");
        for sale in &self.sales {
            report.push_str(&format!(
                "{}\t\t{}\t\t{}\t\t{}\n",
                sale.product.name,
                sale.quantity,
                sale.sale_price,
                sale.total_sales()
            ));
        }
        report.push_str(&format!("\nTotal Sales: {}\n", self.total_sales()));

        report.push_str("\nPurchase Report\n");

        report.push_str("Name\t\tQuantity\t\tPurchase Price\t\tTotal Purchase\n");
        for purchase in &self.purchases {
            report.push_str(&format!(
                "{}\t\t{}\t\t{}\t\t{}\n",
                purchase.product.name,
                purchase.quantity,
                purchase.purchase_price,
                purchase.total_purchase()
            ));
        }

        report.push_str(&format!("\nTotal Purchase: {}\n", self.total_purchase()));

        report.push_str(&format!("\nTotal Profit: {}\n", self.total_profit()));
        report
    }
}

// Function to read user input.
fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Function to parse a floating point number from user input.
fn parse_float(input: &str) -> Result<f64, String> {
    input
        .parse()
        .map_err(|_| "Invalid input. Please enter a valid number.".to_string())
}

// Function to parse an unsigned integer from user input.
fn parse_uint(input: &str) -> Result<u32, String> {
    input
        .parse()
        .map_err(|_| "Invalid input. Please enter a valid number.".to_string())
}

fn main() {
    println!("Let setup an inventory management system for you...");
    let username = read_input("Enter username:");
    let password = read_input("Enter password:");

    let mut inventory = Inventory::new(Auth { username, password });

    println!("Inventory management system is ready...");

    loop {
        println!("\n===== Inventory Management System =====");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("0. Exit");

        let choice = read_input("Enter your choice:");

        match choice.as_str() {
            "1" => {
                let auth = inventory.auth_with_prompt();
                match auth {
                    Ok(_) => {
                        let result = inventory.add_product(Product::new_with_prompt());
                        match result {
                            Ok(_) => println!("Product added successfully."),
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            }

            "2" => {
                let auth = inventory.auth_with_prompt();
                match auth {
                    Ok(_) => {
                        let input = read_input("Enter product index:");
                        let index = parse_uint(&input).unwrap();
                        let product = Product::new_with_prompt();
                        println!("Editing product... {}", product.name);
                        let result = inventory.edit_product(index as usize, product);
                        match result {
                            Ok(_) => println!("Product edited successfully."),
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            }
            "3" => {
                let auth = inventory.auth_with_prompt();
                match auth {
                    Ok(_) => {
                        let input = read_input("Enter product index:");
                        let index = parse_uint(&input).unwrap();
                        let result = inventory.delete_product(index as usize);
                        match result {
                            Ok(_) => println!("Product deleted successfully."),
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(err) => {
                        println!("{}", err);
                        continue;
                    }
                }
            }
            "4" => {
                println!("Generating report...");
                println!("{}", inventory.generate_report());
            }

            "0" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a number between 1 and 7.");
            }
        };
    }
}
