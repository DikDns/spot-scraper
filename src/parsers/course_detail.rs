// src/parsers/course_detail.rs

use crate::error::{Result};
use crate::models::{Course, DetailCourse, Rps, TopicInfo};
use scraper::{ElementRef, Html, Selector};

pub fn parse_course_detail_from_html(html: &str, course: Course) -> Result<DetailCourse> {
    let document = Html::parse_document(html);

    // Handle cases where the course is not yet configured by the lecturer
    let warning_selector = Selector::parse(".white-box.bg-warning").unwrap();
    if document.select(&warning_selector).next().is_some() {
        return Ok(DetailCourse {
            course_info: course,
            description: "Course has not been set by the lecturer.".to_string(),
            rps: Rps { id: None, href: None },
            topics: Vec::new(),
        });
    }

    // --- Robust Logic for Description and RPS ---
    let rps_link_selector = Selector::parse("a.btn-danger[href*='/mhs/rps/']").unwrap();
    let rps_anchor = document.select(&rps_link_selector).next();

    let (rps, description) = if let Some(anchor) = rps_anchor {
        let href = anchor.value().attr("href").map(String::from);
        let rps_data = Rps {
            id: href.as_deref().and_then(|h| h.split('/').last().map(String::from)),
            href,
        };

        let white_box_selector = Selector::parse(".white-box").unwrap();
        let description_container = anchor.ancestors().find_map(|ancestor_node| {
            if let Some(element_ref) = ElementRef::wrap(ancestor_node) {
                if white_box_selector.matches(&element_ref) {
                    return Some(element_ref);
                }
            }
            None
        });

        let p_selector = Selector::parse("p").unwrap();
        let desc = description_container
            .and_then(|container| {
                container.select(&p_selector).find(|p| p.select(&rps_link_selector).next().is_none())
            })
            .map(|p| p.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        (rps_data, desc)
    } else {
        (Rps { id: None, href: None }, String::new())
    };

    // --- Topic Parsing Logic with Corrected `href` ---
    let topic_selector = Selector::parse(".container-fluid .block4").unwrap();
    let topics: Vec<TopicInfo> = document
        .select(&topic_selector)
        .map(|topic_el| {
            let link_selector = Selector::parse(".panel-body a.btn-info").unwrap();
            let link_element = topic_el.select(&link_selector).next();

            let is_accessible = link_element.is_some();

            // Get the full URL from the href attribute.
            let full_href = if is_accessible {
                link_element.and_then(|el| el.value().attr("href"))
            } else {
                None
            };

            // Parse the full URL and extract only the path.
            let relative_path = full_href.and_then(|url_str| {
                reqwest::Url::parse(url_str).ok().map(|url| url.path().to_string())
            });

            let time_selector = Selector::parse(".panel-body div div button.disabled").unwrap();

            TopicInfo {
                id: relative_path.as_deref().and_then(|path| path.split('/').last().map(String::from)),
                access_time: topic_el.select(&time_selector).next()
                                  .map(|btn| btn.text().collect::<String>().replace("Waktu Akses:", "").trim().to_string()),
                is_accessible,
                // Store only the relative path
                href: relative_path,
            }
        })
        .collect();

    Ok(DetailCourse {
        course_info: course,
        description,
        rps,
        topics,
    })
}
