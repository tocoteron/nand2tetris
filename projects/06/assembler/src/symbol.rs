use std::{collections::HashMap};

pub type SymbolTable = HashMap<String, u16>;

pub fn make_symbol_table(program: &Vec<String>) -> SymbolTable {
    let registers = (0..16).map(|i| (format!("R{}", i), i));
    let defined_symbols: Vec<(String, u16)> = vec![
        ("SP".to_string(), 0),
        ("LCL".to_string(), 1),
        ("ARG".to_string(), 2),
        ("THIS".to_string(), 3),
        ("THAT".to_string(), 4),
        ("SCREEN".to_string(), 16384),
        ("KBD".to_string(), 24576),
    ].into_iter().chain(registers).collect();
    
    let mut symbol_table: SymbolTable = defined_symbols.into_iter().collect();
    let mut var_address: u16 = 0x0010;

    let program: Vec<String> = program
        .iter()
        .map(clean_line)
        .filter(|line| !line.is_empty())
        .collect();

    // Collect labels
    let mut parsed_labels = 0;
    program
        .iter()
        .enumerate()
        .filter(|(_, line)| is_label(line))
        .for_each(|(i, line)| {
            let label = line.split(['(', ')']).collect::<Vec<&str>>()[1];
            if !symbol_table.contains_key(label.into()) {
                symbol_table.insert(label.into(), (i - parsed_labels) as u16);
                parsed_labels += 1;
            }  
        });

    // Collect variables
    program
        .iter()
        .filter(|line| is_variable(line))
        .for_each(|line| {
            let mut line = line.to_string();
            let variable: String = line.drain(1..).collect();
            if !symbol_table.contains_key(&variable) {
                symbol_table.insert(variable, var_address);
                var_address += 1;
            }
    });

    symbol_table
}

fn clean_line(line: &String) -> String {
    let mut cleaned_line = line.clone();
    let comment_offset = cleaned_line.find("//").unwrap_or(cleaned_line.len());
    cleaned_line.drain(comment_offset..);
    cleaned_line.trim().to_string()
}

fn is_label(line: &str) -> bool {
    line.starts_with("(") && line.ends_with(")")
}

fn is_variable(line: &str) -> bool {
    if line.starts_with("@") {
        let mut line = line.to_string();
        line
            .drain(1..)
            .collect::<String>()
            .parse::<u16>()
            .is_err()
    } else {
        false
    }
}