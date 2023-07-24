use std::collections::HashMap;
use anyhow::Result;
use tokio::test;
use reqwest::{get};

#[test]
async fn test_root() -> Result<()> {
    let resp = get("http://localhost:3000/api/ready?message=Dozie")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}


#[test]
async fn test_root_with_path() -> Result<()>{
    let resp = get("http://localhost:3000/api/ready/Dozie")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:?}", resp);

    Ok(())
}