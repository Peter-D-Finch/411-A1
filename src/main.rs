use std::collections::LinkedList;
use std::collections::HashMap;
use std::io::{self, BufRead};
fn chk_ws(mut string: String) -> String {
    let result;
    if string.chars().nth(0).unwrap() == ' ' || string.chars().nth(0).unwrap() == '\t' {
        string.remove(0);
        result = chk_ws(string);
    }
    else { result = string; }
    return result;
}
fn add_element(element: String, mut map: HashMap<String, LinkedList<String>>) -> HashMap<String, LinkedList<String>> {
    let mut key = String::from("");
    let mut value = String::from("");
    let mut found_key = false;
    for x in 0..element.chars().count() {
        if element.chars().nth(x).unwrap() != ' ' && found_key == false {
            key.push(element.chars().nth(x).unwrap());
        }
        else {
            found_key = true;
            value.push(element.chars().nth(x).unwrap());
        }
    }
    let mut list: LinkedList<String>;
    if key.chars().count() > 512 { eprintln!("Error: Fingerprint length exceeds 512 characters."); }
    else {
        if map.contains_key(&key) {list = map.remove(&key).unwrap();} else {list = LinkedList::new();}
        list.push_back(chk_ws(value));
        map.insert(key, list);
    }
    return map;
}
fn main() {
    let mut test = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut test).unwrap();
    test = test.replace("\\n", "\n");
    let mut temp = String::from("");
    let mut map: HashMap<String, LinkedList<String>> = HashMap::new();
    for x in 0..test.chars().count() {
        if test.chars().nth(x).unwrap() != '\n' {
            temp.push(test.chars().nth(x).unwrap());
        }
        else {
            map = add_element(temp, map);
            temp = "".to_string();
        }
    }
    map = add_element(temp, map);
    let lists: Vec<LinkedList<String>> = map.values().cloned().collect();
    for x in 0..map.len() {
        if lists[x].len() > 1 {
            let mut iter = lists[x].iter();
            for _y in 0..lists[x].len() {
                println!("{0}", iter.next().unwrap());
            }
            print!("\n");
        }
    }
}