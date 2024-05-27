use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

async fn generate_pdf(req: HttpRequest, body: String) -> HttpResponse {
    // Log received request from client
    println!("Received request with body: {}", body);

    // TODO: Extract the API key from the headers
    let api_key = req.headers().get("X-PDFGEN-API-KEY").and_then(|header| header.to_str().ok());

    // Define the expected API key
    let expected_api_key = "aRpq5HDQxyUjsZ3Kejm7";

    // Validate API key
    if api_key != Some(expected_api_key) {
        eprintln!("Invalid API key");
        return HttpResponse::Unauthorized().body("Invalid API key");
    }

    // Output PDF file path
    let pdf_file_path = "output.pdf";

    // Spawn wkhtmltopdf process
    let mut output = Command::new("wkhtmltopdf")
        .args(&["-", pdf_file_path])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start wkhtmltopdf process");

    // Pass HTML content to wkhtmltopdf via stdin
    if let Some(mut stdin) = output.stdin.take() {
        stdin.write_all(body.as_bytes()).expect("Failed to write HTML to wkhtmltopdf");
    }

    // Wait for process to complete
    let status = output.wait().expect("Failed to wait for wkhtmltopdf process");

    // Check if PDF generation was successful
    if status.success() {
        let mut file = File::open(pdf_file_path).expect("Failed to open PDF file");
        let mut pdf_bytes = Vec::new();
        file.read_to_end(&mut pdf_bytes).expect("Failed to read PDF file");

        HttpResponse::Ok()
            .content_type("application/pdf")
            .body(pdf_bytes)
    } else {
        eprintln!("Error generating PDF");
        HttpResponse::InternalServerError().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/generate_pdf").route(web::post().to(generate_pdf)))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}


