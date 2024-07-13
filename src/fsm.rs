pub enum State{
    Idle,
    Moving_Up,
    Moving_Down,
    Moving_Left,
    Moving_Right,
    Attacking_Up,
    Attacking_Down,
    Attacking_Left,
    Attacking_Right,
    Despawning,
    Spawning
}
use State::*;
pub enum Input{
    Health,
    Attack_Cooldown,
    Up,
    Down,
    Left,
    Right,
    Attack_Key,
    No_Key
}

use Input::*;
pub struct FSM {state:State}

impl Copy for State{}

impl Clone for State{
    fn clone(&self) -> State {
        *self
    }
}


impl FSM{
    pub fn new () -> Self{
        Self{ state: Spawning}
    }

    pub fn recieve(self: &mut Self, input: Input){
        match (self.state , input) {
            (_, No_Key) => self.state = Idle,
            (Moving_Up,Attack_Key) => self.state = Attacking_Up,
            (Moving_Down,Attack_Key) => self.state = Attacking_Down,
            (Moving_Left,Attack_Key) => self.state = Attacking_Left,
            (Moving_Right,Attack_Key) => self.state = Attacking_Right,
            (_,Up) => self.state = Moving_Up,
            (_,Down) => self.state = Moving_Down,
            (_,Left) => self.state = Moving_Left,
            (_,Right) => self.state = Moving_Right,
            (_,Health) => self.state = Despawning,
            (_,_) => todo!()
        }
    }
}
