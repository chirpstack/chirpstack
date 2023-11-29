use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};

// Return root certificates, optionally with the provided ca_file appended.
pub fn get_root_certs(ca_file: Option<String>) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs()?;
    let certs: Vec<_> = certs.into_iter().map(|cert| cert.0).collect();
    roots.add_parsable_certificates(&certs);

    if let Some(ca_file) = &ca_file {
        let f = File::open(ca_file).context("Open CA certificate")?;
        let mut reader = BufReader::new(f);
        let certs = rustls_pemfile::certs(&mut reader)?;
        for cert in certs
            .into_iter()
            .map(rustls::Certificate)
            .collect::<Vec<_>>()
        {
            roots.add(&cert)?;
        }
    }

    Ok(roots)
}

pub fn load_cert(cert_file: &str) -> Result<Vec<rustls::Certificate>> {
    let f = File::open(cert_file).context("Open TLS certificate")?;
    let mut reader = BufReader::new(f);
    let certs = rustls_pemfile::certs(&mut reader)?;
    let certs = certs
        .into_iter()
        .map(rustls::Certificate)
        .collect::<Vec<_>>();
    Ok(certs)
}

pub fn load_key(key_file: &str) -> Result<rustls::PrivateKey> {
    let f = File::open(key_file).context("Open private key")?;
    let mut reader = BufReader::new(f);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
    match keys.len() {
        0 => Err(anyhow!("No private key found")),
        1 => Ok(rustls::PrivateKey(keys.remove(0))),
        _ => Err(anyhow!("More than one private key found")),
    }
}
