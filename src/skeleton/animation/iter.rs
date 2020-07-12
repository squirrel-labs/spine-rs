use super::skin::SkinAnimation;
use super::sprite::Sprites;

/// Iterator over a constant period
#[derive(Clone)]
pub struct AnimationIter<'a> {
    pub skin_animation: &'a SkinAnimation<'a>,
    pub time: f32,
    pub delta: f32,
}

impl<'a> Iterator for AnimationIter<'a> {
    type Item = Sprites<'a>;
    fn next(&mut self) -> Option<Sprites<'a>> {
        let result = self.skin_animation.interpolate(self.time);
        self.time += self.delta;
        result
    }
}
