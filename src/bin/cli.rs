use dnd_character_generator::{CharacterGenerator, save_characters_to_file};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    println!("🎲 GLOG v2 Character Generator (CLI)");
    println!("================================");
    
    // Initialize the character generator
    let generator = CharacterGenerator::new("config.toml")?;
    
    // Get user input
    let level = get_level_input()?;
    let count = get_count_input()?;
    
    // Generate characters
    let characters = generator.generate_characters(level, count)?;
    
    // Display characters
    for (i, character) in characters.iter().enumerate() {
        println!("\nCharacter {}:", i + 1);
        println!("Level: {}", character.level);
        println!("Race: {}", character.race);
        println!("Class: {}", character.class);
        println!("Ability Scores:");
        println!("  Strength: {}", character.ability_scores.strength);
        println!("  Dexterity: {}", character.ability_scores.dexterity);
        println!("  Constitution: {}", character.ability_scores.constitution);
        println!("  Intelligence: {}", character.ability_scores.intelligence);
        println!("  Wisdom: {}", character.ability_scores.wisdom);
        println!("  Charisma: {}", character.ability_scores.charisma);
    }
    
    // Save to file
    let filename = save_characters_to_file(&characters, level, count)?;
    println!("\n{} character(s) generated successfully!", count);
    println!("Characters saved to: {}", filename);
    
    Ok(())
}

fn get_input(prompt: &str) -> Result<String, io::Error> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_string())
}

fn get_level_input() -> anyhow::Result<u32> {
    loop {
        let input = get_input("Enter character level (1-10): ")?;
        
        match input.parse::<u32>() {
            Ok(level) if level >= 1 && level <= 20 => return Ok(level),
            Ok(_) => println!("Level must be between 1 and 20."),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn get_count_input() -> anyhow::Result<u32> {
    loop {
        let input = get_input("Enter number of characters to generate: ")?;
        
        match input.parse::<u32>() {
            Ok(count) if count >= 1 => return Ok(count),
            Ok(_) => println!("Must generate at least 1 character."),
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
