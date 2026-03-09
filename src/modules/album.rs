use std::io;

use crate::modules::create_album;
use crate::utils;

pub async fn get_or_create_album(token: &str) -> String {
    println!("Add to album ? y/n");

    let mut upload_to_album = String::new();
    io::stdin().read_line(&mut upload_to_album).unwrap();

    let mut album_id = String::new();

    if upload_to_album.trim().eq_ignore_ascii_case("y") || upload_to_album.trim().is_empty() {
        println!("create a new album ? y/n");

        let mut create_album_input = String::new();
        io::stdin().read_line(&mut create_album_input).unwrap();

        if create_album_input.trim().eq_ignore_ascii_case("y")
            || create_album_input.trim().is_empty()
        {
            album_id = create_album::create_album_fn(token).await;
        } else {
            match utils::api::get_albums(token).await {
                Ok(data) => {
                    let labels: Vec<String> = data
                        .albums
                        .iter()
                        .map(|album| format!("{} (id: {})", album.name, album.id))
                        .collect();
                    if !labels.is_empty() {
                        let selection = dialoguer::Select::new()
                            .with_prompt("Select an Album")
                            .items(&labels)
                            .default(0)
                            .interact()
                            .unwrap();
                        album_id = data.albums[selection].id.to_string();
                    } else {
                        eprintln!("No albums found.");
                    }
                }
                Err(err) => eprintln!("Error getting albums: {}", err),
            };
        }
    }
    album_id
}
