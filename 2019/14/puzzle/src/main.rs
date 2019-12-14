use std::collections::BTreeMap;
use std::io;
use std::io::prelude::*;

fn get_bill_of_materials(bill: &str) -> BTreeMap<String, u64> {
    let mut bill_of_materials = BTreeMap::new();
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
    bill_of_materials: &BTreeMap<String, u64>,
    material: &String,
    material_bill: &BTreeMap<String, u64>,
    min_count: u64,
    needed_count: u64,
) -> BTreeMap<String, u64> {
    let mut new_bill_of_materials = bill_of_materials.clone();
    new_bill_of_materials.remove(material);
    material_bill.iter().for_each(|(material, c)| {
        *new_bill_of_materials
            .entry(material.to_string())
            .or_insert(0) += c * ((needed_count + min_count - 1) / min_count);
    });
    new_bill_of_materials
}

fn get_ores(
    cache: &mut BTreeMap<BTreeMap<String, u64>, Option<u64>>,
    bills_of_materials: &BTreeMap<String, (u64, BTreeMap<String, u64>)>,
    bill_of_materials: BTreeMap<String, u64>,
) -> Option<u64> {
    if cache.contains_key(&bill_of_materials) {
        *cache.get(&bill_of_materials).unwrap()
    } else {
        let result = if bill_of_materials.len() == 1 && bill_of_materials.get("ORE").is_some() {
            Some(*bill_of_materials.get("ORE").unwrap())
        } else {
            bill_of_materials
                .iter()
                .map(
                    |(material, needed_count)| match bills_of_materials.get(material) {
                        Some((min_count, material_bill)) => get_ores(
                            cache,
                            bills_of_materials,
                            expand_material(
                                &bill_of_materials,
                                material,
                                material_bill,
                                *min_count,
                                *needed_count,
                            ),
                        ),
                        None => None,
                    },
                )
                .filter(|x| x.is_some())
                .min()
                .unwrap_or(None)
        };
        cache.insert(bill_of_materials, result);
        result
    }
}

fn main() {
    let mut bills_of_materials: BTreeMap<String, (u64, BTreeMap<String, u64>)> = BTreeMap::new();
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
    let (count, bill_of_materials) = bills_of_materials.get("FUEL").unwrap();
    assert!(*count == 1);
    let mut cache: BTreeMap<BTreeMap<String, u64>, Option<u64>> = BTreeMap::new();
    let result = get_ores(&mut cache, &bills_of_materials, bill_of_materials.clone());
    println!("{:?}", result);
}
