/*--[[
© ClassKsune, © CHC - Creative Hill College

]]*/


use serde_json::Value;
use std::error::Error as StdError;
use reqwest;
use open;

// Main funkce - Koukne jestli byla nějaká chyba v _animal_get()_ a ukončí
// Main func - If error in _animal_get()_ ends program
fn main() -> Result<(), Box<dyn StdError>> {
    if let Err(e) = animal_get() {
        eprintln!("!!ERROR!!: {}", e);
    }

    Ok(())
}

// Vezme pomocí GET odkaz na random obrázek kočky z some-random-api.com
// Uses GET to get url for cat image from some-random-api.com
fn animal_get() -> Result<(), Box<dyn StdError>> {
    let client = reqwest::blocking::Client::new();
    let res = client
        .get("https://some-random-api.com/img/cat")
        .send()?
        .json::<Value>()?;

    openBrowser(&res)?;

    Ok(())
}

// Čistě otevře okno - v jiné fuknci kvůli čitelnosti
// Opens new browser window/tab - Could be in _animal_get()_ but now it's PRETTY :)
fn openBrowser(in_res: &Value) -> Result<(), Box<dyn StdError>> {
    if let Some(link) = in_res.get("link").and_then(|v| v.as_str()) {
        println!("Link for image is: {}", link);
        open::that(link)?;

        Ok(())
    } else {
        Err("No image, Exiting now...".into())
    }
}
