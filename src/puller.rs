use std::borrow::BorrowMut;

use crate::{client, model};

#[allow(dead_code)]
pub async fn get_manga_list(page: i64) -> Vec<model::Manga> {
    let link = format!(
        "https://www.niceoppai.net/manga_list/all/any/name-az/{}/",
        page
    );
    let response = client::builder(reqwest::Method::GET, link.as_str())
        .send()
        .await;

    match response {
        Ok(res) => {
            let document = scraper::Html::parse_document(res.text().await.unwrap().as_str());

            let manga_selector =
                scraper::Selector::parse("#sct_content > div > div > div > div.cvr > div > a")
                    .unwrap();

            let image_selector = scraper::Selector::parse("img").unwrap();

            return document
                .select(&manga_selector)
                .map(|e| model::Manga {
                    name: String::from(
                        e.select(&image_selector)
                            .next()
                            .unwrap()
                            .value()
                            .attr("alt")
                            .unwrap(),
                    ),
                    path: String::from(e.value().attr("href").unwrap()),
                    cover_image_url: String::from(
                        e.select(&image_selector)
                            .next()
                            .unwrap()
                            .value()
                            .attr("src")
                            .unwrap(),
                    ),
                    eps: vec![],
                })
                .collect::<Vec<model::Manga>>();
        }
        Err(e) => log::error!("{}", e),
    }

    return Vec::new();
}

#[allow(dead_code)]
pub async fn get_manga_info(url: &str) -> Option<model::Manga> {
    let link = [url, "1"].join("/");
    let response = client::builder(reqwest::Method::GET, link.as_str())
        .send()
        .await;

    match response {
        Ok(res) => {
            let document = scraper::Html::parse_document(res.text().await.unwrap().as_str());

            let option_selector = scraper::Selector::parse(
                "#sct_content > div.wpm_pag.mng_rdr > div:nth-child(2) > ul.nav_chp > li > select > option",
            ).unwrap();

            let title_selector =
                scraper::Selector::parse("#sct_content > div.wpm_pag.mng_rdr > h1 > a").unwrap();

            let mut manga = model::Manga {
                name: document
                    .select(&title_selector)
                    .next()
                    .unwrap()
                    .inner_html(),
                path: url.parse().unwrap(),
                cover_image_url: "".to_string(),
                eps: document
                    .select(&option_selector)
                    .map(|e| model::MangaEp {
                        name: String::from(e.inner_html()),
                        ep_path: String::from(e.value().attr("value").unwrap()),
                    })
                    .collect::<Vec<model::MangaEp>>(),
            };
            manga.eps.reverse();

            return Some(manga);
        }
        Err(e) => log::error!("{}", e),
    }

    return None;
}

#[allow(dead_code)]
pub async fn get_pages(link: &str) -> Vec<model::Page> {
    let response = client::builder(reqwest::Method::GET, link).send().await;

    let mut pages: Vec<model::Page> = Vec::new();

    match response {
        Ok(res) => {
            let text = res.text().await.unwrap();
            let document = scraper::Html::parse_document(text.as_str());
            let page_selector = scraper::Selector::parse("#image-container > center").unwrap();
            let script_selector = scraper::Selector::parse("script").unwrap();
            let img_selector = scraper::Selector::parse("img").unwrap();
            let div_selector = scraper::Selector::parse("div").unwrap();

            for ele in document.select(&page_selector) {
                let mut select = ele.select(&script_selector);
                let mut page = model::Page {
                    content_type: model::ContentType::Script,
                    content: "".to_string(),
                    content_base_id: "".to_string(),
                    container_style: "".to_string(),
                };
                if select.borrow_mut().count() != 0 {
                    page.content = select.next().unwrap().inner_html();
                    let div_ele = ele.select(&div_selector).next().unwrap();
                    page.content_base_id = String::from(div_ele.value().attr("id").unwrap());
                    page.container_style = String::from(div_ele.value().attr("style").unwrap());
                } else {
                    page.content_type = model::ContentType::Image;
                    page.content = String::from(
                        ele.select(&img_selector)
                            .next()
                            .unwrap()
                            .value()
                            .attr("src")
                            .unwrap(),
                    );
                }
                pages.push(page);
            }
        }
        Err(e) => log::error!("{}", e),
    }

    return pages;
}
