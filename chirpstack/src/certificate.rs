use std::time::SystemTime;

use anyhow::{Context, Result};
use rcgen::{
    Certificate, CertificateParams, DnType, ExtendedKeyUsagePurpose, KeyPair, KeyUsagePurpose,
};
use rsa::{
    pkcs1::DecodeRsaPrivateKey,
    pkcs8::{EncodePrivateKey, LineEnding},
    RsaPrivateKey,
};
use tokio::fs;
use uuid::Uuid;

use crate::config;
use lrwn::EUI64;

fn gen_client_cert(id: &str, not_before: SystemTime, not_after: SystemTime) -> Result<Certificate> {
    let mut params = CertificateParams::new(vec![id.to_string()]);
    params
        .distinguished_name
        .push(DnType::CommonName, id.to_string());
    params.use_authority_key_identifier_extension = true;
    params.not_before = not_before.into();
    params.not_after = not_after.into();
    params.key_usages.push(KeyUsagePurpose::DigitalSignature);
    params
        .extended_key_usages
        .push(ExtendedKeyUsagePurpose::ClientAuth);

    Ok(Certificate::from_params(params)?)
}

async fn get_ca_cert(ca_cert_file: &str, ca_key_file: &str) -> Result<Certificate> {
    let ca_cert_s = fs::read_to_string(ca_cert_file)
        .await
        .context("Read gateway ca_cert")?;
    let ca_key_s = fs::read_to_string(ca_key_file)
        .await
        .context("Read gateway ca_key")?;
    let ca_key_s = private_key_to_pkcs8(&ca_key_s)?;

    let ca_key = KeyPair::from_pem(&ca_key_s).context("Parse gateway CA key")?;
    let params = CertificateParams::from_ca_cert_pem(&ca_cert_s, ca_key)
        .context("Parse gateway CA certificate")?;

    // Workaround for:
    // https://github.com/rustls/rcgen/issues/193
    let ca_key =
        KeyPair::from_pem_and_sign_algo(&ca_key_s, params.alg).context("Parse gateway CA key")?;
    let params = CertificateParams::from_ca_cert_pem(&ca_cert_s, ca_key)
        .context("Parse gateway CA certificate")?;

    Certificate::from_params(params).context("Init Certificate struct")
}

// This returns the CA, certificate and private-key as PEM encoded strings.
pub async fn client_cert_for_gateway_id(
    gateway_id: &EUI64,
) -> Result<(SystemTime, String, String, String)> {
    let conf = config::get();
    let ca_cert = get_ca_cert(&conf.gateway.ca_cert, &conf.gateway.ca_key)
        .await
        .context("Get CA cert")?;
    let not_before = SystemTime::now();
    let not_after = SystemTime::now() + conf.gateway.client_cert_lifetime;
    let gw_cert = gen_client_cert(&gateway_id.to_string(), not_before, not_after)
        .context("Generate client certificate")?;

    Ok((
        not_after,
        ca_cert.serialize_pem().context("Serialize CA cert")?,
        gw_cert
            .serialize_pem_with_signer(&ca_cert)
            .context("Serialize client cert")?,
        gw_cert.serialize_private_key_pem(),
    ))
}

pub async fn client_cert_for_application_id(
    application_id: &Uuid,
) -> Result<(SystemTime, String, String, String)> {
    let conf = config::get();
    let ca_cert = get_ca_cert(
        &conf.integration.mqtt.client.ca_cert,
        &conf.integration.mqtt.client.ca_key,
    )
    .await?;
    let not_before = SystemTime::now();
    let not_after = SystemTime::now() + conf.integration.mqtt.client.client_cert_lifetime;
    let app_cert = gen_client_cert(&application_id.to_string(), not_before, not_after)?;

    Ok((
        not_after,
        ca_cert.serialize_pem()?,
        app_cert.serialize_pem_with_signer(&ca_cert)?,
        app_cert.serialize_private_key_pem(),
    ))
}

fn private_key_to_pkcs8(pem: &str) -> Result<String> {
    if pem.contains("RSA PRIVATE KEY") {
        let pkey = RsaPrivateKey::from_pkcs1_pem(pem).context("Read RSA PKCS#1")?;
        let pkcs8_pem = pkey.to_pkcs8_pem(LineEnding::default())?;
        Ok(pkcs8_pem.as_str().to_owned())
    } else {
        Ok(pem.to_string())
    }
}
