use bevy::prelude::{ Component, Vec2 };

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Rectangle {
    size: Vec2,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            size: Vec2::ZERO,
        }
    }

    pub fn with_size(mut self, size: Vec2) -> Self {
        self.size = size.max(-size);
        self
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }
}


#[cfg(test)]
mod tests {
    use crate::components::Rectangle;
    use bevy::math::Vec2;

    #[test]
    fn test_size() {
        let a = Rectangle::new().with_size(Vec2::new(2.0, 3.0));
        assert_eq!(a.size, Vec2::new(2.0, 3.0));

        let b = Rectangle::new().with_size(Vec2::new(-4.0, -6.0));
        assert_eq!(b.size, Vec2::new(4.0, 6.0));
    }
}