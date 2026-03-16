use std::sync::LazyLock;

use crate::models::person::Person;

fn load_people_from_yaml() -> Result<Vec<Person>, Box<dyn std::error::Error>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let mut people = serde_yaml::from_str::<Vec<Person>>(&yaml_content)?;

    // Keep bot IDs out of YAML and assign deterministic negatives at load time.
    for (idx, person) in people.iter_mut().enumerate() {
        person.id = Some(-((idx as i32) + 1));
    }

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

#[test]
fn no_broken_bot_img_urls() {
    use crate::models::person::Pic;
    use std::time::Duration;

    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; cringe-bot/1.0)")
        .timeout(Duration::from_secs(15))
        .build()
        .unwrap();

    let pps = load_people_from_yaml().unwrap();

    for pp in pps {
        for (idx, img) in pp.pictures.iter().enumerate() {
            let src = match img {
                Pic::Url(src) => src,
                Pic::Advanced { url, .. } => url,
            };

            // check only 3rd party links
            if src.starts_with("https://") {
                let response = client.get(src).send().unwrap_or_else(|err| {
                    panic!(
                        "\n\t{}'s image #{idx} request failed: '{src}'\n\tError: {err}\n",
                        pp.name
                    )
                });
                assert!(
                    response.status().is_success(),
                    "\n\t{}'s image #{idx} is not success: '{}'\n\tStatus: {}\n",
                    pp.name,
                    src,
                    response.status()
                );
            }
        }
    }
}
