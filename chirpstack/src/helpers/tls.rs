use std::fs::File;
use std::io::BufReader;

use anyhow::{Context, Result};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio::fs;

// Return root certificates, optionally with the provided ca_file appended.
pub fn get_root_certs(ca_file: Option<String>) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
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

pub async fn load_cert(cert_file: &str) -> Result<Vec<CertificateDer<'static>>> {
    let cert_s = fs::read_to_string(cert_file)
        .await
        .context("Read TLS certificate")?;
    let mut cert_b = cert_s.as_bytes();
    let certs = rustls_pemfile::certs(&mut cert_b);
    let mut out = Vec::new();
    for cert in certs {
        out.push(cert?.into_owned());
    }
    Ok(out)
}

pub async fn load_key(key_file: &str) -> Result<PrivateKeyDer<'static>> {
    let key_s = fs::read_to_string(key_file)
        .await
        .context("Read private key")?;
    let key_s = private_key_to_pkcs8(&key_s)?;
    let mut key_b = key_s.as_bytes();
    let mut keys = rustls_pemfile::pkcs8_private_keys(&mut key_b);
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

pub fn private_key_to_pkcs8(pem: &str) -> Result<String> {
    if pem.contains("RSA PRIVATE KEY") {
        use rsa::{
            pkcs1::DecodeRsaPrivateKey,
            pkcs8::{EncodePrivateKey, LineEnding},
            RsaPrivateKey,
        };

        let pkey = RsaPrivateKey::from_pkcs1_pem(pem).context("Read RSA PKCS#1")?;
        let pkcs8_pem = pkey.to_pkcs8_pem(LineEnding::default())?;
        Ok(pkcs8_pem.as_str().to_owned())
    } else if pem.contains("EC PRIVATE KEY") {
        use sec1::{
            der::{Decode, Encode, EncodePem},
            pkcs8::{AlgorithmIdentifierRef, PrivateKeyInfo},
            EcPrivateKey, LineEnding,
        };

        // Get a SEC1 ECPrivateKey from the PEM string input
        let pem = pem::parse(pem).context("Parse PEM string")?;
        let pkey =
            EcPrivateKey::from_der(pem.contents()).context("Decode PEM into SEC1 ECPrivateKey")?;

        // Retrieve the curve name from the decoded private key's parameters
        let params_oid = pkey.parameters.and_then(|params| params.named_curve());

        // Get the proper types to construct a PKCS#8 PrivateKeyInfo
        let private_key = &pkey.to_der()?;
        let algorithm = AlgorithmIdentifierRef {
            oid: sec1::ALGORITHM_OID,
            parameters: params_oid.as_ref().map(Into::into),
        };

        let pkcs8 = PrivateKeyInfo {
            algorithm,
            private_key,
            public_key: None,
        };

        Ok(pkcs8.to_pem(LineEnding::default())?)
    } else {
        Ok(pem.to_string())
    }
}
