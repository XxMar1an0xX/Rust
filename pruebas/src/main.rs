use std::fs::File;
use std::io::copy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://dashboard.olympusbiblioteca.com/storage/comics/463/39935/1_01.webp";
    let mut response = reqwest::blocking::get(url)?;
    let mut file = File::create("imagen.webp")?;

    copy(&mut response, &mut file)?;
    println!("Imagen descargada correctamente");

    Ok(())
}
