use crate::*;

#[derive(Debug, Clone)]
pub struct Apple {
    pub(crate) position: Point2<f32>,
}

impl Apple {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    #[test]
    fn random() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let apple = Apple::random(&mut rng);
        
        assert_relative_eq!(apple.position, Point2::new(0.1872406, 0.8369197));
    }

    #[test]
    fn position() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let apple = Apple::random(&mut rng);
     

        assert_relative_eq!(apple.position(), Point2::new(0.1872406, 0.8369197));
    }
}