use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn get_bill_of_materials(bill: &str) -> HashMap<String, u64> {
    let mut bill_of_materials = HashMap::new();
    bill.split(",").for_each(|s| {
        let component: Vec<&str> = s.split_whitespace().collect();
        bill_of_materials.insert(
            component[1].to_string(),
            component[0].parse::<u64>().unwrap(),
        );
    });
    bill_of_materials
}

fn expand_material(
    bills_of_materials: &HashMap<String, (u64, HashMap<String, u64>)>,
    bill_of_materials: &mut HashMap<String, u64>,
    material: &String,
    mut need: u64,
    excess: &mut HashMap<String, u64>,
) {
    bill_of_materials.remove(material);
    let (get, material_bill) = bills_of_materials.get(material).unwrap();
    let mut excess_material = excess.entry(material.to_string()).or_insert(0);
    if need <= *excess_material {
        *excess_material -= need;
        return;
    }
    need -= *excess_material;
    let reactions = (need + get - 1) / get;
    *excess_material = reactions * get - need;
    material_bill.iter().for_each(|(material, material_need)| {
        *bill_of_materials.entry(material.to_string()).or_insert(0) +=
            material_need * reactions;
    });
}

fn get_ores(
    bills_of_materials: &HashMap<String, (u64, HashMap<String, u64>)>,
    material: &str,
    need: u64
) -> u64 {
    let mut excess: HashMap<String, u64> = HashMap::new();
    let mut bill_of_materials: HashMap<String, u64> = HashMap::new();
    bill_of_materials.insert(material.to_string(), need);
    loop {
        if bill_of_materials.len() == 1 && bill_of_materials.get("ORE").is_some() {
            return *bill_of_materials.get("ORE").unwrap();
        }
        let (material, needed_count) = bill_of_materials
            .remove_entry(
                &bill_of_materials
                    .keys()
                    .filter(|k| k != &"ORE")
                    .nth(0)
                    .unwrap()
                    .to_string(),
            )
            .unwrap();
        expand_material(
            bills_of_materials,
            &mut bill_of_materials,
            &material,
            needed_count,
            &mut excess,
        );
    }
}

fn main() {
    let mut bills_of_materials: HashMap<String, (u64, HashMap<String, u64>)> = HashMap::new();
    io::stdin().lock().lines().for_each(|line| {
        line.map(|l| {
            let bill: Vec<&str> = l.split("=>").collect();
            let result: Vec<&str> = bill[1].split_whitespace().collect();
            bills_of_materials.insert(
                result[1].to_string(),
                (
                    result[0].parse::<u64>().unwrap(),
                    get_bill_of_materials(bill[0]),
                ),
            )
        })
        .unwrap();
    });
    let ore_per_fuel = get_ores(&bills_of_materials, "FUEL", 1) as i64;
    println!("{}", ore_per_fuel);
    let available_ore = 1000000000000;
    let mut left = 1;
    let mut right = available_ore;
    assert!(get_ores(&bills_of_materials, "FUEL", right as u64) > available_ore as u64);
    loop {
        let mid = (left + right) / 2;
        let result = get_ores(&bills_of_materials, "FUEL", mid as u64) as i64 - available_ore;
        if result > 0 {
            right = mid;
        } else if result < -ore_per_fuel {
            left = mid;
        } else {
            println!("{}", mid);
            break;
        }
    }
}
