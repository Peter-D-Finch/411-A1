use std::collections::LinkedList;
use std::collections::HashMap;
// TESTING CHANGE
fn add_element(element: String, mut map: HashMap<String, LinkedList<String>>) -> HashMap<String, LinkedList<String>> {
    let mut key = String::from("");
    let mut value = String::from("");
    let mut found_key = false;
    let ws_idx = element.chars().position(|c| c == ' ').unwrap();
    //println!("{0}", &element[ws_idx..]);
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
    if map.contains_key(&key) {list = map.remove(&key).unwrap();} else {list = LinkedList::new();}
    list.push_back(value);
    map.insert(key, list);
    return map;
}
fn main() {
    let test = "A Allen\nB Jack\nA Emily\nC Seth\nC Becca\n";
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
