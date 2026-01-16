use std::{error::Error, fmt::Formatter};

use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //NOTE: agarrar el sitio web
    let link = "https://quotes.toscrape.com/";
    println!("Buscando datos en {}", link);

    //NOTE: agarrando el html de la pagina
    let texto_fuente = reqwest::get(link).await?.text().await?;

    //NOTE: formatenadolo a html
    let html_fuente = Html::parse_document(&texto_fuente);
    // println!("==========\n html: \n {:?}", html_fuente);

    let seleccion_quote = Selector::parse(".quote").unwrap();
    let seleccion_texto = Selector::parse(".text").unwrap();

    for bloque in html_fuente.select(&seleccion_quote) {
        dbg!(bloque);
        let frase = bloque
            .select(&seleccion_texto)
            .next()
            .map(|palabras| {
                dbg!(palabras.text().collect::<String>());
                palabras.text().collect::<Vec<_>>().join("")
            })
            .unwrap_or_default();
        dbg!(frase);
    }
    let _ = olympus().await?;
    Ok(())
}
async fn olympus() -> Result<(), Box<dyn Error>> {
    println!("Olympus empieza...");
    let link = "https://olympusbiblioteca.com/";
    let fuente_raw = reqwest::get(link).await?.text().await?;

    let html_fuente = Html::parse_document(&fuente_raw);

    let seleccion_manwha =
        Selector::parse("a.p-4,relative,bg-gray-800,rounded-md,overflow-hidden").unwrap();
    // Selector::parse(".flex,flex-col,gap-2,snap-start,shrink-0").unwrap();
    let seleccion_titulo = Selector::parse(".font-medium text-lg h-13 line-clamp-2").unwrap();
    // dbg!(&html_fuente);
    println!("");

    for bloque in html_fuente.select(&seleccion_manwha) {
        dbg!(bloque);
    }

    Ok(())
}
