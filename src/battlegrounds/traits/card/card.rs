#[path = "../player/player.rs"]
mod player;

pub use player::Player;

pub trait Card {
    fn get_name(&self) -> &String;          // Gets the name of the card
    fn get_cost(&self) -> i32;             // Gets the cost of the card
    fn get_owner(&self) -> *const Player;         // Gets the player that owns said card.
    fn get_health(&self) -> i32;           // Gets the current health of the card.
    fn is_active(&self) -> bool;           // Returns true if the card has not attacked this turn.

    fn damage(&mut self, damage: i32);

    fn heal(&mut self, heal: i32);
}