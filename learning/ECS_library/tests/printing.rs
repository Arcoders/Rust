use std::ops::Deref;
#[test]
fn structs() {
    let mut player_health = Health(100);
    let enemy_health = Health::new(50);

    player_health.lose_health(10);

    player_health.print_health();
    enemy_health.print_health()

}

struct Health(pub u32);

impl Health {
    pub fn new(data: u32) -> Self {
        Self(data)
    } 

    pub fn lose_health(&mut self, amount: u32) {
        self.0 -= amount  ; 
    }

    pub fn print_health(&self) {
        println!("player health: {}", **self);
    }
}  

impl Deref for Health {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}