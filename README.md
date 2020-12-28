[![CI](https://github.com/PepaRokos/fakturoid/workflows/Rust/badge.svg)](https://github.com/PepaRokos/fakturoid/actions?query=workflow%3ARust)
[![crates.io](https://img.shields.io/crates/v/fakturoid.svg)](https://crates.io/crates/fakturoid)
[![Documentation](https://docs.rs/fakturoid/badge.svg)](https://docs.rs/fakturoid)


# fakturoid.cz Rust API

The Rust interface to online accounting service [Fakturoid](http://fakturoid.cz/).

This library is developed and maintained by Josef Rokos ([pepa@bukova.info](mailto:pepa@bukova.info)).
It is unoficial and no support from Fakturoid team can be claimed.

## Features

- Account detail
- Subjects: create, update, delete, list, filters and fulltext
- Invoices: create, update, delete, list, filters and fulltext, invoice actions

## Examples

This library is asynchronous, so you will need [Tokio](https://tokio.rs) to execute library methods.
`Cargo.toml` could look like this:

```toml
[dependencies]
fakturoid = "0.1.2"
tokio = {version = "0.2", features = ["full"]}
```

### Get object detail

```rust
use fakturoid::models::Subject;
use tokio::prelude::*;
use fakturoid::client::Fakturoid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Fakturoid::new(
        "your@account.cz",
        "c08950e6s70f982dbca56295b123eff987237b9",
        "yourslug",
        Some("Rust Test API client (developer@address.mail)")
    );

    let subject = cli.detail::<Subject>(11223344).await?;
    println!("{:?}", subject);

    Ok(())
}
```

### Update object

All fields of model structs has type `Option<...>`. If some field will have `None` value this field will not be serialized,
so you can create new struct of given type and set only fields which you want update:

```rust
use fakturoid::models::Subject;
use tokio::prelude::*;
use fakturoid::client::Fakturoid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Fakturoid::new(
        "your@account.cz",
        "c08950e6s70f982dbca56295b123eff987237b9",
        "yourslug",
        Some("Rust Test API client (developer@address.mail)")
    );

    let mut subject = Subject::default(); // initialize all fields to None
    subject.name = Some("Some other name".to_string());
    let subject = cli.update(11223344, subject).await?;
    println!("{:?}", subject);

    Ok(())
}
```

Updated object will be returned in case of success. You can create new objects in similar way:

```rust
use fakturoid::models::Subject;
use tokio::prelude::*;
use fakturoid::client::Fakturoid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Fakturoid::new(
        "your@account.cz",
        "c08950e6s70f982dbca56295b123eff987237b9",
        "yourslug",
        Some("Rust Test API client (developer@address.mail)")
    );

    let mut subject = Subject::default(); // initialize all fields to None
    subject.name = Some("Some other name".to_string()); // only mandatory fields can be set
    let subject = cli.create(11223344, subject).await?;
    println!("{:?}", subject);

    Ok(())
}
```

### Get list of subjects

Fakturoid.cz API returns all lists with more than 20 items in form pages of 20 items. This is represented by `PagedResponse`
struct. Pages can be accessed through methods of this struct:

```rust
    ...
    let invoices = cli.list::<Invoice>(None).await?;
    println!("{:?}", invoices.data()[0]);
    let invoices = invoices.next_page().await?;
    ...
```
