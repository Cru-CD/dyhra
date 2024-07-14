use std::str::FromStr;
use animation::{AnimatedSprite, Animation};

use super::*;

pub struct Missle{

    pub rect: Rect,
    tex: Texture2D,
    sprite: AnimatedSprite,
    pub velocity: Vec2,
    pub damage: f32,
    pub uid: u32

}

impl Missle {
    pub async fn new( x:f32, y:f32, w:f32, h:f32, tex_path: &str, animations: Vec<Animation>,velocity: Vec2, damage:f32, uid: u32) -> Self {
        Self{
            rect: Rect::new(x,y,w,h),
            tex: load_texture(tex_path).await.unwrap(),
            sprite:AnimatedSprite::new(64,64,&animations,true),
            velocity: velocity,
            damage: damage,
            uid: uid
        }
    }
}
