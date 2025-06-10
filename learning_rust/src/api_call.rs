// Code to make a basic API call

#[cfg(test)]
mod tests{
    async  fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error>{
    let response: serde_json::Value = reqwest::get(url)
    .await?
    .json::<serde_json::Value>()
    .await?;

    Ok(response)
}

    #[tokio::test]
    async fn tests_calls_fn(){
        let api_url: &str = "https://catfact.ninja/fact";
        let my_res = my_async_call(api_url).await;
        match  my_res {
            Ok(r)=> {
                dbg!(r);
            },
            Err(_) => {
                panic!("Request failed");
            }
        }        
    }
}