use bevy::prelude::{ Vec2, Transform };
use crate::components::Rectangle;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub extents: Vec2,
    pub position: Vec2,
}

impl Aabb {
    pub fn new(rectangle: &Rectangle, transform: &Transform) -> Self {
        let mut aabb = Self {
            extents: rectangle.size().max(Vec2::ZERO) / 2.0,
            position: Vec2::ZERO,
        };

        aabb.position.x = transform.translation.x;
        aabb.position.y = transform.translation.y;

        aabb
    }

    pub fn min_max(&self) -> (Vec2, Vec2) {
        (self.position - self.extents, self.position + self.extents)
    }

    pub fn collide_with(&self, other: Aabb) -> bool {
        let (a_min, a_max) = self.min_max();
        let (b_min, b_max) = other.min_max();

        a_min.x < b_max.x
            && a_max.x > b_min.x
            && a_min.y < b_max.y
            && a_max.y > b_min.y
    }

    pub fn get_overlap(&self, other: Aabb) -> Vec2 {
        let (a_min, a_max) = self.min_max();
        let (b_min, b_max) = other.min_max();

        let mut overlap = Vec2::ZERO;

        let ox1 = b_max.x - a_min.x;
        let ox2 = b_min.x - a_max.x;
        overlap.x = if ox1.abs() < ox2.abs() { ox1 } else { ox2 };

        let oy1 = b_max.y - a_min.y;
        let oy2 = b_min.y - a_max.y;
        overlap.y = if oy1.abs() < oy2.abs() { oy1 } else { oy2 };

        if overlap.y.abs() < overlap.x.abs() {
            overlap.x = 0.0;
        } else {
            overlap.y = 0.0;
        }

        overlap
    }

    pub fn expand(mut self, value: Vec2) -> Self {
        self.extents += value;
        self.extents = self.extents.max(Vec2::ZERO);

        self
    }
}

#[cfg(test)]
mod tests {
    use crate::aabb::Aabb;
    use bevy::math::Vec2;
    
    #[test]
    fn test_overlap() {
        let a = Aabb {
            extents: Vec2::splat(3.0),
            position: Vec2::new(1.0, 2.0),
        };
        
        let b = Aabb {
            extents: Vec2::new(2.0, 4.0),
            position: Vec2::ZERO,
        };
        
        assert!(a.collide_with(b));
        assert_eq!(a.get_overlap(b), Vec2::new(4.0, 0.0));
    }

    #[test]
    fn test_expand() {
        let a = Aabb {
            extents: Vec2::splat(3.0),
            position: Vec2::ZERO,
        };
        let b = a.expand(Vec2::splat(2.0));

        assert_eq!(a.extents, Vec2::splat(3.0));
        assert_eq!(b.extents, Vec2::splat(5.0));
    }
}