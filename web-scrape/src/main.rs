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
        println!("== {:#?} ==\n", bloque);
        let frase = bloque
            .select(&seleccion_texto)
            .next()
            .map(|palabras| {
                println!("> {:?} <\n", palabras.text().collect::<String>());
                palabras.text().collect::<Vec<_>>().join("")
            })
            .unwrap_or_default();
        println!("\n {}", frase);
    }
    Ok(())
}
