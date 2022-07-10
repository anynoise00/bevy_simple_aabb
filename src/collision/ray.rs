use bevy::prelude::{ GlobalTransform, Vec2 };
use crate::{ collision::Aabb, components::Raycast };

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub direction: Vec2,
    pub position: Vec2,
}

impl Ray {
    pub fn new(direction: Vec2, position: Vec2) -> Self {
        Self {
            direction,
            position,
        }
    }

    pub fn from_ray(raycast: &Raycast, transform: &GlobalTransform) -> Self {
        Self::new(
            raycast.direction,
            Vec2::new(transform.translation.x, transform.translation.y) + raycast.offset,
        )
    }

    // https://noonat.github.io/intersect/#axis-aligned-bounding-boxes
    pub fn intersect_aabb(self, other: Aabb) -> Option<Hit> {
        let mut hit = Hit::default();

        let inv_dir = 1.0 / self.direction;
        let sign = inv_dir.signum();

        let t_near = (other.position() - self.position - (sign * other.extents())) * inv_dir;
        let t_far = (other.position() - self.position + (sign * other.extents())) * inv_dir;

        if t_near.x > t_far.y || t_near.y > t_far.x {
            return None;
        }

        let t_hit_near = (t_near.x).max(t_near.y);
        let t_hit_far = (t_far.x).min(t_far.y);

        if t_hit_near >= 1.0 || t_hit_far <= 0.0 {
            return None;
        }

        hit.time = t_hit_near.max(0.0);
        
        if t_near.x > t_near.y {
            hit.normal.x = -sign.x;
        } else if t_near.x < t_near.y {
            hit.normal.y = -sign.y;
        }

        Some(hit)
    }
}

#[derive(Copy, Clone, Default)]
pub struct Hit {
    pub time: f32,
    pub normal: Vec2,
}

#[cfg(test)]
mod tests {
    use crate::collision::{ Aabb, Ray };
    use bevy::math::Vec2;

    #[test]
    fn test_time() {
        let a = Aabb::new(Vec2::splat(2.0), Vec2::new(-3.0, 0.0));
        let ray = Ray::new(Vec2::new(-2.0, 0.0), Vec2::ZERO);

        let hit = ray.intersect_aabb(a).unwrap();

        assert_eq!(hit.time, 0.5);
    }

    #[test]
    fn test_normal() {
        let a = Aabb::new(Vec2::splat(2.0), Vec2::new(-3.0, 0.0));
        let ray = Ray::new(Vec2::new(-2.0, 0.0), Vec2::ZERO);

        let hit = ray.intersect_aabb(a).unwrap();
        assert_eq!(hit.normal, Vec2::new(1.0, 0.0));
    }
}