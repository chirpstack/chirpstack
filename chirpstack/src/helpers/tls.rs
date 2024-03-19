use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};

// Return root certificates, optionally with the provided ca_file appended.
pub fn get_root_certs(ca_file: Option<String>) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs()? {
        roots.add(cert)?;
    }

    if let Some(ca_file) = &ca_file {
        let f = File::open(ca_file).context("Open CA certificate")?;
        let mut reader = BufReader::new(f);
        let certs = rustls_pemfile::certs(&mut reader);
        for cert in certs.flatten() {
            roots.add(cert)?;
        }
    }

    Ok(roots)
}

pub fn load_cert(cert_file: &str) -> Result<Vec<CertificateDer<'static>>> {
    let f = File::open(cert_file).context("Open TLS certificate")?;
    let mut reader = BufReader::new(f);
    let certs = rustls_pemfile::certs(&mut reader);
    let mut out = Vec::new();
    for cert in certs {
        out.push(cert?.into_owned());
    }
    Ok(out)
}

pub fn load_key(key_file: &str) -> Result<PrivateKeyDer<'static>> {
    let f = File::open(key_file).context("Open private key")?;
    let mut reader = BufReader::new(f);
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader);
    if let Some(key) = keys.next() {
        match key {
            Ok(v) => return Ok(PrivateKeyDer::Pkcs8(v.clone_key())),
            Err(e) => {
                return Err(anyhow!("Error parsing private key, error: {}", e));
            }
        }
    }

    Err(anyhow!("No private key found"))
}
