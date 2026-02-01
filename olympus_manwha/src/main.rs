use dirs;
use reqwest::blocking::{Response, get};
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::copy;
use std::{error::Error, fmt::Formatter};
fn main() -> Result<(), Box<dyn Error>> {
    println!("Olympus empieza...");
    let link = "https://olympusbiblioteca.com";
    //NOTE: aqui se "llama" al link para "ver" la pagina
    let fuente_raw = get(link)?.text()?;

    //NOTE: aqui se lo formatea a HTML para que se pueda extraer ciertas partes o "bloques"
    let html_fuente = Html::parse_document(&fuente_raw);

    //NOTE: aqui son los selectores, en base a la estructura del html

    //NOTE: aqui es la iteracion de bloques flitradon en base a una seleccion

    let (nombre, agregado_manwha) = nombre_y_link(&html_fuente, "Loco Frontera")?;

    let agregado_cap = primer_cap(link.to_string(), agregado_manwha)?;

    let directorio = dirs::home_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string()
        + "/Descargas/"
        + nombre.as_str();
    // + "capitulo "
    // + &nombre_carpeta;
    fs::create_dir_all(&directorio);

    Ok(())
}

fn nombre_y_link(codigo_fuente: &Html, nombre: &str) -> Result<(String, String), Box<dyn Error>> {
    let mut agregado_manwha = "";
    let mut titulo_manwha = String::new();

    let seleccion_manwha =
        Selector::parse("a.p-4,relative,bg-gray-800,rounded-md,overflow-hidden").unwrap();
    let seleccion_texto = Selector::parse("h3.font-medium,text-lg,h-13,line-clamp-2").unwrap();

    for bloque_manwha in codigo_fuente.select(&seleccion_manwha) {
        titulo_manwha = bloque_manwha
            .select(&seleccion_texto)
            //NOTE: seleccion del texto dentro del bloque
            .next()
            .map(|x| x.text().collect::<String>())
            .unwrap_or_default();

        //NOTE: extraccion de link por medio de attributo de bloque
        if titulo_manwha == nombre.to_string() {
            agregado_manwha = bloque_manwha.attr("href").unwrap_or_default();
            // dbg!((&titulo_manwha, &agregado_manwha));
            break;
        }
    }

    return Ok((titulo_manwha, agregado_manwha.to_owned()));
}
fn primer_cap(link_base: String, agregado: String) -> Result<String, Box<dyn Error>> {
    let mut agregado_url = String::new();
    let codigo_fuente = extraer_codigo_fuente(link_base + agregado.as_str())?;

    let seleccion_cap1 =
        Selector::parse("a.text-amber-300,w-full,flex-between,p-3,rounded sf-ripple-container")
            .unwrap();

    agregado_url = codigo_fuente
        .select(&seleccion_cap1)
        .map(|bloque| bloque.attr("href"))
        .find(|&agregado| agregado.is_some())
        .unwrap()
        .unwrap()
        .to_string(); //NOTE: ??

    dbg!(&agregado_url);
    Ok(agregado_url)
}

fn extraer_codigo_fuente(link: String) -> Result<Html, Box<dyn Error>> {
    let mut codigo_fuente = Html::parse_fragment("");
    let pagina = get(link.as_str())?.text()?;
    codigo_fuente = Html::parse_fragment(&pagina);

    Ok(codigo_fuente)
}

fn frontera(link_base: &str, agregado_manwha: &str) -> Result<(), Box<dyn Error>> {
    println!("Frontera Empieza BVVV");
    let link_manwha = link_base.to_string() + agregado_manwha;
    let fuente_sin_formato = get(link_manwha.as_str())?.text()?;

    let codigo_fuente = Html::parse_document(&fuente_sin_formato);

    // dbg!(agregado_url);

    // let imagen = write("hola.webp", extraer_img(link_base, agregado_url)?);

    // let mut imagen = File::create("imagen.jpg")?;
    // let mut response = extraer_img(link_base, agregado_url)?;
    // copy(&mut response, &mut imagen)?; //NOTE: puede servir
    // fs::write("hola.webp", response.bytes()?);
    // extraer_img(link_base, agregado_url)?;
    Ok(())
}
fn extraer_img(link_base: &str, agregado_cap: &str) -> Result<Response, Box<dyn Error>> {
    let link_cap = link_base.to_string() + agregado_cap;
    let respuesta = get(link_cap.as_str())?.text()?;
    // let respuesta =
    //     get("https://olympusbiblioteca.com/capitulo/67184/comic-loco-frontera-20260119-081341644")?
    //         .text()?;

    let codigo_fuente = Html::parse_document(&respuesta);
    let seleccion_manwha =
        Selector::parse(".flex-col,rounded-xl,overflow-hidden,shadow-xl").unwrap();
    let seleccion_img = Selector::parse("img.object-cover,rounded-inherit,w-full,h-full").unwrap();
    // let seleccion_titulo = Selector::parse("div.flex-between,w-full").unwrap();
    let seleccion_titulo = Selector::parse("header.h-18,bg-gray-800,relative,z-20").unwrap();
    let seleccion_cap_num = Selector::parse("title").unwrap();

    // dbg!(&seccion_titulo.html());

    let nombre_carpeta = codigo_fuente
        .root_element()
        .select(&seleccion_cap_num)
        .next()
        .map(|bloque| bloque.text().collect::<String>())
        .unwrap()
        .matches(|texto: char| texto.is_numeric() || texto == '.')
        .collect::<String>();
    // dbg!(&nombre_carpeta);

    let manwha_completo = codigo_fuente.select(&seleccion_manwha).next().unwrap();
    // dbg!(&manwha_completo);

    //NOTE: esto es opcional
    let link_img = codigo_fuente
        .select(&seleccion_img)
        .filter_map(|bloque| bloque.attr("src"))
        .nth(4)
        .unwrap();

    for link in manwha_completo //NOTE: esto si es la parte importante
        .select(&seleccion_img)
        .filter_map(|bloque| bloque.attr("src"))
    {
        let nombre_imagen = &link[(74 - 9)..];
        dbg!(&nombre_imagen);
        let fuente_img = get(link)?/* .text()? */;

        fs::write(
            (directorio.clone() + "/" + nombre_imagen).as_str(),
            fuente_img.bytes()?,
        );
    }

    let fuente_img = get(link_img)?/* .text()? */;
    let _ = link_siguiente(link_cap);
    // dbg!(&fuente_img);
    Ok(fuente_img)
}
fn link_siguiente(link_cap: String) -> Result<(), Box<dyn Error>> {
    let respuesta = get(link_cap.as_str())?.text()?;
    let codigo_fuente = Html::parse_document(&respuesta);

    let seleccion_flecha = Selector::parse(
        "a.h-12,px-4,flex-center,gap-2,rounded-xl,ransition-color,sf-ripple-container",
    )
    .unwrap();

    let flecha = codigo_fuente
        .select(&seleccion_flecha)
        .next_back()
        .unwrap()
        .attr("href")
        .unwrap();
    dbg!(flecha);
    dbg!((link_cap + flecha));
    Ok(())
}

fn test(codigo_fuente: Html, selector: Selector) -> Result<(), Box<dyn Error>> {
    for bloque_test in codigo_fuente.select(&selector) {
        dbg!(bloque_test);
        dbg!(bloque_test.text().collect::<String>());
        dbg!(bloque_test.attr("href"));
    }
    Ok(())
}
