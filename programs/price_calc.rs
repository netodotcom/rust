use std::io;

#[derive(Clone)]
struct Item {
    name: String,
    price: f32,
}

enum MenuOption {
    AddItem(Item),
    CalculateTotal,
}

fn add_item(item: &Item, items: &mut Vec<Item>) {
    items.push(item.clone());
}

fn calculate_total(items: &[Item]) -> f32 {
    items.iter().map(|item| item.price).sum()
}

fn run() {
    let mut items = vec![];

    loop {
        println!("Type an item name and it's price or type C to calculate total.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail.");
        let input = input.trim();

        if input == "C" {
            break;
        }

        let split_input: Vec<_> = input.split(' ').collect();
        
        if split_input.len() != 2 {
            println!("Invalid input.");
            continue;
        }

        let name = split_input[0].to_string();
        let price = split_input[1].parse::<f32>();

        match price {
            Ok(price) => {
                let item = Item { name, price };
                add_item(&item, &mut items);
            }
            Err(_) => {
                println!("Invalid price.");
                continue;
            }
        }
    }
    println!("Calculating total...");
    let total = calculate_total(&items);
    println!("Total cost: ${:.2}", total);
}

fn main(){
    run();
}