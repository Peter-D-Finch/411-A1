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
    let key = element.splitn(2, ' ').collect::<Vec<&str>>()[0].to_string();
    let value = element.splitn(2, ' ').collect::<Vec<&str>>()[1].to_string();
    let mut list: LinkedList<String>;
    if key.chars().count() > 512 { eprintln!("Error: Fingerprint length exceeds 512 characters."); }
    else {
        if map.contains_key(&key) {list = map.remove(&key).unwrap();}
        else {list = LinkedList::new();}
        list.push_back(chk_ws(value));
        map.insert(key, list);
    }
    return map;
}

fn parse_input(x: String, mut map: HashMap<String, LinkedList<String>>) -> HashMap<String, LinkedList<String>> {
    if x.contains("\n") {
        let line = x.splitn(2, '\n').collect::<Vec<&str>>()[0].to_string();
        map = add_element(line, map);
        map = parse_input(x.splitn(2, '\n').collect::<Vec<&str>>()[1].to_string(), map);
    }
    else {
        let line = x.splitn(2, '\n').collect::<Vec<&str>>()[0].to_string();
        map = add_element(line, map);
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
    map = parse_input(test, map);
    let lists: Vec<LinkedList<String>> = map.values().cloned().collect();
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