use anyhow::{anyhow, Result};
use csv::Writer;
use thirtyfour::prelude::*;
use url::Url;

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

            let processed_href = process_infos_link(href)?;
            // println!("link: {:?}", href);
            return Ok(processed_href);
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

fn process_infos_link(link: String) -> Result<String> {
    let base = "https://amazon.de";
    let full_link = format!("{}{}", base, link);
    let url = Url::parse(&full_link)?;

    let segments: Vec<&str> = url
        .path_segments()
        .map(|c| c.collect())
        .unwrap_or_else(Vec::new);

    // extract dp segment (actual identification of product)
    let dp_path = segments
        .iter()
        .skip_while(|&&segment| segment != "dp")
        .take(2)
        .cloned()
        .collect::<Vec<&str>>()
        .join("/");

    // reconstruct minimal url
    let mut new_url = Url::parse("https://www.amazon.de").expect("Failed to parse base URL");
    new_url.set_path(&format!("/{}", dp_path));
    new_url.set_query(None); // remove all query parameters

    new_url.query_pairs_mut().append_pair("tag", "my-tag");

    Ok(new_url.to_string())
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
    // img_src is low res which we don't need currently
    // let img_src = img.attr("src").await?.unwrap_or_default();
    let img_src_set = img.attr("srcset").await?.unwrap_or_default();
    println!("image src set: {:?}", img_src_set);

    let highest_res_img = extract_highest_res_img(&img_src_set)?;

    Ok(highest_res_img)
}

// amazon adds an attribute to images in overview srcset with entries separated by ,
// respectively containing a src and a string denoting resolution (like "2x")
// we choose the highest res string, which empirically it's about right (slightly lower probably) for mobile devices.
fn extract_highest_res_img(src_set: &str) -> Result<String> {
    let resolution_entries = src_set.split(",").map(|e| e.trim()).collect::<Vec<&str>>();

    // note: we assume highest resolution to be the last
    // if we wanted to be more robust we'd parse all the resolutions and get highest
    // but it seems that highest is always returned last so using that for now
    if let Some(last) = resolution_entries.last() {
        let parts = last.split_whitespace().collect::<Vec<&str>>();
        if parts.len() == 2 {
            let src = parts[0].trim();
            let res = parts[1].trim();

            if res != "3x" {
                // either we got a higher than 3x resolution (seems not to exist currently? or they're not sorted as expected)
                println!("warn: last resolution isn't 3x, are we getting the highest res?")
            }

            Ok(src.to_string())
        } else {
            // a resolution looks like this: 1x, 1.5x and so on
            Err(anyhow!(
                "unexpected: srcset entries should be `<imagepath> <resolution>`"
            ))
        }
    } else {
        Err(anyhow!("unexpected: srcset didn't contain any images"))
    }
}

async fn extract_price(container: &WebElement) -> Result<(String, f32)> {
    let whole_part = container.find(By::ClassName("a-price-whole")).await?;
    let fraction_part = container.find(By::ClassName("a-price-fraction")).await?;
    let symbol_part = container.find(By::ClassName("a-price-symbol")).await?;

    if symbol_part.text().await? != "€" {
        // we assume all prices are always euros, but a double check just in case
        // TODO we should return an error here
        println!("unexpected currency symbol: {}", symbol_part);
    }

    let price_str = format!(
        "{}.{}",
        whole_part.text().await?,
        fraction_part.text().await?
    );

    let price_float = price_str.parse()?;

    Ok((price_str, price_float))
}

pub struct ProductInfo {
    name: String,
    details_link: String,
    price: String,
    price_number: f32,
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
                            price: price.0,
                            price_number: price.1,
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

pub async fn extract_infos_for_all_pages(
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

#[allow(unused)]
async fn collect_details(driver: &WebDriver, infos: &[ProductInfo]) -> Result<Vec<ProductDetails>> {
    let mut product_details: Vec<ProductDetails> = vec![];
    for info in infos {
        // example link to test just one page (comment loop)
        // let link = "https://amazon.de/sspa/click?ie=UTF8&spc=MTo1NzU5Nzg0NjQ1NTU0NDQ3OjE3MzkyODE1MTc6c3BfYXRmOjMwMDM0NTQ5MTgzMDkzMjo6MDo6&url=%2Fs-Oliver-Damen-Ring-Edelstahl-Swarovski-Kristalle-Breite%2Fdp%2FB07FD729LJ%2Fref%3Dsr_1_1_sspa%3Fdib%3DeyJ2IjoiMSJ9.bMcM1L4llnp90s8_saI8idf565ai9cImntwXUe2M0C30kPlwkWo5Mq4k3_LOO0SUP9Sofu-TCe-QjGORDi_lOu27QdUkGVQWDkjZXEkky-eccusHY51_ZOZkG17ILR6j87jO3SruEkxLu8sLzm2M7EP6395CeKLq3xLgZsCr1FWu1PM-L2BtlBGGPGKgP6VPXRnH_EK8ZyqTJCR-L74_FOdgcQ7VB_brEhBqiDW4enmS4wKswD83qTT5kzf08WvEkMwIYAOBQkfef6kEkzc6v7W3IWaTZ5ScMQUc7i1zfjU.IPHI5Mxj-tn6zvcwFmWLZHZjVOKsEfuykyn9d1QDWCE%26dib_tag%3Dse%26keywords%3Dringe%26qid%3D1739281517%26s%3Dapparel%26sr%3D1-1-spons%26sp_csd%3Dd2lkZ2V0TmFtZT1zcF9hdGY%26psc%3D1".to_string();

        match extract_product_details(driver, &info.details_link).await {
            Ok(details) => {
                product_details.push(details);
            }
            Err(e) => {
                println!(
                    "Couldn't extract product details for: {}, error: {}",
                    info.details_link, e
                )
            }
        }
    }
    Ok(product_details)
}

fn to_csv(infos: &[ProductInfo]) -> Result<()> {
    let mut wtr = Writer::from_path("product_infos.csv")?;
    for info in infos {
        wtr.write_record(&[
            info.name.clone(),
            info.price.clone(),
            info.img.clone(),
            info.details_link.clone(),
        ])?;
    }
    wtr.flush()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use thirtyfour::{DesiredCapabilities, WebDriver};

    use crate::scrapper::{extract_infos_for_all_pages, to_csv};

    #[tokio::test]
    async fn scrap() -> Result<()> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:52711", caps).await?;

        // let root_url: &str = "https://www.amazon.de/s?k=ringe&i=fashion";
        // only a few pages
        let root_url: &str = "https://www.amazon.de/s?k=disinfectant+hand";

        let infos = extract_infos_for_all_pages(&driver, root_url).await?;
        // println!("extracted links ({}) for all pages: {:?}", links.len(), links);

        println!("extracted links ({}) for all pages", infos.len());

        to_csv(&infos)?;

        // // collect details
        // collect_details(&driver, &infos)
        //     .await?;

        // Keep the browser open by looping indefinitely
        // enable this if needed to inspect something after finish
        // loop {
        //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;
        // }

        // Always explicitly close the browser.
        // driver.quit().await?;

        Ok(())
    }
}
