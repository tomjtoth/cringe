use crate::models::person::Person;

pub async fn get_db() -> sqlx::PgPool {
    let dioxus::prelude::dioxus_fullstack::extract::Extension(pool) =
        dioxus::prelude::dioxus_fullstack::FullstackContext::extract::<
            dioxus::prelude::dioxus_fullstack::extract::Extension<sqlx::PgPool>,
            _,
        >()
        .await
        .expect("PgPool extension is missing from server context");

    pool
}

fn parse_yaml() -> anyhow::Result<Vec<Person>> {
    let yaml_content = std::fs::read_to_string("public/bots.yaml")?;
    let mut bots = serde_yaml::from_str::<Vec<Person>>(&yaml_content)?;

    // assign negative IDs to bots
    for (idx, bot) in bots.iter_mut().enumerate() {
        bot.id = Some(-((idx as i32) + 1));
    }

    Ok(bots)
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

    let pps = parse_yaml().unwrap();

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
