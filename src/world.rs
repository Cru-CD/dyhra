use std::ops::Index;

use super::*;
use map::*;
use entity::*;
use fsm;


pub struct World {
    map: Map,
    camera: Camera2D,
    player: Entity,
    enemies: Vec<Entity>,
    time: f64,
    uid: Vec<u32>
}

impl World{
    pub async fn new() -> Self {
        let wolf_animation = vec![
            Entity::animation("idle_up", 11,8,5),
            Entity::animation("idle_left", 10, 8, 5),
            Entity::animation("idle_down", 8,8,5),
            Entity::animation("idle_right",9,8,5),
            Entity::animation("walk_up", 15, 8, 15),
            Entity::animation("walk_left", 14, 8,15),
            Entity::animation("walk_down", 12, 8,15),
            Entity::animation("walk_right", 13, 8,15)
        ];
        let mut enemies = Vec::new();
        let mut uids : Vec<u32> = Vec::new();
        
        let player_uid : u32 = 1;

        uids.push(player_uid);

        for _ in 0..150{
            let mut uid : u32 = Self::uid_gen();
            while uids.contains(&uid) {
                uid = Self::uid_gen();
            }
            let machine = fsm::FSM::new();
            uids.push(uid);
            enemies.push(
                Entity::new(
                    rand::gen_range(-1600.0, 1600.0),
                    rand::gen_range(800.0, 2200.0),
                    32.0,32.0,
                    "assets/critters/wolf/wolf-all.png",
                    wolf_animation.clone(),
                    20.0,
                    uid,
                    machine
                ).await
            )
        }

        let machine = fsm::FSM::new();
        Self {
            map: Map::new("assets/tileset.png", "assets/tilemap.json").await,
            camera: Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), -screen_height())),
            player: Entity::new(
                -800.0, 1600.0,32.0,32.0,
                "assets/critters/wolf/wolf-all.png",
                wolf_animation,
                100.0,
                player_uid,
                machine
            ).await,
            enemies,
            time: get_time(),
            uid: uids
        }
    }

    pub fn update(&mut self){
        self.camera.target = vec2(
            self.player.rect.x + self.player.rect.w/2.0,
            self.player.rect.y + self.player.rect.h/2.0
        );

        self.player.update();
        self.player.keyboard_controller();
        let mut despawn_buffer = Vec::new();
        for enemy in &mut self.enemies {
            enemy.update();

            if self.player.aabb(enemy.rect){
                println!("{} {}: Encountered an enemy!", get_time(), self.player.health);
                self.player.health -= 5.0;
                enemy.health -= 5.0;
                
            }
            if enemy.health <= 0.0 {
                despawn_buffer.push(enemy.uid);
                
            }

        }

        for id in despawn_buffer{
            let index = &mut self.enemies.iter().position(|r| r.uid== id).unwrap();
            
            self.enemies.remove(*index);
        }

        if get_time() - self.time> 1.0 {
            for enemy in &mut self.enemies {
                enemy.ai_controller();
            }

            self.time = get_time();
        }
    }

    pub fn draw(&mut self) {
        set_camera(&self.camera);
        self.map.draw();
        self.player.draw(4.0);

        for enemy in &mut self.enemies {
            enemy.draw(1.0);
        }
    }
    pub fn uid_gen() -> u32 {
        let rng = rand::rand();

        return rng;

    }
}
