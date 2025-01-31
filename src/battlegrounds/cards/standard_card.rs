#[path = "../traits/card/card.rs"]
mod card; // Import the `card.rs` module

use card::Card;
use crate::standard_card::card::Player;

pub struct StandardCard {
    pub name: String,
    pub description: String,
    pub cost: i32,
    pub health: i32,
    pub owner: Player,
    pub attack: i32,
    pub is_active: bool,
}

impl Card for StandardCard {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_cost(&self) -> i32 {
        self.cost
    }

    fn get_owner(&self) -> *const Player {
        &self.owner
    }

    fn get_health(&self) -> i32 {
        self.health
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn damage(&mut self, damage: i32) {
        self.health -= damage
    }

    fn heal(&mut self, heal: i32) {
        self.health += heal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_card() -> StandardCard {
        StandardCard {
            cost : 3,
            health: 10,
            owner: Player::Findus,
            attack : 7,
            name : "Bob".to_string(),

            description: "This is a standard card".to_string(),
            is_active: false,
        }
    }
    #[test]
    fn damage_is_taken_properly() {
        let mut card = create_test_card();
        let damage = 5;
        let new_health = card.get_health() - damage;
        card.damage(damage);
        assert_eq!(card.get_health(), new_health)
    }

    #[test]
    fn healing_heals_card() {
        let mut card = create_test_card();
        let heal = 5;
        let new_health = card.get_health() + heal;
        card.heal(heal);
        assert_eq!(card.get_health(), new_health)
    }
}