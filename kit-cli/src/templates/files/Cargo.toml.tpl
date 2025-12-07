[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"
description = "{description}"
{authors_line}
[dependencies]
kit = {{ package = "kit-rs", version = "0.1" }}
tokio = {{ version = "1", features = ["full"] }}
