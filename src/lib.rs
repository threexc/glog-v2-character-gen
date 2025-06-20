use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub races: Vec<String>,
    pub classes: Vec<String>,
    pub wizard_archetypes: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Character {
    pub level: u32,
    pub class: String,
    pub race: String,
    pub ability_scores: AbilityScores,
}

#[derive(Debug, Serialize, Clone)]
pub struct AbilityScores {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

pub struct CharacterGenerator {
    config: Config,
}

impl CharacterGenerator {
    pub fn new(config_path: &str) -> anyhow::Result<Self> {
        let config = Self::load_config(config_path)?;
        
        Ok(Self { config })
    }
    
    pub fn from_config(config: Config) -> Self {
        Self { config }
    }
    
    pub fn generate_character(&self, level: u32) -> anyhow::Result<Character> {
        if level < 1 || level > 10 {
            return Err(anyhow::anyhow!("Level must be between 1 and 10"));
        }
        
        let mut rng = rand::thread_rng();
        
        // Generate random race and class
        let race = self.config.races[rng.gen_range(0..self.config.races.len())].clone();
        let mut class = self.config.classes[rng.gen_range(0..self.config.classes.len())].clone();
        
        // If wizard is selected, add an archetype
        if class == "Wizard" {
            let archetype = &self.config.wizard_archetypes[rng.gen_range(0..self.config.wizard_archetypes.len())];
            class = format!("Wizard ({})", archetype);
        }
        
        // Generate ability scores
        let ability_scores = Self::generate_ability_scores(&mut rng);
        
        Ok(Character {
            level,
            class,
            race,
            ability_scores,
        })
    }
    
    pub fn generate_characters(&self, level: u32, count: u32) -> anyhow::Result<Vec<Character>> {
        if count < 1 {
            return Err(anyhow::anyhow!("Must generate at least 1 character"));
        }
        
        if count > 100 {
            return Err(anyhow::anyhow!("Cannot generate more than 100 characters at once"));
        }
        
        let mut characters = Vec::new();
        
        for _ in 0..count {
            characters.push(self.generate_character(level)?);
        }
        
        Ok(characters)
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    fn load_config(filename: &str) -> anyhow::Result<Config> {
        let content = fs::read_to_string(filename)
            .map_err(|_| anyhow::anyhow!("Could not read config file: {}", filename))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;
        
        Self::validate_config(&config)?;
        
        Ok(config)
    }
    
    fn validate_config(config: &Config) -> anyhow::Result<()> {
        if config.races.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one race"));
        }
        
        if config.classes.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one class"));
        }
        
        if config.wizard_archetypes.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one wizard archetype"));
        }
        
        Ok(())
    }
    
    fn generate_ability_scores(rng: &mut impl Rng) -> AbilityScores {
        AbilityScores {
            strength: Self::roll_ability_score(rng),
            dexterity: Self::roll_ability_score(rng),
            constitution: Self::roll_ability_score(rng),
            intelligence: Self::roll_ability_score(rng),
            wisdom: Self::roll_ability_score(rng),
            charisma: Self::roll_ability_score(rng),
        }
    }
    
    //fn roll_ability_score(rng: &mut impl Rng) -> u8 {
    //  // Roll 4d6, drop the lowest
    //  let mut rolls: Vec<u8> = (0..4).map(|_| rng.gen_range(1..=6)).collect();
    //  rolls.sort_unstable();
    //rolls[1..].iter().sum() // Skip the lowest roll
    //}
    fn roll_ability_score(rng: &mut impl Rng) -> u8 {
        // Roll 3d6
        let mut rolls: Vec<u8> = (0..3).map(|_| rng.gen_range(1..=6)).collect();
        rolls.sort_unstable();
        rolls.iter().sum()
    }
}

// Utility functions for file operations
pub fn save_characters_to_file(characters: &[Character], level: u32, count: u32) -> anyhow::Result<String> {
    let filename = format!("characters_level_{}_count_{}.toml", level, count);
    
    // Create a wrapper struct to hold all characters
    #[derive(Serialize)]
    struct CharacterCollection<'a> {
        characters: &'a [Character],
    }
    
    let collection = CharacterCollection { characters };
    let content = toml::to_string_pretty(&collection)?;
    fs::write(&filename, content)?;
    
    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> Config {
        Config {
            races: vec!["Human".to_string(), "Elf".to_string()],
            classes: vec!["Fighter".to_string(), "Wizard".to_string()],
            wizard_archetypes: vec!["Necromancer".to_string(), "Pyromancer".to_string()],
        }
    }
    
    #[test]
    fn test_character_generation() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        let character = generator.generate_character(5).unwrap();
        
        assert_eq!(character.level, 5);
        assert!(!character.race.is_empty());
        assert!(!character.class.is_empty());
        assert!(character.ability_scores.strength >= 3 && character.ability_scores.strength <= 18);
    }
    
    #[test]
    fn test_multiple_character_generation() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        let characters = generator.generate_characters(3, 5).unwrap();
        
        assert_eq!(characters.len(), 5);
        assert!(characters.iter().all(|c| c.level == 3));
    }
    
    #[test]
    fn test_invalid_level() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        assert!(generator.generate_character(0).is_err());
        assert!(generator.generate_character(21).is_err());
    }
    
    #[test]
    fn test_wizard_archetype() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        // Generate many characters to eventually get a wizard
        for _ in 0..100 {
            let character = generator.generate_character(1).unwrap();
            if character.class.starts_with("Wizard") {
                assert!(character.class.contains("("));
                assert!(character.class.contains(")"));
                break;
            }
        }
    }
}
