use std::{error::Error, time::Duration};

use thirtyfour::prelude::*;

use std::thread;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    // Navigate to https://wikipedia.org.

    driver.goto("https://olympus.pages.dev/").await?;
    driver.maximize_window().await?;

    let boton_redirige_olympus = driver
        .find(By::XPath("/html/body/main/section[2]/a[1]"))
        .await?;
    boton_redirige_olympus.click().await?;

    println!(
        "{:?}",
        driver
            .query(By::XPath(
                "/html/body/div[1]/div/header/div/div[2]/button[1]"
            ))
            .exists()
            .await?
    );
    let bloque_comp = driver
        .find(By::Css("button.aspect-square:nth-child(1)"))
        .await?;
    let anuncio = driver.active_element().await?;
    thread::sleep(Duration::from_secs(2));
    anuncio.click().await?;

    println!("clicable? {:?}", anuncio.is_clickable().await?);
    println!("{:?}", anuncio.text().await?);

    // let elem_lupa = driver
    //     .find(By::XPath(
    //         "/html/body/div[1]/div/header/div/div[2]/button[1]",
    //     ))
    //     .await?;
    // elem_lupa.click().await?;

    // let cuadro_texto = driver
    //     .find(By::XPath("/html/body/article/div/main/div/div[1]/div/div"))
    //     .await?;
    // cuadro_texto.send_keys("Loco Frontera").await?;

    // let manwa_boton = driver.find(By::Name("Loco Frontera")).await?;
    // manwa_boton.click().await?;
    // Find element from element.
    // let elem_text = elem_form.find(By::Id("searchInput")).await?;

    // Type in the search terms.
    // elem_text.send_keys("selenium").await?;

    // Click the search button.
    // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
    // elem_button.click().await?;

    // Look for header to implicitly wait for the page to load.
    // driver.query(By::ClassName("firstHeading")).first().await?;
    // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
    thread::sleep(Duration::from_secs(5));

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
