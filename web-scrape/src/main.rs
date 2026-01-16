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
        // dbg!(bloque);
        let frase = bloque
            .select(&seleccion_texto)
            .next()
            .map(|palabras| {
                // dbg!(palabras.text().collect::<String>());
                palabras.text().collect::<Vec<_>>().join("")
            })
            .unwrap_or_default();
        // dbg!(frase);
    }
    let _ = olympus().await?;
    Ok(())
}
async fn olympus() -> Result<(), Box<dyn Error>> {
    println!("Olympus empieza...");
    let mut link = "https://olympusbiblioteca.com/";
    let mut link_agregado = "";
    let fuente_raw = reqwest::get(link).await?.text().await?;

    let html_fuente = Html::parse_document(&fuente_raw);

    let seleccion_manwha =
        Selector::parse("a.p-4,relative,bg-gray-800,rounded-md,overflow-hidden").unwrap();
    // Selector::parse(".flex,flex-col,gap-2,snap-start,shrink-0").unwrap();
    let seleccion_texto = Selector::parse("h3.font-medium,text-lg,h-13,line-clamp-2").unwrap();
    // dbg!(&html_fuente);

    for bloque_manwha in html_fuente.select(&seleccion_manwha) {
        // dbg!(bloque_manwha.clone());
        //NOTE: extraccion de link por medio de attributo de bloque
        link_agregado = bloque_manwha.attr("href").unwrap_or_default();

        let titulo = bloque_manwha
            .select(&seleccion_texto)
            //NOTE: seleccion del texto dentro del bloque
            .next()
            .map(|x| {
                // dbg!(x);
                x.text().collect::<String>()
            })
            .unwrap_or_default();
        println!("{}: {}", titulo, link_agregado);
    }

    Ok(())
}
