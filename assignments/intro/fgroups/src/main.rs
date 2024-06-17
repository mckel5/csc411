use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let lines = io::stdin().lock().lines();
    let fingerprints_map = assemble_map(lines);
    print_groups(&fingerprints_map);
}

fn assemble_map(lines: io::Lines<io::StdinLock<'_>>) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    
    for line in lines {
        let text = line.unwrap();
        let split: Vec<&str> = text.splitn(2, [' ', '\t']).collect();
        // Discard any improperly formatted lines, i.e. those without a
        // distinct fingerprint and name
        if split.len() != 2 { continue };

        let group = split[0].to_string();
        // Trailing whitespace must be kept in name
        let name = split[1].trim_start().to_string();
        
        if !map.contains_key(&group) {
            map.insert(group.to_string(), Vec::new());
        
        }

        map.get_mut(&group).unwrap().push(name.to_string());
    }
    
    map
}

fn print_names(names: &Vec<String>) {
    for name in names {
        println!("{}", name);
    }
}

fn print_groups(groups: &HashMap<String, Vec<String>>) {
    // Keeping the separator in its own variable and printing it *before* each
    // group allows us to avoid the dreaded extra newline
    let mut separator = "";
    for names in groups.values() {
        if names.len() <= 1 { continue };

        print!("{}", separator);
        print_names(names);
        separator = "\n";
    }
}