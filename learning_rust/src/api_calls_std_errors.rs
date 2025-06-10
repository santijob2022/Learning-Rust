// Code to make a basic API call
// Standarize the way of Handling errors 

use std::io::{Error,ErrorKind};

async  fn my_async_call(url: &str) -> Result<serde_json::Value, Error>{
    let response: reqwest::Response = reqwest::get(url)
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve Response"))?;

    let json_response: serde_json::Value = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| Error::new(ErrorKind::Other, "Could not decode to JSON"))?;

    Ok(json_response)
}

#[cfg(test)]
mod tests{
    use  super::*;

    #[tokio::test]
    async fn tests_calls_fn_std_error(){
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