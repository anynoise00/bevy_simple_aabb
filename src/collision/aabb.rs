use bevy::prelude::{ Vec2, GlobalTransform };
use crate::{components::Rectangle, collision::{ Hit, Raycast, Ray }, utils::EPSILON };

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    extents: Vec2,
    position: Vec2,

    min: Vec2,
    max: Vec2,
}

impl Aabb {
    pub fn new(extents: Vec2, position: Vec2) -> Self {
        Self {
            extents,
            position,

            min: position - extents,
            max: position + extents,
        }
    }

    pub fn extents(&self) -> Vec2 {
        self.extents
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn min(&self) -> Vec2 {
        self.min
    }

    pub fn max(&self) -> Vec2 {
        self.max
    }

    pub fn from_rectangle(rectangle: Rectangle, transform: &GlobalTransform) -> Self {
        Self::new(rectangle.size() / 2.0, Vec2::new(transform.translation.x, transform.translation.y))
    }

    pub fn from_ray(ray: &Raycast, transform: &GlobalTransform) -> Self {
        let half_dir = ray.direction / 2.0;
        Self::new(
            // temporary solution to ray not detecting when it checks a gap
            // TODO remove this when QuadTree is implemented
            half_dir.abs().max(Vec2::splat(EPSILON)),
            Vec2::new(
                transform.translation.x, transform.translation.y
            ) + half_dir + ray.offset,
        )
    }

    pub fn minkowski_diff(&self, other: &Aabb) -> Self {
        Aabb::new(
            self.extents + other.extents,
            self.position - other.position,
        )
    }

    pub fn is_overlapping(&self, other: &Aabb) -> bool {
        self.min.x < other.max.x
        && self.max.x > other.min.x
        && self.min.y < other.max.y
        && self.max.y > other.min.y
    }

    pub fn get_broad(&self, motion: Vec2) -> Self {
        let half_motion = motion / 2.0;

        Aabb::new(
            self.extents + half_motion.abs(),
            self.position + half_motion,
        )
    }

    pub fn sweep_test(&self, other: &Aabb, motion: Vec2) -> Option<Hit> {
        if motion == Vec2::ZERO { return None };

        let minkowski = other.minkowski_diff(self);
        let ray = Ray::new(motion, Vec2::ZERO);

        ray.intersect_aabb(minkowski)
    }
}

#[cfg(test)]
mod tests {
    use crate::collision::Aabb;
    use bevy::math::Vec2;
    
    #[test]
    fn test_overlap() {
        let a = Aabb::new(Vec2::splat(3.0), Vec2::new(1.0, 2.0));
        let b = Aabb::new(Vec2::new(2.0, 4.0), Vec2::ZERO);
        
        assert!(a.is_overlapping(&b));
    }
}