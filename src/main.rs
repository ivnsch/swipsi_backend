use anyhow::{anyhow, Result};
use thirtyfour::prelude::*;

async fn extract_link(container: &WebElement) -> Result<String> {
    let link_wrappers = container
        .find_all(By::ClassName("s-title-instructions-style"))
        .await?;
    if link_wrappers.len() == 1 {
        let link_wrapper = &link_wrappers[0];
        let link = link_wrapper.find_all(By::Tag("a")).await?;
        // let link = child.find_all(By::Css(".a-link-normal.s-link-style.a-text-normal")).await?;
        // println!("link elements: {:?}", link.len());
        if link.len() == 1 {
            println!("found 1 link");
            let link = &link[0];
            let href = link.attr("href").await?.unwrap_or_default();
            // println!("link: {:?}", href);
            return Ok(href);
        } else {
            return Err(anyhow!("no links or too many found: {}", link.len()));
        }
    } else {
        return Err(anyhow!(
            "no link wrappers or too many found: {}",
            link_wrappers.len()
        ));
    }
}

async fn extract_name(container: &WebElement) -> Result<String> {
    let name = container
        .find_all(By::Css(".a-size-base-plus.a-spacing-none"))
        .await?;

    if name.len() == 1 {
        let name = &name[0];
        // println!("{:?}", name);

        let spans = name.find_all(By::Tag("span")).await?;
        // println!("spans len: {:?}", spans);
        if spans.len() == 1 {
            let span = &spans[0];
            let span_text: String = span.text().await?;
            return Ok(span_text);
            // println!("text: {:?}", span_text);
        } else {
            return Err(anyhow!(
                "multiple or no spans for name span: {}",
                spans.len()
            ));
        }
    } else {
        return Err(anyhow!("multiple or no spans for name: {}", name.len()));
    }
}

async fn extract_img(container: &WebElement) -> Result<String> {
    let img = container.find(By::ClassName("s-image")).await?;
    let img_src = img.attr("src").await?.unwrap_or_default();
    Ok(img_src)
}

async fn extract_price(container: &WebElement) -> Result<String> {
    let whole_part = container.find(By::ClassName("a-price-whole")).await?;
    let fraction_part = container.find(By::ClassName("a-price-fraction")).await?;
    let symbol_part = container.find(By::ClassName("a-price-symbol")).await?;

    if symbol_part.text().await? != "€" {
        // we assume all prices are always euros, but a double check just in case
        // TODO we should return an error here
        println!("unexpected currency symbol: {}", symbol_part);
    }

    Ok(format!("{}.{}", whole_part, fraction_part))
}

struct ProductInfo {
    name: String,
    details_link: String,
    price: String,
    img: String,
}

async fn extract_product_info(container: &WebElement) -> Result<ProductInfo> {
    match extract_link(container).await {
        Ok(link) => match extract_name(container).await {
            Ok(name) => match extract_price(container).await {
                Ok(price) => match extract_img(container).await {
                    Ok(img) => {
                        return Ok(ProductInfo {
                            name,
                            details_link: link,
                            price,
                            img,
                        })
                    }
                    Err(e) => return Err(anyhow!("error extracting img: {}", e)),
                },
                Err(e) => return Err(anyhow!("error extracting price: {}", e)),
            },
            Err(e) => return Err(anyhow!("error extracting name: {}", e)),
        },
        Err(e) => return Err(anyhow!("error extracting link: {}", e)),
    }
}

async fn extract_infos(container: &WebElement) -> Result<Vec<ProductInfo>> {
    let children = container.find_all(By::ClassName("s-result-item")).await?;
    // println!("children: {:?}", children.len());

    let mut infos = vec![];
    for child in children {
        match extract_product_info(&child).await {
            Ok(info) => {
                infos.push(info);
            }
            Err(e) => println!("error extracting link: {}", e),
        }
    }

    println!("finish a page! extracted infos: {:?}", infos.len());

    Ok(infos)
}

async fn reject_cookies_if_dialog_present(driver: &WebDriver) -> Result<()> {
    // using find all as a way to allow optional, surely there's a better way?
    let reject_cookies_buttons = driver.find_all(By::Id("sp-cc-rejectall-link")).await?;
    if reject_cookies_buttons.len() == 1 {
        let reject_cookies_button = &reject_cookies_buttons[0];
        reject_cookies_button
            .click()
            .await
            .expect("error rejecting cookies");
    }
    Ok(())
}

async fn hover_all_details_thumbnails(driver: &WebDriver) -> Result<()> {
    let thumbnails = driver.find_all(By::ClassName("imageThumbnail")).await?;
    println!("found thumbnails: {}", thumbnails.len());

    for thumbnail in thumbnails {
        let action_chain = driver.action_chain();
        action_chain
            .move_to_element_center(&thumbnail)
            .perform()
            .await?;
    }

    Ok(())
}

async fn extract_imgs_from_details(driver: &WebDriver) -> Result<Vec<String>> {
    // reject cookies - otherwise overlay on the way to hover for images
    reject_cookies_if_dialog_present(driver).await?;

    // hover so all big images are added to dom
    hover_all_details_thumbnails(driver).await?;

    let image_wrappers = driver.find_all(By::ClassName("imgTagWrapper")).await?;
    // println!("found image wrappers: {}", image_wrappers.len());

    let mut imgs = vec![];
    for image_wrapper in image_wrappers {
        let imgs_children = image_wrapper.find_all(By::Tag("img")).await?;

        if imgs_children.len() == 1 {
            let img = &imgs_children[0];
            // Get the src attribute from the img element
            let img_src = img.attr("src").await?.unwrap_or_default();
            imgs.push(img_src);
        }
    }

    Ok(imgs)
}

struct ProductDetailsInfos {
    name: String,
}

struct ProductDetails {
    name: String,
    images: Vec<String>,
}

async fn extract_infos_from_details(driver: &WebDriver) -> Result<ProductDetailsInfos> {
    let name_span = driver
        .find(By::Id("productTitle"))
        .await
        .expect("no title in details");
    let name: String = name_span.text().await?;
    Ok(ProductDetailsInfos { name })
}

async fn extract_product_details(driver: &WebDriver, link: &str) -> Result<ProductDetails> {
    driver.goto(link).await?;

    let images = extract_imgs_from_details(driver).await?;
    let infos = extract_infos_from_details(driver).await?;

    Ok(ProductDetails {
        name: infos.name.clone(),
        images,
    })
}

async fn is_in_last_page(driver: &WebDriver) -> Result<bool> {
    let next_page_disabled = driver
        .find_all(By::Css(
            ".s-pagination-item.s-pagination-next.s-pagination-disabled",
        ))
        .await?;
    Ok(!next_page_disabled.is_empty())
}

async fn extract_infos_for_all_pages(
    driver: &WebDriver,
    root_url: &str,
) -> Result<Vec<ProductInfo>> {
    driver.goto(root_url).await?;

    // reject cookies - otherwise overlay might get in the way
    reject_cookies_if_dialog_present(driver).await?;

    let mut next_page = 2;
    let mut all_links = vec![];

    while !is_in_last_page(&driver)
        .await
        .expect("error checking is last page")
    {
        let container = driver.find(By::ClassName("s-main-slot")).await?;
        let page_links = extract_infos(&container).await.expect("...");
        all_links.extend(page_links);

        let next_page_par = format!("&page={}", next_page);
        driver
            .goto(format!("{}{}", root_url, next_page_par))
            .await?;

        next_page += 1;
    }

    println!("finished extracting links for {} pages", next_page - 1);
    Ok(all_links)
}

async fn collect_details(driver: &WebDriver, infos: &[ProductInfo]) -> Result<Vec<ProductDetails>> {
    let mut product_details: Vec<ProductDetails> = vec![];
    for info in infos {
        let full_link = format!("https://amazon.de{}", info.details_link);
        // example link to test just one page (comment loop)
        // let full_link = "https://amazon.de/sspa/click?ie=UTF8&spc=MTo1NzU5Nzg0NjQ1NTU0NDQ3OjE3MzkyODE1MTc6c3BfYXRmOjMwMDM0NTQ5MTgzMDkzMjo6MDo6&url=%2Fs-Oliver-Damen-Ring-Edelstahl-Swarovski-Kristalle-Breite%2Fdp%2FB07FD729LJ%2Fref%3Dsr_1_1_sspa%3Fdib%3DeyJ2IjoiMSJ9.bMcM1L4llnp90s8_saI8idf565ai9cImntwXUe2M0C30kPlwkWo5Mq4k3_LOO0SUP9Sofu-TCe-QjGORDi_lOu27QdUkGVQWDkjZXEkky-eccusHY51_ZOZkG17ILR6j87jO3SruEkxLu8sLzm2M7EP6395CeKLq3xLgZsCr1FWu1PM-L2BtlBGGPGKgP6VPXRnH_EK8ZyqTJCR-L74_FOdgcQ7VB_brEhBqiDW4enmS4wKswD83qTT5kzf08WvEkMwIYAOBQkfef6kEkzc6v7W3IWaTZ5ScMQUc7i1zfjU.IPHI5Mxj-tn6zvcwFmWLZHZjVOKsEfuykyn9d1QDWCE%26dib_tag%3Dse%26keywords%3Dringe%26qid%3D1739281517%26s%3Dapparel%26sr%3D1-1-spons%26sp_csd%3Dd2lkZ2V0TmFtZT1zcF9hdGY%26psc%3D1".to_string();

        match extract_product_details(driver, &full_link).await {
            Ok(details) => {
                product_details.push(details);
            }
            Err(e) => {
                println!(
                    "Couldn't extract product details for: {}, error: {}",
                    full_link, e
                )
            }
        }
    }
    Ok(product_details)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:52711", caps).await?;

    // let root_url: &str = "https://www.amazon.de/s?k=ringe&i=fashion";
    // only a few pages
    let root_url: &str = "https://www.amazon.de/s?k=disinfectant+hand";

    let infos = extract_infos_for_all_pages(&driver, root_url)
        .await
        .expect("couldn't extract links");
    // println!("extracted links ({}) for all pages: {:?}", links.len(), links);
    println!("extracted links ({}) for all pages", infos.len());

    // collect details
    collect_details(&driver, &infos)
        .await
        .expect("couldn't collect details");

    // Keep the browser open by looping indefinitely
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }

    // Always explicitly close the browser.
    // driver.quit().await?;

    Ok(())
}
