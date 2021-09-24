use std::collections::LinkedList;
use std::collections::HashMap;
use std::io::{self, BufRead};

// A function that is used recursively to remove whitespace from the beginning of a String
fn chk_ws(mut string: String) -> String {
    let result;
    if string.chars().nth(0).unwrap() == ' ' || string.chars().nth(0).unwrap() == '\t' {
        string.remove(0);
        result = chk_ws(string);
    }
    else { result = string; }
    return result;
}

// A function that takes a string containing fingerprint and name, seperates them, and adds them to HashMap<String, LinkedList<String>>
fn add_element(element: String, mut map: HashMap<String, LinkedList<String>>) -> HashMap<String, LinkedList<String>> {
    // instantiating variables to hold the key, value, and whether we've encountered whitespace yet
    let mut key = String::from("");
    let mut value = String::from("");
    let mut found_key = false;

    // Iterates through every character in String passed to function
    for x in 0..element.chars().count() {
        // If the current character isn't whitespace and no whitespace has been encountered yet ... 
        if element.chars().nth(x).unwrap() != ' ' && element.chars().nth(x).unwrap() != '\t'  && found_key == false {
            // ... Then add the character to the key
            key.push(element.chars().nth(x).unwrap());
        }
        // Else if the character is whitespace, or if we've encountered a whitespace before this...
        else {
            // ... Add character to name
            found_key = true;
            value.push(element.chars().nth(x).unwrap());
        }
    }

    let mut list: LinkedList<String>;
    // If the fingerprint is greater than 512 characters, print to stderr
    if key.chars().count() > 512 { eprintln!("Error: Fingerprint length exceeds 512 characters."); }
    else {
        // If the map already has fingerprint, get it's LinkedList<String>
        if map.contains_key(&key) {list = map.remove(&key).unwrap();}
        // Else create a new LinkedList<String>
        else {list = LinkedList::new();}

        // Add the name and insert into HashMap
        list.push_back(chk_ws(value));
        map.insert(key, list);
    }
    return map;
}

fn main() {
    let now = Instant::now();
    // Get input from user
    let mut test = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut test).unwrap();
    test = test.replace("\\n", "\n");

    let mut tracker = 0;
    // Parse the different lines of input
    let mut temp = String::from("");
    let mut map: HashMap<String, LinkedList<String>> = HashMap::new();
    for x in 0..test.chars().count() {
        if test.chars().nth(x).unwrap() != '\n' {
            temp.push(test.chars().nth(x).unwrap());
        }
        else {
            // Add each line of input to the HashMap using add_element function
            tracker = tracker + 1;
            map = add_element(temp, map);
            temp = "".to_string();
        }
    }
    // Add the last line of input to the HashMap
    map = add_element(temp, map);

    // Collect fingerprint groups as vector
    let lists: Vec<LinkedList<String>> = map.values().cloned().collect();

    // Displaying the fingerprint groups by maintaining state of "Have we printed a group?"
    let mut statement = "";
    for x in 0..map.len() {
        if lists[x].len() > 1 {
            print!("{0}", statement);
            let mut iter = lists[x].iter();
            for _y in 0..lists[x].len() {
                println!("{0}", iter.next().unwrap());
            }
            statement = "\n";
        }
    }
}