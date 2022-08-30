/// Simple http client to  hit the rust-telemetry-datadog server
#[actix_web::main]
async fn main() -> Result<(), String> {
    let client = reqwest::Client::new();
    // List of dummy email ids used to create post request.
    // There are some duplicate data to mock http failures.
    let test_cases = vec!["name=le%20guin&email=ursula_le_guin%40gmail.com",
    "name=spicewest&email=spicewest%40hotmail.red",
    "name=dj14763&email=dj14763%40lvufaa.xyz",
    "name=evmorosh&email=evmorosh%40getmail.lt",
    "name=seal6&email=seal6%40newsote.com",
    "name=zbulygiolimpi1980om&email=zbulygiolimpi1980om%40janurganteng.com",
    "name=madisonbabin&email=madisonbabin%40gmailni.com",
    "name=dj14763&email=dj14763%40lvufaa.xyz", // Duplicate Data
    "name=wursting&email=wursting%40mailcuk.com",
    "name=petrtyumen2&email=petrtyumen2%40hoanguhanho.com",
    "name=jemonpepolel&email=jemonpepolel%40otpku.com",
    "name=nikolaevna22&email=nikolaevna22%40nproxi.com",
    "name=jemonpepolel&email=jemonpepolel%40otpku.com", // Duplicate Data
    "name=evmorosh&email=evmorosh%40getmail.lt", // Duplicate Data
    "name=ib21&email=ib21%40dmxs8.com",
    "name=conmcnamara&email=conmcnamara%40lakibaba.com",
    "name=ib21&email=ib21%40dmxs8.com", // Duplicate Data
    "name=ngrenkova&email=ngrenkova%40getmail.lt"];

    for body in test_cases {
        let response = client
            .post("http://localhost:8000/create_user")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
    }

    Ok(())
}
