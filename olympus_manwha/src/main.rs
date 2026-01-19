use reqwest::blocking::{Response, get};
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::copy;
use std::{error::Error, fmt::Formatter};
fn main() -> Result<(), Box<dyn Error>> {
    println!("Olympus empieza...");
    let link = "https://olympusbiblioteca.com";
    let mut link_agregado = "";
    //NOTE: aqui se "llama" al link para "ver" la pagina
    let fuente_raw = get(link)?.text()?;

    //NOTE: aqui se lo formatea a HTML para que se pueda extraer ciertas partes o "bloques"
    let html_fuente = Html::parse_document(&fuente_raw);

    //NOTE: aqui son los selectores, en base a la estructura del html
    let seleccion_manwha =
        Selector::parse("a.p-4,relative,bg-gray-800,rounded-md,overflow-hidden").unwrap();
    // Selector::parse(".flex,flex-col,gap-2,snap-start,shrink-0").unwrap();
    let seleccion_texto = Selector::parse("h3.font-medium,text-lg,h-13,line-clamp-2").unwrap();
    // dbg!(&html_fuente);

    //NOTE: aqui es la iteracion de bloques flitradon en base a una seleccion
    for bloque_manwha in html_fuente.select(&seleccion_manwha) {
        // dbg!(bloque_manwha.clone());

        let titulo = bloque_manwha
            .select(&seleccion_texto)
            //NOTE: seleccion del texto dentro del bloque
            .next()
            .map(|x| {
                // dbg!(x);
                x.text().collect::<String>()
            })
            .unwrap_or_default();

        //NOTE: extraccion de link por medio de attributo de bloque
        if titulo == "Loco Frontera".to_string() {
            link_agregado = bloque_manwha.attr("href").unwrap_or_default()
        }

        println!("{}: {}", titulo, link_agregado);
    }
    frontera(link, link_agregado);

    Ok(())
}

fn frontera(link_base: &str, agregado_manwha: &str) -> Result<(), Box<dyn Error>> {
    println!("Frontera Empieza BVVV");
    let link_manwha = link_base.to_string() + agregado_manwha;
    let fuente_sin_formato = get(link_manwha.as_str())?.text()?;

    let codigo_fuente = Html::parse_document(&fuente_sin_formato);

    let seleccion_cap1 =
        Selector::parse("a.text-amber-300,w-full,flex-between,p-3,rounded sf-ripple-container")
            .unwrap();

    let agregado_url = codigo_fuente
        .select(&seleccion_cap1)
        .map(|bloque| bloque.attr("href"))
        .find(|&agregado| agregado.is_some())
        .unwrap()
        .unwrap(); //NOTE: ??
    dbg!(agregado_url);

    // let imagen = write("hola.webp", extraer_img(link_base, agregado_url)?);

    let mut imagen = File::create("imagen.jpg")?;
    let mut response = extraer_img(link_base, agregado_url)?;
    // copy(&mut response, &mut imagen)?; //NOTE: puede servir
    fs::write("hola.webp", response.bytes()?);
    // extraer_img(link_base, agregado_url)?;
    Ok(())
}
fn extraer_img(link_base: &str, agregado_cap: &str) -> Result<Response, Box<dyn Error>> {
    let link_cap = link_base.to_string() + agregado_cap;
    let respuesta = get(link_cap.as_str())?.text()?;
    let codigo_fuente = Html::parse_document(&respuesta);
    let seleccion_manwha =
        Selector::parse(".flex-col,rounded-xl,overflow-hidden,shadow-xl").unwrap();
    let seleccion_img = Selector::parse("img.object-cover,rounded-inherit,w-full,h-full").unwrap();
    // let seleccion_titulo = Selector::parse("div.flex-between,w-full").unwrap();
    let seleccion_titulo = Selector::parse("header.h-18,bg-gray-800,relative,z-20").unwrap();
    let seleccion_cap_num = Selector::parse(".flex-center").unwrap();

    let seccion_titulo = codigo_fuente.select(&seleccion_titulo).next().unwrap();
    dbg!(&seccion_titulo.html());

    let nombre_carpeta = seccion_titulo
        .select(&seleccion_cap_num)
        .next()
        .map(|bloque| {
            dbg!(&bloque);
            bloque.text().collect::<String>()
        })
        .unwrap_or_default();
    dbg!(&nombre_carpeta);

    let manwha_completo = codigo_fuente.select(&seleccion_manwha).next().unwrap();
    // dbg!(&manwha_completo);

    for link in manwha_completo
        .select(&seleccion_img)
        .filter_map(|bloque| bloque.attr("src"))
    {
        let nombre_imagen = &link[(74 - 9)..];
        // dbg!(nombre_imagen);
    }

    let link_img = codigo_fuente
        .select(&seleccion_img)
        .filter_map(|bloque| bloque.attr("src"))
        .nth(4)
        .unwrap();

    dbg!(&link_img);
    let fuente_img = get(link_img)?/* .text()? */;
    // dbg!(&fuente_img);
    Ok(fuente_img)
}
