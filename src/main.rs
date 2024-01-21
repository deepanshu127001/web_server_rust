use actix_web::{web, App, HttpServer, HttpResponse};

async fn handle_request() -> HttpResponse {
    // Craft a stylish HTML response with a dark background
    let html_response = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>My First Rust Web Server 🦀</title>
            <style>
                body {
                    font-family: 'Arial', sans-serif;
                    background-color: #282c35; /* Dark background color */
                    color: #ffffff; /* Light text color */
                    margin: 40px;
                    text-align: center;
                }

                h1 {
                    color: #2ecc71; /* Green color for headings */
                    margin-bottom: 20px; /* Increased margin for spacing */
                }

                p {
                    font-size: 1.2em;
                    margin-bottom: 30px; /* Increased margin for spacing */
                }

                .emoji {
                    font-size: 2em;
                    margin-right: 10px;
                }
            </style>
        </head>
        <body>
            <h1><span class="emoji">🚀</span>My First Rust Web Server<span class="emoji">🦀</span></h1>
            <p>Welcome to my first web server built with Rust and Actix! Rust's performance and low-level control make it an excellent choice for building fast and efficient web servers. The language's focus on safety and concurrency, combined with a thriving ecosystem, ensures a smooth and enjoyable development experience. 🌐💨</p>
        </body>
        </html>
    "#;

    HttpResponse::Ok()
        .header("Content-Type", "text/html")
        .body(html_response)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(handle_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
