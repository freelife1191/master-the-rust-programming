/*
Exercise-diy-13 : Implement a simple generic stack data structure
Requirements
    ==> Define a generic Stack struct that can store items of any type.
Implement the following methods for the Stack:
    ==> new: Creates a new, empty stack.
    ==> push: Adds an item to the top of the stack.
    ==> pop: Removes and returns the item from the top of the stack.
    ==> peek: Returns a reference to the top item without removing it.
    ==> is_empty: Checks if the stack is empty.
    ==> size: Returns the number of items in the stack.
    ==> clear: Removes all items from the stack.

Hints:
    ==> Define a generic struct whose name is 'Stack' generic over T
    ==> USe a 'Vector' to store the items
 */


// Define the generic Stack struct
//TODO

//Create a generic impl block
//TODO
//Note : This is partial implementation

impl Stack {
    // Creates a new, empty stack
    fn new()  {

    }

    // Adds an item to the top of the stack
    fn push() {

    }

    // Removes and returns the item from the top of the stack
    fn pop()  {

    }

    // Returns a reference to the top item without removing it
    fn peek() {

    }

    // Checks if the stack is empty
    fn is_empty() -> bool {
        false
    }

    // Returns the number of items in the stack
    fn size() -> usize {
        0
    }

    // Removes all items from the stack
    fn clear() {

    }

}


fn main() {

}

#[cfg(test)]
mod test {
    use crate::Stack;

    #[test]
    fn test_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(true, stack.is_empty());
    }

    #[test]
    fn test_stack_is_not_empty() {
        let mut stack: Stack<&str> = Stack::new();
        stack.push("Rose");
        assert_eq!(false, stack.is_empty());
    }

    #[test]
    fn test_stack_clear() {
        let mut stack_of_temperatures = Stack::new();
        stack_of_temperatures.push(33.3);
        stack_of_temperatures.push(34.9);
        stack_of_temperatures.clear();
        assert_eq!(true, stack_of_temperatures.is_empty());
        assert_eq!(0, stack_of_temperatures.size());
        assert_eq!(None, stack_of_temperatures.pop());
        assert_eq!(None, stack_of_temperatures.peek());
    }

    //write more test cases. 
}