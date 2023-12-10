use rand::Rng;

pub struct PasswordGenerator {
    length: usize,
}

impl PasswordGenerator {
    pub fn new(length: usize) -> Self {
        PasswordGenerator { length }
    }

    pub fn generate(&self) -> String {
        let characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut rng = rand::thread_rng();
        let password: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..characters.len());
                characters.chars().nth(idx).unwrap()
            })
            .collect();
        password
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_generation() {
        let generator = PasswordGenerator::new(12);
        let password = generator.generate();
        assert_eq!(password.len(), 12);
    }
}
