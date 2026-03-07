use std::sync::LazyLock;

use crate::models::person::Person;

fn load_people_from_yaml() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let yaml_content = std::fs::read_to_string("assets/bots.yaml")?;
    let people = serde_yaml::from_str::<Vec<Person>>(&yaml_content)?;
    Ok(people)
}

pub static BOTS: LazyLock<Vec<Person>> = LazyLock::new(|| {
    // Load people from YAML on server startup
    match load_people_from_yaml() {
        Ok(p) => {
            println!("Loaded {} people from bots.yaml", p.len());
            p
        }
        Err(e) => {
            eprintln!("Failed to load bots.yaml: {}", e);
            vec![]
        }
    }
});
