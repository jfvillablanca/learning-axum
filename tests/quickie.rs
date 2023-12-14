use anyhow::Result;

#[tokio::test]
async fn quickie() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:6969")?;

    hc.do_get("/").await?.print().await?;

    Ok(())
}
