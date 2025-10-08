// tests/all_topics_test.rs

use spot_scraper::{Result, SpotClient};
use std::env;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use tokio::time::sleep;

/// Integration test ini akan melakukan scraping SEMUA topik yang bisa diakses
/// dari SEMUA mata kuliah dan menyimpan output-nya ke file `all_topics_output.log`.
///
/// Test ini sengaja dibuat lambat dengan jeda antar request untuk menghormati
/// server SPOT dan menghindari potensi rate-limiting.
///
/// Cara menjalankan test ini:
/// SPOT_NIM="your_nim" SPOT_PASSWORD="your_password" cargo test test_scrape_all_topics -- --nocapture
#[tokio::test]
// #[ignore] // Hapus baris ini jika kamu benar-benar ingin menjalankan test yang intensif ini
async fn test_scrape_all_topics() -> Result<()> {
    // --- SETUP: Buat file log ---
    let mut log_file = File::create("all_topics_output.log")
        .expect("Tidak bisa membuat file log.");

    // --- SETUP: Ambil kredensial ---
    let nim = env::var("SPOT_NIM").expect("ERROR: Environment variable SPOT_NIM tidak di-set.");
    let password = env::var("SPOT_PASSWORD").expect("ERROR: Environment variable SPOT_PASSWORD tidak di-set.");

    writeln!(log_file, "--- Memulai Full Topic Scraping Test ---").unwrap();
    println!("--- Memulai Full Topic Scraping Test ---");

    let client = SpotClient::new();

    // --- Langkah 1: Login ---
    writeln!(log_file, "\n[1/4] Mencoba login...").unwrap();
    println!("[1/4] Mencoba login...");
    client.login(&nim, &password).await?;
    writeln!(log_file, "[1/4] Login berhasil!").unwrap();

    // --- Langkah 2: Ambil Daftar Mata Kuliah ---
    writeln!(log_file, "\n[2/4] Mengambil daftar mata kuliah...").unwrap();
    println!("[2/4] Mengambil daftar mata kuliah...");
    let courses = client.get_courses().await?;
    writeln!(log_file, "[2/4] Berhasil mendapatkan {} mata kuliah.", courses.len()).unwrap();

    // --- Langkah 3 & 4: Loop Semua Mata Kuliah dan Topiknya ---
    writeln!(log_file, "\n[3/4] & [4/4] Memulai proses scraping untuk setiap mata kuliah dan topik...").unwrap();
    println!("[3/4] & [4/4] Memulai proses scraping untuk setiap mata kuliah dan topik...");

    for (course_index, course) in courses.iter().enumerate() {
        writeln!(log_file, "\n==================================================").unwrap();
        writeln!(log_file, "({}/{}) Scraping Course: {}", course_index + 1, courses.len(), course.name).unwrap();
        println!("\n({}/{}) Scraping Course: {}", course_index + 1, courses.len(), course.name);

        // Jeda singkat antar request mata kuliah
        sleep(Duration::from_millis(500)).await;

        match client.get_course_detail(course).await {
            Ok(course_detail) => {
                let accessible_topics: Vec<_> = course_detail.topics.iter().filter(|t| t.is_accessible).collect();
                if accessible_topics.is_empty() {
                    writeln!(log_file, " -> Tidak ada topik yang bisa diakses di mata kuliah ini.").unwrap();
                    println!(" -> Tidak ada topik yang bisa diakses di mata kuliah ini.");
                    continue;
                }

                for (topic_index, topic_info) in accessible_topics.iter().enumerate() {
                    writeln!(log_file, "\n--------------------------------------------------").unwrap();
                    writeln!(log_file, "    -> ({}/{}) Scraping Topic ID: {}", topic_index + 1, accessible_topics.len(), topic_info.id.as_deref().unwrap_or("N/A")).unwrap();
                    println!("    -> ({}/{}) Scraping Topic ID: {}", topic_index + 1, accessible_topics.len(), topic_info.id.as_deref().unwrap_or("N/A"));

                    // Jeda singkat antar request topik
                    sleep(Duration::from_millis(250)).await;

                    match client.get_topic_detail(topic_info).await {
                        Ok(topic_detail) => {
                            writeln!(log_file, "{:#?}", topic_detail).unwrap();
                        }
                        Err(e) => {
                            writeln!(log_file, "    -> ERROR: Gagal mengambil detail topik: {:?}", e).unwrap();
                            println!("    -> ERROR: Gagal mengambil detail topik: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                writeln!(log_file, " -> ERROR: Gagal mengambil detail mata kuliah: {:?}", e).unwrap();
                println!(" -> ERROR: Gagal mengambil detail mata kuliah: {:?}", e);
            }
        }
    }

    writeln!(log_file, "\n--- Full Topic Scraping Test Selesai ---").unwrap();
    println!("\n--- Full Topic Scraping Test Selesai ---");
    println!("Semua output telah disimpan di 'all_topics_output.log'.");

    Ok(())
}
