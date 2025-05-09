use crate::*;

#[derive(Debug, Clone)]
pub struct Snake {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
}

impl Snake {
    pub fn new(position: Point2<f32>, rotation: Rotation2<f32>, speed: f32) -> Self {
        Self {
            position,
            rotation,
            speed,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

}