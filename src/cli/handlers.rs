use crate::templates::{
    ErrorDotSvelte, LayoutDotServer, LayoutDotSvelte, LayoutDotTs, PageDotServer, PageDotSvelte,
    PageDotTs, ServerDotTs,
};
use askama::Template;
use cli::create_spinner;
use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::io::Write;

pub fn handle_create_command(path: String) -> () {
    let multiselected = &[
        "+page.svelte",
        "+page.ts",
        "+page.server.ts",
        "+layout.svelte",
        "+layout.ts",
        "+layout.server.ts",
        "+server.ts",
        "+error.svelte",
    ];
    let defaults = &[true, false, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your types")
        .items(&multiselected[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("No types selected. Aborting...");
        return;
    }

    let mut spinner = create_spinner("Creating Route...".into());
    let directory = format!("src/routes/{}", path);
    let directory_path = std::path::Path::new(&directory);

    if !directory_path.exists() {
        std::fs::create_dir_all(directory_path).expect("failed to create directory");
    }

    for selection in selections {
        let target_path = directory_path.join(&multiselected[selection]);

        let template = match multiselected[selection] {
            "+page.svelte" => PageDotSvelte {}.render().unwrap(),
            "+page.ts" => PageDotTs {}.render().unwrap(),
            "+page.server.ts" => PageDotServer {}.render().unwrap(),
            "+layout.svelte" => LayoutDotSvelte {}.render().unwrap(),
            "+layout.ts" => LayoutDotTs {}.render().unwrap(),
            "+layout.server.ts" => LayoutDotServer {}.render().unwrap(),
            "+server.ts" => ServerDotTs {}.render().unwrap(),
            "+error.svelte" => ErrorDotSvelte {}.render().unwrap(),
            _ => panic!("unknown template"),
        };

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(target_path)
            .expect("failed to create file");

        file.write_all(template.as_bytes())
            .expect("failed to write file");

        spinner.stop_and_persist("✔", "Route created successfully".into());
    }
}

pub fn handle_delete_route(path: String) -> () {
    let directory = format!("src/routes/{}", path);
    let directory_path = std::path::Path::new(&directory);

    if !directory_path.exists() {
        println!("Route does not exist. Aborting...");
        return;
    }
    let mut spinner = create_spinner("Deleting Route...".into());
    std::fs::remove_dir_all(directory_path).expect("failed to delete directory");
    spinner.stop_and_persist("✔", "Route deleted successfully".into());
}

pub fn handle_count_routes() -> () {
    let mut count = 0;
    let mut spinner = create_spinner("Counting Routes...".into());
    let directory_path = std::path::Path::new("src/routes");

    if !directory_path.exists() {
        println!("No routes directory found. Aborting...");
        return;
    }

    for entry in walkdir::WalkDir::new(directory_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str() {
                match file_name {
                    "+page.svelte" | "+server.ts" => count += 1,
                    _ => {}
                }
            }
        }
    }
    spinner.stop_and_persist("✔", format!("{} routes found", count).into());
}
