#[path = "battlegrounds/traits/card/card.rs"]
mod card; // Import the `card.rs` module

use card::Card;
use crate::card::Player;
// Bring the `Card` trait into scope

// Define a struct to represent a monster card
pub struct MonsterCard {
    pub name: String,
    pub description: String,
    pub cost: i32,
    pub card_type: String,
    pub rarity: String,
    pub set: String,
}

// Implement the `Card` trait for `MonsterCard`
impl Card for MonsterCard {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_cost(&self) -> i32 {
        self.cost
    }

    fn get_owner(&self) -> Player {
        todo!()
    }

    fn get_rarity(&self) -> String {
        self.rarity.clone()
    }

    fn get_set(&self) -> String {
        self.set.clone()
    }
}

fn main() {
    let my_card = MonsterCard {
        name: "Blue-Eyes White Dragon".to_string(),
        description: "This legendary dragon is a powerful engine of destruction.".to_string(),
        cost: 8,
        card_type: "Monster".to_string(),
        rarity: "Ultra Rare".to_string(),
        set: "Legend of Blue-Eyes".to_string(),
    };

    println!("Card Info:");
    println!("Name: {}", my_card.get_name());
    println!("Cost: {}", my_card.get_cost());
    println!("Rarity: {}", my_card.get_rarity());
    println!("Set: {}", my_card.get_set());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}