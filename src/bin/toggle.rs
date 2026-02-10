#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use wks::prelude::*;

#[derive(Parser)]
struct Tuna {
    namespace: String,
    items: Vec<String>,
}

type CacheType = HashMap<String, Option<String>>;

static JSON_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::cache_dir()
        .unwrap()
        .join("toggle.rs.jsonc")
});

fn main() -> Result<()> {
    let tuna = Tuna::parse();
    let mut json_file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .write(true)
        .read(true)
        .open(&*JSON_PATH)
        .unwrap();
    let mut buf = String::new();
    json_file
        .read_to_string(&mut buf)
        .unwrap();
    let mut cache_model: CacheType = serde_json::from_str(&buf).unwrap_or_default();
    let entry = cache_model
        .entry(tuna.namespace)
        .or_default();
    *entry = Some(
        entry
            .as_ref()
            .and_then(|value| {
                #[allow(clippy::skip_while_next)]
                #[allow(clippy::iter_skip_next)]
                tuna.items
                    .iter()
                    .skip_while(|item| item != &value)
                    .skip(1)
                    .next()
                    .inspect(|next_value| println!("{}", next_value))
            })
            .unwrap_or_else(|| {
                tuna.items
                    .first()
                    .inspect(|next_value| println!("{}", next_value))
                    .unwrap()
            })
            .to_owned(),
    );
    json_file.set_len(0).unwrap();
    json_file.rewind().unwrap();
    json_file
        .write_all(
            serde_json::to_string_pretty(&cache_model)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();
    Ok(())
}
