// Only implementing UP, no DOWN direction
pub async fn migrate(pool: &sqlx::PgPool) -> Result<(), anyhow::Error> {
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS migrations(
            file TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    let already_done = sqlx::query_scalar::<_, String>("SELECT file FROM migrations;")
        .fetch_all(pool)
        .await?;

    // ensure migrations table exists
    // load migration files and execute them in filename order
    let migrations = read_migrations(already_done)?;
    for (file, sql) in migrations {
        let mut tx = pool.begin().await?; // start transaction

        // execute migration SQL inside transaction
        sqlx::query(&sql).execute(&mut *tx).await?;

        // record applied migration inside same transaction
        sqlx::query("INSERT INTO migrations (file) VALUES ($1);")
            .bind(&file)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?; // commit if all succeeded
    }

    Ok(())
}

/// Read all files from `migrations/` directory, sort by filename, and return their
/// contents as a Vec<String>.
fn read_migrations(done: Vec<String>) -> Result<Vec<(String, String)>, anyhow::Error> {
    use std::path::PathBuf;

    let rd = match std::fs::read_dir("migrations") {
        Ok(rd) => rd,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e.into()),
    };

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for entry in rd {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path
                .file_name()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string())
            {
                if !done.contains(&name) {
                    files.push((name, path));
                }
            }
        }
    }

    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut contents = Vec::with_capacity(files.len());
    for (name, path) in files {
        let s = std::fs::read_to_string(path)?;
        contents.push((name, s));
    }

    Ok(contents)
}
