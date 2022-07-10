use bevy::prelude::{ Vec2, Transform, GlobalTransform };
use crate::{components::Rectangle, collision::{ Hit, Raycast, Ray }, utils::EPSILON };

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub extents: Vec2,
    pub position: Vec2,
}

impl Aabb {
    pub fn new(extents: Vec2, position: Vec2) -> Self {
        Self {
            extents: extents,
            position,
        }
    }

    pub fn from_rectangle(rectangle: Rectangle, transform: &Transform) -> Self {
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

    pub fn minkowski_diff(mut self, other: Aabb) -> Self {
        self.extents += other.extents;
        self.position -= other.position;

        self
    }

    pub fn min_max(&self) -> (Vec2, Vec2) {
        (self.position - self.extents, self.position + self.extents)
    }

    pub fn is_overlapping(&self, other: Aabb) -> bool {
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

    pub fn get_broad(mut self, motion: Vec2) -> Self {
        let half_motion = motion / 2.0;
        self.extents += half_motion.abs();
        self.position += half_motion;
        self
    }

    pub fn get_hit_info(&self, other: Aabb, motion: Vec2) -> (f32, Vec2) {
        let (a_min, a_max) = self.min_max();
        let (b_min, b_max) = other.min_max();

        let mut normal = Vec2::ZERO;

        let mut x_entry = f32::NEG_INFINITY;
        let mut x_exit = f32::INFINITY;
        if motion.x > 0.0 {
            x_entry = (b_min.x - a_max.x) / motion.x;
            x_exit = (b_max.x - a_min.x) / motion.x;
            
            normal.x = -1.0;
        } else if motion.x < 0.0 {
            x_entry = (b_max.x - a_min.x) / motion.x;
            x_exit = (b_min.x - a_max.x) / motion.x;
            
            normal.x = 1.0;
        }

        let mut y_entry = f32::NEG_INFINITY;
        let mut y_exit = f32::INFINITY;
        if motion.y > 0.0 {
            y_entry = (b_min.y - a_max.y) / motion.y;
            y_exit = (b_max.y - a_min.y) / motion.y;
            
            normal.y = -1.0;
        } else if motion.y < 0.0 {
            y_entry = (b_max.y - a_min.y) / motion.y;
            y_exit = (b_min.y - a_max.y) / motion.y;
            
            normal.y = 1.0;
        }


        let mut entry_time = 1.0;
        let exit_time = x_exit.min(y_exit);
        if x_entry >= y_entry {
            entry_time = x_entry;
            normal.y = 0.0;
        } else if x_entry < y_entry {
            entry_time = y_entry;
            normal.x = 0.0;
        }

        if entry_time > exit_time || entry_time > 1.0 {
            (1.0, Vec2::ZERO)
        } else {
            (entry_time, normal)
        }
    }

    pub fn sweep_test(self, other: Aabb, motion: Vec2) -> Option<Hit> {
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
        
        assert!(a.is_overlapping(b));
        assert_eq!(a.get_overlap(b), Vec2::new(4.0, 0.0));
    }

    #[test]
    fn test_hit() {
        let a = Aabb::new(Vec2::splat(2.0), Vec2::new(-3.0, 0.0));
        let b = Aabb::new(Vec2::splat(2.0), Vec2::new(3.0, 0.0));

        let hit_info = a.get_hit_info(b, Vec2::new(4.0, 0.0));

        assert_eq!(hit_info, (0.5, Vec2::new(-1.0, 0.0)));
    }
}