// src/parsers/course_detail.rs

use crate::error::Result;
use crate::models::{Course, DetailCourse, Rps, TopicInfo};
use chrono::NaiveDateTime;
use scraper::{ElementRef, Html, Selector};

fn parse_dt(s: &str) -> Option<NaiveDateTime> {
    let t = s.trim();
    [
        "%d-%m-%Y %H:%M",
        "%d-%m-%Y %H:%M:%S",
        "%d/%m/%Y %H:%M",
        "%d/%m/%Y %H:%M:%S",
    ]
    .iter()
    .find_map(|fmt| NaiveDateTime::parse_from_str(t, fmt).ok())
}

pub fn parse_course_detail_from_html(html: &str, course: Course) -> Result<DetailCourse> {
    let document = Html::parse_document(html);

    // Handle cases where the course is not yet configured by the lecturer
    let warning_selector = Selector::parse(".white-box.bg-warning").unwrap();
    if document.select(&warning_selector).next().is_some() {
        return Ok(DetailCourse {
            course_info: course,
            description: "Course has not been set by the lecturer.".to_string(),
            rps: Rps {
                id: None,
                href: None,
            },
            topics: Vec::new(),
        });
    }

    // --- Robust Logic for Description and RPS ---
    let rps_link_selector = Selector::parse("a.btn-danger[href*='/mhs/rps/']").unwrap();
    let rps_anchor = document.select(&rps_link_selector).next();

    let (rps, description) = if let Some(anchor) = rps_anchor {
        let href = anchor.value().attr("href").map(String::from);
        let rps_data = Rps {
            id: href
                .as_deref()
                .and_then(|h| h.split('/').last())
                .and_then(|v| v.parse::<u64>().ok()),
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
                container
                    .select(&p_selector)
                    .find(|p| p.select(&rps_link_selector).next().is_none())
            })
            .map(|p| p.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        (rps_data, desc)
    } else {
        (
            Rps {
                id: None,
                href: None,
            },
            String::new(),
        )
    };

    // --- Topic Parsing Logic ---
    let topic_selector = Selector::parse(".container-fluid .block4").unwrap();
    let topics: Vec<TopicInfo> = document
        .select(&topic_selector)
        .map(|topic_el| {
            let link_selector = Selector::parse(".panel-body a.btn-info").unwrap();
            let link_element = topic_el.select(&link_selector).next();

            let is_accessible = link_element.is_some();

            // Get href attribute (may be absolute or relative).
            let raw_href = if is_accessible {
                link_element.and_then(|el| el.value().attr("href"))
            } else {
                None
            };

            // Normalize into a relative path string.
            let relative_path = raw_href.and_then(|url_str| {
                if url_str.starts_with("http://") || url_str.starts_with("https://") {
                    reqwest::Url::parse(url_str)
                        .ok()
                        .map(|url| url.path().to_string())
                } else {
                    Some(url_str.to_string())
                }
            });

            let time_selector = Selector::parse(".panel-body div div button.disabled").unwrap();

            TopicInfo {
                id: relative_path
                    .as_deref()
                    .and_then(|path| path.split('/').last())
                    .and_then(|v| v.parse::<u64>().ok()),
                course_id: relative_path
                    .as_deref()
                    .and_then(|path| path.split('/').nth(3))
                    .and_then(|v| v.parse::<u64>().ok()),
                access_time: topic_el.select(&time_selector).next().and_then(|btn| {
                    parse_dt(&btn.text().collect::<String>().replace("Waktu Akses:", ""))
                }),
                is_accessible,
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
