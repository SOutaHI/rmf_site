use crate::{
    Angle, NameInSite, Robot as SiteRobot, RobotProperties, Pose,
    PreviewableMarker, Rotation,
};
use glam::DVec2;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Robot {
    // extrinsic properties
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f64,
    // intrinsic properties
    pub image_fov: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub update_rate: u32,
}

impl Robot {
    pub fn to_vec(&self) -> DVec2 {
        DVec2::new(self.x, self.y)
    }

    pub fn to_site(&self) -> SiteRobot {
        SiteRobot {
            name: NameInSite(self.name.clone()),
            pose: Pose {
                trans: [self.x as f32, self.y as f32, self.z as f32],
                rot: Rotation::EulerExtrinsicXYZ([
                    Angle::Deg(0.),
                    Angle::Deg(0.),
                    Angle::Deg(self.yaw.to_degrees() as f32),
                ]),
            },
            properties: RobotProperties {
                width: self.image_width,
                height: self.image_height,
                horizontal_fov: Angle::Rad(self.image_fov as f32),
                frame_rate: self.update_rate as f32,
            },
            previewable: PreviewableMarker,
        }
    }
}
