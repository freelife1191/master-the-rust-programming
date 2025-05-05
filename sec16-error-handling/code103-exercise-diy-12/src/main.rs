/*
Exercise-diy-12
Partial code of the inventory management application 
The code currently uses panic!() for error handling, you have to refactor the code to 
replace all instances of panic! with proper error handling using the Result enum 
and a custom error type InventoryError. 
Ensure that each method in the Inventory struct returns a Result type, 
providing meaningful error messages and handling errors gracefully.
 */

use std::collections::HashMap;

//structure to describe an Item of the Inventory
#[derive(Debug, Clone)]
struct Item {
    id: u32,
    name: String,
    quantity: u32,
    price: f64,
}

//Inventory is collection of Items
#[derive(Debug)]
struct Inventory {
    items: HashMap<u32, Item>,
    next_item_id: u32, // Id of the next item to be added
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: HashMap::new(),
            next_item_id: 1, // Id of the next Item
        }
    }

    fn add_item(&mut self, name: String, quantity: u32, price: f64) {

        //remove this macro once you complete the TODOs
        todo!();

        //TODO
        if self.items.values().any(|item| item.name == name) {
            //Note: replace the panic!() by returning Result::Err()  
            // Result::Err() should contain enum error code DuplicateItem with 
            // existing item as it associated data.
            panic!("Item with name '{}' already exists!", name);
        }

        //If it is a new Item, add it to Inventory
        self.items.insert(TODO);

        //Update next_item_id
        self.next_item_id += 1;


    }


    //Note : Arguments are 'Option' types  to allow partial updates to the item
    fn update_item(&mut self, id: u32, name: Option<String>, quantity: Option<u32>, price: Option<f64>) {

        //remove this macro once you complete the TODOs
        todo!();

        //This would panic if item to be updated not found. 
        //Repalce it by returning Result::Err()
        //Result::Err() should contain enum error code ItemNotFound 
        let item = self.items.get_mut(&id).expect("Item not found!");

        //Update the item's name
        if let Some(name) = name {
            item.name = name;
        }

        //Update the item's quantity
        //TODO

        //Updat the item's price
        //TODO

    }

    fn delete_item(&mut self, id: u32) {

        //remove this macro once you complete the TODOs
        todo!();

        //Implement the logic to remove Item by Id
        //Hint1 : explore remove() method of HashMap
        //Hint2 : Explore is_none() method of Option Type
        //Return Result::Err() if item not found
        //Result::Err() should contain enum error code ItemNotFound 
    }

    fn list_items(&self) -> Vec<&Item> {
        self.items.values().collect()
    }

    fn find_item(&self, name: &str) -> &Item {

        //remove this macro once you complete the TODOs
        todo!();

        //This would panic if item to be found does not exist. 
        //Repalce it by returning Result::Err()
        //Result::Err() should contain enum error code ItemNotFound 
        //Hint : Explore using ok_or() in place of expect() 
        //ok_or() is covered in lecture 90: Converting Option<T> to Result<T, E> type
        self.items.values().find(|item| item.name == name).expect("Item not found!")
    }
}


fn main() {
    let mut inventory = Inventory::new();

    // Example usage
    match inventory.add_item("Laptop".to_string(), 10, 999.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }

    match inventory.add_item("Smartphone".to_string(), 20, 499.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }


    println!("/////Inventory/////");
    for item in inventory.list_items().unwrap() {
        println!("{:?}", item);
    }
    println!("/////End/////");

    inventory.update_item(1, Some("Gaming Laptop".to_string()), None, Some(1299.99)).unwrap();
    inventory.delete_item(2).unwrap();

    match inventory.find_item("Gaming Laptop") {
        Ok(item) => println!("Found item: {:?}", item),
        Err(e) => println!("Error: {:?}", e),
    }


    match inventory.find_item("Business Laptop") {
        Ok(item) => println!("Found item: {:?}", item),
        Err(e) => println!("Error: {:?}", e),
    }

    match inventory.add_item("Gaming Laptop".to_string(), 10, 879.99) {
        Ok(_) => println!("Item added!"),
        Err(e) => println!("Error: {:?}", e),
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_item_should_be_ok() {
        let mut inventory = Inventory::new();
        assert!(inventory.add_item("Test Item".to_string(), 10, 9.99).is_ok());
    }

    //Write more test cases
}