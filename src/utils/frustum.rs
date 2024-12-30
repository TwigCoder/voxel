use glam::{Mat4, Vec3, Vec4};

#[derive(Debug)]
pub struct Frustum {
    planes: [Vec4; 6],
}

impl Frustum {
    pub fn from_matrix(view_projection: Mat4) -> Self {
        let mut planes = [Vec4::ZERO; 6];

        planes[0] = view_projection.row(3) + view_projection.row(0);
        planes[1] = view_projection.row(3) - view_projection.row(0);
        planes[2] = view_projection.row(3) + view_projection.row(1);
        planes[3] = view_projection.row(3) - view_projection.row(1);
        planes[4] = view_projection.row(3) + view_projection.row(2);
        planes[5] = view_projection.row(3) - view_projection.row(2);

        for plane in &mut planes {
            let length = (Vec3::new(plane.x, plane.y, plane.z)).length();
            if length != 0.0 {
                *plane /= length;
            }
        }

        Self { planes }
    }

    pub fn is_box_visible(&self, min: Vec3, max: Vec3) -> bool {
        for plane in &self.planes {
            let p = Vec3::new(
                if plane.x > 0.0 { max.x } else { min.x },
                if plane.y > 0.0 { max.y } else { min.y },
                if plane.z > 0.0 { max.z } else { min.z },
            );
            let n = Vec3::new(
                if plane.x <= 0.0 { max.x } else { min.x },
                if plane.y <= 0.0 { max.y } else { min.y },
                if plane.z <= 0.0 { max.z } else { min.z },
            );

            if plane.dot(p.extend(1.0)) < 0.0 && plane.dot(n.extend(1.0)) < 0.0 {
                return false;
            }
        }
        true
    }
}
