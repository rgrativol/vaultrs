mod common;

use common::VaultServer;
use vaultrs::{
    api::{sys::requests::ListMountsRequest, ResponseWrapper},
    sys::mount,
};

#[tokio::test]
async fn create_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let resp = mount::enable(&server.client, "pki_temp", "pki", None).await;
    assert!(resp.is_ok());

    let mounts = mount::list(&server.client).await;
    assert!(mounts.is_ok());
    assert!(mounts.unwrap().contains_key("pki_temp/"));
}

#[tokio::test]
async fn list_mount() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let resp = mount::list(&server.client).await;
    assert!(resp.is_ok());
    assert!(!resp.unwrap().is_empty());
}

#[tokio::test]
async fn test_wrap() {
    let docker = testcontainers::clients::Cli::default();
    let server = VaultServer::new(&docker);

    let endpoint = ListMountsRequest::builder().build().unwrap();
    let wrap_resp = endpoint.wrap(&server.client).await;
    assert!(wrap_resp.is_ok());

    let wrap_resp = wrap_resp.unwrap();
    let info = wrap_resp.lookup(&server.client).await;
    assert!(info.is_ok());

    let unwrap_resp = wrap_resp.unwrap(&server.client).await;
    assert!(unwrap_resp.is_ok());

    let info = wrap_resp.lookup(&server.client).await;
    assert!(info.is_err());
}
