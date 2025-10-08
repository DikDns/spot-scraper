// src/parsers/topic_detail.rs

use crate::error::{Result, ScraperError};
use crate::models::{Answer, Content, Task, TaskStatus, TopicDetail};
use scraper::{CaseSensitivity, ElementRef, Html, Selector};

/// Helper untuk mendapatkan konten teks yang sudah di-trim dari elemen `<td>` berdasarkan indeksnya.
fn get_td_content(row: ElementRef, index: usize) -> Option<String> {
    row.select(&Selector::parse("td").unwrap())
        .nth(index)
        .map(|cell| cell.text().collect::<String>().trim().replace(": ", ""))
}

/// Helper untuk mendapatkan atribut `href` dari tag `<a>` di dalam `<td>`.
fn get_td_file_href(row: ElementRef, index: usize) -> Option<String> {
    row.select(&Selector::parse("td").unwrap())
        .nth(index)?
        .select(&Selector::parse("a").unwrap())
        .next()?
        .value()
        .attr("href")
        .map(String::from)
}

/// Fungsi utama untuk mem-parsing seluruh halaman detail topik.
pub fn parse_topic_detail_from_html(html: &str, topic_id: &str, course_id: &str) -> Result<TopicDetail> {
    let document = Html::parse_document(html);

    let description = document.select(&Selector::parse("#dashboard div").unwrap()).next().map(|el| el.text().collect::<String>().trim().to_string());
    let access_time = document.select(&Selector::parse(".panel-heading p").unwrap()).next().map(|p| p.text().collect::<String>().replace("Waktu Akses:", "").trim().to_string());

    let content_selector = Selector::parse("#materi .row .col-lg-12").unwrap();
    let contents: Vec<Content> = document
        .select(&content_selector)
        .enumerate()
        .map(|(index, content_el)| {
            let youtube_id = content_el.select(&Selector::parse("iframe[src*='youtube.com']").unwrap()).next()
                .and_then(|iframe| iframe.value().attr("src"))
                .and_then(|src| src.split("embed/").last()?.split('?').next().map(String::from));
            let raw_html = content_el.inner_html();
            Content { id: index.to_string(), youtube_id, raw_html }
        })
        .collect();

    let task_instruction_selector = Selector::parse("#tugas .table-striped").unwrap();
    let task_modal_selector = Selector::parse("#tugas .modal").unwrap();
    let task_tables: Vec<_> = document.select(&task_instruction_selector).collect();
    let modals: Vec<_> = document.select(&task_modal_selector).collect();
    let mut tasks = Vec::new();

    for (i, task_table) in task_tables.iter().enumerate() {
        let mut task = Task {
            id: String::new(), course_id: course_id.to_string(), topic_id: topic_id.to_string(), token: String::new(),
            title: String::new(), description: String::new(), file: None, start_date: None, due_date: None,
            status: TaskStatus::NotSubmitted, // Default-nya NotSubmitted
            answer: None,
        };

        for row in task_table.select(&Selector::parse("tbody > tr").unwrap()) {
            if let Some(header) = get_td_content(row, 0) {
                match header.as_str() {
                    "Judul" => task.title = get_td_content(row, 1).unwrap_or_default(),
                    "Deskripsi" => task.description = get_td_content(row, 1).unwrap_or_default(),
                    "File" => task.file = get_td_file_href(row, 1),
                    "Waktu Pengumpulan" => {
                        let dates: Vec<String> = row.select(&Selector::parse("b").unwrap()).map(|b| b.text().collect()).collect();
                        if dates.len() >= 2 {
                            task.start_date = Some(dates[0].clone());
                            task.due_date = Some(dates[1].clone());
                        }
                    }
                    _ => {}
                }
            }
        }

        let next_element = task_table.next_siblings().find_map(ElementRef::wrap);
        if let Some(sibling) = next_element {
            if sibling.value().has_class("panel-info", CaseSensitivity::AsciiCaseInsensitive) {
                task.status = TaskStatus::Submitted;
                let mut answer = Answer {
                    id: String::new(), content: String::new(), file_href: None, is_graded: false,
                    lecturer_notes: String::new(), score: 0.0, date_submitted: String::new(),
                };

                for row in sibling.select(&Selector::parse("tr").unwrap()) {
                    if let Some(header) = get_td_content(row, 0) {
                        match header.as_str() {
                            "Waktu Pengumpulan" => answer.date_submitted = get_td_content(row, 1).unwrap_or_default(),
                            "Nilai" => {
                                answer.score = get_td_content(row, 1).unwrap_or_default().parse().unwrap_or(0.0);
                                answer.is_graded = true;
                                task.status = TaskStatus::Graded;
                            },
                            "Catatan" => answer.lecturer_notes = get_td_content(row, 1).unwrap_or_default(),
                            _ => {}
                        }
                    }
                }

                let panel_body_selector = Selector::parse(".panel-body").unwrap();
                if let Some(body) = sibling.select(&panel_body_selector).next() {
                    answer.content = body.children().filter_map(|node| node.value().as_text()).map(|text| text.trim()).collect::<Vec<_>>().join(" ");

                    // --- INI PERBAIKANNYA ---
                    answer.file_href = body.select(&Selector::parse("a[href*='/tugas/mhs']").unwrap()).next()
                        .and_then(|a| a.value().attr("href"))
                        .map(|href| href.replace("/tugas/mhs", "/tugas")); // Ganti path yang salah

                    if let Some(delete_link) = body.select(&Selector::parse("a[href*='tugas_del']").unwrap()).next() {
                        answer.id = delete_link.value().attr("href").and_then(|h| h.split('/').last()).unwrap_or_default().to_string();
                    }
                }
                task.answer = Some(answer);
            }
        }

        if let Some(modal) = modals.get(i) {
            task.id = modal.select(&Selector::parse("input[name='id_tg']").unwrap()).next().and_then(|inp| inp.value().attr("value")).unwrap_or_default().to_string();
            task.token = modal.select(&Selector::parse("input[name='_token']").unwrap()).next().and_then(|inp| inp.value().attr("value")).unwrap_or_default().to_string();
        }
        tasks.push(task);
    }

    Ok(TopicDetail {
        id: topic_id.to_string(), access_time, is_accessible: true,
        href: format!("/mhs/topik/{}/{}", course_id, topic_id),
        description, contents, tasks,
    })
}
