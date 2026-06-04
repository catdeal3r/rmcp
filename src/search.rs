use reqwest::Client;
use tokio::runtime::Runtime;

pub fn tavily_keyless_search(query: &str) -> String {
    let rt = Runtime::new().expect("FAILED: to create new tokio runtime. Err12");

    let future_task = async {
        let url = "https://api.tavily.com/search";
        let client = Client::new();
                
        let payload = format!(
            r#"{{"query": "{}", "max_results": 5}}"#, 
            query
        );

        let response = client.post(url)
            .header("Content-Type", "application/json")
            .header("X-Tavily-Access-Mode", "keyless")
            .body(payload)
            .send()
            .await
            .expect("FAILED: to create and post search. Err13");

        let response = response.error_for_status().expect("FAILED: to get search response. Err14");

        response.text().await
    };

    let res = rt.block_on(future_task).expect("FAILED: to get response text. Err15");

    res
}
