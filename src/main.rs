use std::{sync::Arc, thread::sleep, time::Duration};
use anyhow::Result;
use thirtyfour::{action_chain::ActionChain, common::command::Actions, prelude::*, session::handle::SessionHandle};

async fn extract_links(container: &WebElement) -> Result<Vec<String>> {
     let children = container.find_all(By::ClassName("s-result-item")).await?;
     // println!("children: {:?}", children.len());

     let mut hrefs = vec![];
     for child in children {

          // get link..

          let link = child.find_all(By::Css(".a-link-normal.s-link-style.a-text-normal")).await?;
          // println!("link elements: {:?}", link.len()); 
          if link.len() == 1 {
               let link = &link[0];
               let href = link.attr("href").await?.unwrap_or_default();
               println!("link: {:?}", href);
               hrefs.push(href);
          } else {
               println!("no links or too many links found: {}", link.len());
          }

          // get name (not used right now)..

          let name = child.find_all(By::Css(".a-size-base-plus.a-spacing-none")).await?;
          // let name = child.find_all(By::ClassName("a-size-base-plus")).await?;
          // println!("name elements: {:?}", name.len());

          // Get the text from the span element

          if name.len() == 1 {
               let name = &name[0];
               // println!("{:?}", name);

               let spans = name.find_all(By::Tag("span")).await?;
               // println!("spans len: {:?}", spans);
               if spans.len() == 1 {
                    let span = &spans[0];
                    let span_text = span.text().await?;
                    // println!("text: {:?}", span_text);
               }
          }
     }

     // // Find element from element.
     // let elem_text = elem_form.find(By::Id("searchInput")).await?;

     // // Type in the search terms.
     // elem_text.send_keys("selenium").await?;

     // // Click the search button.
     // let elem_button = elem_form.find(By::Css("button[type='submit']")).await?;
     // elem_button.click().await?;

     // // Look for header to implicitly wait for the page to load.
     // driver.find(By::ClassName("firstHeading")).await?;
     // assert_eq!(driver.title().await?, "Selenium - Wikipedia");
     
     println!("finish a page! extracted links: {:?}", hrefs.len());

     Ok(hrefs)
}

async fn reject_cookies(driver: &WebDriver) -> Result<()> {
     let reject_cookies_button =  driver.find(By::Id("sp-cc-rejectall-link")).await?;
     reject_cookies_button.click().await.expect("error rejecting cookies");
     Ok(())
}

async fn hover_all_details_thumbnails(driver: &WebDriver) -> Result<()> {
     let thumbnails =  driver.find_all(By::ClassName("imageThumbnail")).await?;
     println!("found thumbnails: {}", thumbnails.len()); 

     for thumbnail in thumbnails {
          let action_chain = driver.action_chain(); 
          action_chain.move_to_element_center(&thumbnail).perform().await?;
     }

     Ok(())
}

async fn extract_imgs_from_details(driver: &WebDriver) -> Result<Vec<String>> {
    
     // reject cookies - otherwise overlay on the way to hover for images
     reject_cookies(driver).await?;

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

async fn is_in_last_page(driver: &WebDriver) -> Result<bool> {
     let next_page_disabled = driver.find_all(By::Css(".s-pagination-item.s-pagination-next.s-pagination-disabled")).await?;
     Ok(!next_page_disabled.is_empty())
}

async fn extract_links_for_all_pages(driver: &WebDriver, root_url: &str) -> Result<Vec<String>> {
     driver.goto(root_url).await?;

     // reject cookies - otherwise overlay might get in the way
     reject_cookies(driver).await?;

     let mut next_page = 2;
     let mut all_links = vec![];

     // while !is_in_last_page(&driver).await.expect("error checking is last page")  {
          let container = driver.find(By::ClassName("s-main-slot")).await?;
          let page_links = extract_links(&container).await.expect("...");
          all_links.extend(page_links);

          let next_page_par = format!("&page={}", next_page);
          driver.goto(format!("{}{}", root_url, next_page_par)).await?;

          next_page += 1;
     // }

     println!("finished extracting links for {} pages", next_page - 1);
     Ok(all_links)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:54970", caps).await?;

     // let root_url: &str = "https://www.amazon.de/s?k=ringe&i=fashion";
     let root_url: &str = "https://www.amazon.de/s?k=disinfectant+hand";

     let links = extract_links_for_all_pages(&driver, root_url).await.expect("couldn't extract links");
     println!("extracted links ({}) for all pages: {:?}", links.len(), links);

     // // for link in links {
     //      // let link = &links[0];
     //      // let full_link = format!("https://amazon.de{}", link);
     //      let full_link = "https://amazon.de/sspa/click?ie=UTF8&spc=MTo1NzU5Nzg0NjQ1NTU0NDQ3OjE3MzkyODE1MTc6c3BfYXRmOjMwMDM0NTQ5MTgzMDkzMjo6MDo6&url=%2Fs-Oliver-Damen-Ring-Edelstahl-Swarovski-Kristalle-Breite%2Fdp%2FB07FD729LJ%2Fref%3Dsr_1_1_sspa%3Fdib%3DeyJ2IjoiMSJ9.bMcM1L4llnp90s8_saI8idf565ai9cImntwXUe2M0C30kPlwkWo5Mq4k3_LOO0SUP9Sofu-TCe-QjGORDi_lOu27QdUkGVQWDkjZXEkky-eccusHY51_ZOZkG17ILR6j87jO3SruEkxLu8sLzm2M7EP6395CeKLq3xLgZsCr1FWu1PM-L2BtlBGGPGKgP6VPXRnH_EK8ZyqTJCR-L74_FOdgcQ7VB_brEhBqiDW4enmS4wKswD83qTT5kzf08WvEkMwIYAOBQkfef6kEkzc6v7W3IWaTZ5ScMQUc7i1zfjU.IPHI5Mxj-tn6zvcwFmWLZHZjVOKsEfuykyn9d1QDWCE%26dib_tag%3Dse%26keywords%3Dringe%26qid%3D1739281517%26s%3Dapparel%26sr%3D1-1-spons%26sp_csd%3Dd2lkZ2V0TmFtZT1zcF9hdGY%26psc%3D1".to_string();
     //      // println!("!! link: {}", full_link);

     //      driver.goto(full_link).await?;

     //      let imgs = extract_imgs_from_details(&driver).await.expect("couldn't extract imgs from details");

     //      println!("images: {:?}", imgs);
     // // }

      // Keep the browser open by looping indefinitely
      loop {
          tokio::time::sleep(std::time::Duration::from_secs(60)).await;
      }

      // Always explicitly close the browser.
     // driver.quit().await?;

     Ok(())
}