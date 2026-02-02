use reqwest::Client;
use serde_json::json;


pub struct FireflyClient {
    firefly_base_url: String,
    firefly_token: String,
}

#[derive(Debug)]
pub struct Transaction{
    pub amount: f64,
    pub description: String,
    pub note: Option<String>,
    pub date: Option<String>,
    pub transaction_type: Option<String>,
}


impl FireflyClient {
    pub fn new(firefly_base_url: String, firefly_token: String) -> Self {
        Self {
            firefly_base_url,
            firefly_token,
        }
    }

    pub async fn create_transaction(
        &self,
        transaction: &Transaction, // Accept a reference to the Transaction struct
    ) -> Result<bool, reqwest::Error> {
        let url = format!("{}/api/v1/transactions", self.firefly_base_url);
        let headers = {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.firefly_token)
                    .parse()
                    .unwrap(),
            );
            headers.insert(
                reqwest::header::CONTENT_TYPE,
                "application/json".parse().unwrap(),
            );
            headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
            headers
        };

        // Convert the Transaction struct into the required JSON payload
        let data_payload = json!({
            "transactions": [{
                "amount": transaction.amount,
                "description": transaction.description,
                "notes": transaction.note,
                "date": transaction.date,
                "type": transaction.transaction_type,
                "source_name": "Cash wallet",
                "tags": ["matrix_bot"]
            }]
        });

        let client = Client::new();
        let response = client.post(&url).headers(headers).json(&data_payload).send().await?;

        if response.status().is_success() {
            println!(
                "Transaction created successfully: {:?}",
                response.json::<serde_json::Value>().await?
            );
            Ok(true)
        } else {
            eprintln!(
                "Failed to create transaction: {} - {}",
                response.status(),
                response.text().await?
            );
            Ok(false)
        }
    }
}