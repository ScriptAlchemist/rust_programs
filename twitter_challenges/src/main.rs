use std::collections::btree_map::BTreeMap;

struct Person { blood_alcohol: f32 }

fn main() {

    let orders = vec![1, 2, 1, 2, 3, 4, 1, 2, 2, 3, 4, 1, 1, 1];

    let mut blood_alcohol = BTreeMap::new();
    for id in orders {
        println!("{id} ordering a beer");
        let person = blood_alcohol.entry(id).or_insert(Person { blood_alcohol: 0.0 });

        person.blood_alcohol *= 0.9;

        if person.blood_alcohol > 0.3 {
            println!("Sorry {id}, I have to cut you off");
        } else {
            person.blood_alcohol += 0.1;
        }
    }
}

