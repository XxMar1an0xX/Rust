use dirs;
use reqwest::blocking::{Response, get};
use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::copy;
use std::path::Path;
use std::{error::Error, fmt::Formatter};
fn main() -> Result<(), Box<dyn Error>> {
    //TODO: checkear por archivos ya hechos asi no re-hacer
    println!("Olympus empieza...");

    let link = "https://olympusbiblioteca.com";
    let fuente_raw = get(link)?.text()?;

    //NOTE: aqui se lo formatea a HTML para que se pueda extraer ciertas partes o "bloques"
    let html_fuente = Html::parse_document(&fuente_raw);

    let (nombre, agregado_manwha) = nombre_y_link(&html_fuente, "Loco Frontera")?;

    let directorio = dirs::home_dir()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_string()
        + "/Descargas/"
        + nombre.as_str();
    let _ = fs::create_dir_all(&directorio)?;

    let mut agregado_cap = primer_cap(link.to_string(), agregado_manwha)?;

    loop {
        let _ = extraer_cap(link.to_string(), &agregado_cap, &directorio)?;

        let siguiente = link_siguiente(link.to_string(), &agregado_cap)?;

        dbg!(&(link.to_string() + &siguiente));
        agregado_cap = siguiente;
        if agregado_cap == "".to_string() {
            break;
        }
    }

    println!("Ya c descargo todo bv");
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

fn extraer_cap(
    link_base: String,
    agregado_cap: &str,
    direccion: &str,
) -> Result<(), Box<dyn Error>> {
    let codigo_fuente = extraer_codigo_fuente(link_base + &agregado_cap)?;
    let seleccion_imgs = Selector::parse("img.object-cover,rounded-inherit,w-full,h-full").unwrap();
    let seleccion_cap_num = Selector::parse("title").unwrap();

    let num_cap = codigo_fuente
        .select(&seleccion_cap_num)
        .next()
        .map(|bloque| bloque.text().collect::<String>())
        .unwrap()
        .matches(|texto: char| texto.is_numeric() || texto == '.')
        .collect::<String>();
    println!(">>Descargando cap {}", &num_cap);

    let direccion_cap = direccion.to_string() + "/capitulo " + &num_cap;
    let _ = fs::create_dir_all(&direccion_cap);

    for imagen_link in codigo_fuente
        .select(&seleccion_imgs)
        .filter(|bloque| bloque.attr("srcset").is_none())
        .filter_map(|bloque| bloque.attr("src"))
    {
        let nombre_imagen = &imagen_link[(74 - 9)..];
        println!("descargando: {}", nombre_imagen.to_string());

        let direccion_imagen = direccion_cap.clone() + "/" + nombre_imagen;
        if Path::exists(Path::new(&direccion_imagen)) {
        } else {
            let fuente_img = get(imagen_link)?;
            fs::write(&direccion_imagen, fuente_img.bytes()?)?;
        }
    }

    Ok(())
}

fn extraer_codigo_fuente(link: String) -> Result<Html, Box<dyn Error>> {
    let mut codigo_fuente = Html::parse_fragment("");
    let pagina = get(link.as_str())?.text()?;
    codigo_fuente = Html::parse_fragment(&pagina);

    Ok(codigo_fuente)
}

fn link_siguiente(link_base: String, agregado_cap: &str) -> Result<String, Box<dyn Error>> {
    let codigo_fuente = extraer_codigo_fuente(link_base + agregado_cap)?;

    let seleccion_flecha = Selector::parse(
        "a.h-12,px-4,flex-center,gap-2,rounded-xl,ransition-color,sf-ripple-container",
    )
    .unwrap();

    let link_siguiente: &str = codigo_fuente
        .select(&seleccion_flecha)
        .next_back()
        .unwrap()
        .attr("href")
        .unwrap();
    Ok(link_siguiente.to_string())
}

fn test(codigo_fuente: Html, selector: Selector) -> Result<(), Box<dyn Error>> {
    for bloque_test in codigo_fuente.select(&selector) {
        dbg!(bloque_test);
        dbg!(bloque_test.text().collect::<String>());
        dbg!(bloque_test.attr("href"));
    }
    Ok(())
}
