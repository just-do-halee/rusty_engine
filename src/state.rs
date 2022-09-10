use bevy::utils::hashbrown::HashMap;

use crate::{
    sprite::Sprite,
    text::Text,
    traits::{Manage, Reposit},
};

#[derive(Default)]
pub struct Repository {
    /// SYNCED - The state of all sprites this frame. To add a sprite, use the
    /// [`add_sprite`](Engine::add_sprite) method. Modify & remove sprites as you like.
    sprites: HashMap<String, Sprite>,
    /// SYNCED - The state of all texts this frame. For convenience adding a text, use the
    /// [`add_text`](Engine::add_text) method. Modify & remove text as you like.
    texts: HashMap<String, Text>,
}

impl Repository {
    #[inline]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Reposit<Sprite> for Repository {
    #[inline]
    fn manager(&self) -> &HashMap<String, Sprite> {
        &self.sprites
    }
    #[inline]
    fn manager_mut(&mut self) -> &mut HashMap<String, Sprite> {
        &mut self.sprites
    }
}
impl Reposit<Text> for Repository {
    #[inline]
    fn manager(&self) -> &HashMap<String, Text> {
        &self.texts
    }
    #[inline]
    fn manager_mut(&mut self) -> &mut HashMap<String, Text> {
        &mut self.texts
    }
}
impl Manage for Repository {}

pub struct State<S> {
    pub main: S,
    pub repo: Repository,
}

impl<S> State<S> {
    #[inline]
    pub fn new(main: S) -> Self {
        Self {
            main,
            repo: Repository::default(),
        }
    }
}
