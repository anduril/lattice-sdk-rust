# Anduril SDK Rust

The official [Anduril](https://www.anduril.com/) client library.

## Requirements

[Rust installed](https://doc.rust-lang.org/beta/book/ch01-01-installation.html#installing-rustup-on-linux-or-macos). Code samples were written in 1.79.0 

## Installation

### Authentication

To authenticate with the Github package repository, you will need to generate a [personal access token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-personal-access-token-classic). This should have at least `read:packages` scope. Please keep the token safe for the next stage of the setup procedure.

Authenticate with the Github CLI:
```
gh auth setup-git
```

### Cargo
Modify `Cargo.toml`

```
[dependencies]
anduril-sdk = { git = "https://github.com/anduril/anduril-rust" }
```

```
cargo fetch
```
## Usage

Once we've installed the modules, let's create a channel and generate an
instance of an EM client. **Note:** To use gRPC's in Rust, you must install the
[`tonic`](https://docs.rs/tonic/latest/tonic/index.html) crate

main.rs

```rust
use anduril_sdk::anduril::entitymanager::v1::entity_manager_api_client::EntityManagerApiClient;
use anduril_sdk::anduril::entitymanager::v1::GetEntityRequest;
use anduril_sdk::anduril::entitymanager::v1::GetEntityResponse;

use tonic::metadata::MetadataValue;
use tonic::transport::ClientTlsConfig;
use tonic::transport::Channel;
use tonic::Request;
use tonic::Status;

async fn get_entity() -> Result<GetEntityResponse, Status>{
    let token = "eyJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MjMyMjYwMjEsImlzcyI6ImFuZHVyaWwiLCJqdGkiOiI5Y2U2MTJlNi0xMGFiLTQ2ZGItYTU3My05ZDkxOGQyNzE4NjEiLCJuYmYiOjE3MjI2MjEyMTEsInN1YiI6InVzZXIvNzU1YjcwYmItOTcxYS00ZWYwLTgwOTYtMzY3NDQ4MjE3ZmJkIn0.DlsBKfA4kvUQGvmRYVE5ZymeB6M5qAsWXXXgCM_QAzo";
    let bearer_token = format!("Bearer {}", token);
    let header_value: MetadataValue<_> = bearer_token.parse().map_err(|_| Status::internal("Invalid Bearer Token"))?;
    let tls_config = ClientTlsConfig::new().with_native_roots();
    let http_endpoint = format!("https://desert-guardian.anduril.com");
    let registration_channel = Channel::from_shared(http_endpoint)
      .map_err(|e| Status::invalid_argument(format!("Invalid HTTP endpoint: {}", e)))?
      .tls_config(tls_config)
      .map_err(|_| Status::internal("TLS configuration failed"))?
      .connect()
      .await
      .map_err(|e| Status::unavailable(format!("Failed to connect: {}", e)))?;

    let mut em_client = EntityManagerApiClient::with_interceptor(registration_channel, |mut req: Request<()>| {
       req.metadata_mut().insert("authorization",  header_value.clone());
       Ok(req)
    });
    
    let response = em_client.get_entity(GetEntityRequest {
       entity_id: String::from("<ENTITY ID>")
    }).await?;
    Ok(response.into_inner())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let entity_result = get_entity().await;

   match entity_result {
        Ok(entity) => println!("{:?}", entity),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
```

## Support

For support with this library please [file an issue](https://github.com/anduril/anduril-rust/issues/new) or reach out to your Anduril representative. 



