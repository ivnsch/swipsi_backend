use std::{thread::sleep, time::Duration};
use anyhow::Result;
use thirtyfour::prelude::*;

async fn extract_links(container: &WebElement) -> Result<Vec<String>> {
     let children = container.find_all(By::ClassName("s-result-item")).await?;
     // println!("children: {:?}", children.len());

     let mut hrefs = vec![];
     for child in children {

          // get link..

          let link = child.find_all(By::Css(".a-link-normal.s-line-clamp-2.s-link-style.a-text-normal")).await?;
          // println!("link elements: {:?}", link.len()); 
          if link.len() == 1 {
               let link = &link[0];
               let href = link.attr("href").await?.unwrap_or_default();
               println!("link: {:?}", href);
               hrefs.push(href);
          }

          // get name (not used right now)..

          let name = child.find_all(By::Css(".a-size-base-plus.a-spacing-none")).await?;
          // let name = child.find_all(By::ClassName("a-size-base-plus")).await?;
          // println!("name elements: {:?}", name.len());

          // Get the text from the span element

          if name.len() == 1 {
               let name = &name[0];
               println!("{:?}", name);

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
     
     println!("????");

     println!("finish! extrancted links: {:?}", hrefs.len());

     Ok(hrefs)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
     let caps = DesiredCapabilities::chrome();
     let driver = WebDriver::new("http://localhost:54970", caps).await?;

     // Navigate to https://wikipedia.org.
     driver.goto("https://www.amazon.de/s?k=ringe&i=fashion").await?;
     let container = driver.find(By::ClassName("s-main-slot")).await?;

     // let children = container.find_all(By::XPath("./*")).await?;

    
    let links = extract_links(&container).await.expect("...");







 
     // loop {
     //      sleep(Duration::from_secs(60));
     //  } 
      // Always explicitly close the browser.
     // driver.quit().await?;

     Ok(())
}