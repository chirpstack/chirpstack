use std::ops::Add;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::ec::{EcGroup, EcKey};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectKeyIdentifier,
};
use openssl::x509::{X509NameBuilder, X509Ref, X509ReqBuilder, X509};
use tokio::fs;
use uuid::Uuid;

use crate::config;
use lrwn::EUI64;

// Based on:
// https://github.com/sfackler/rust-openssl/blob/master/openssl/examples/mk_certs.rs
// TODO: wrap in tokio?
async fn generate_client_certificate(
    id: &str,
    ttl: SystemTime,
    ca_cert: &X509Ref,
    ca_key_pair: &PKeyRef<Private>,
) -> Result<(X509, PKey<Private>)> {
    let ttl_unix = ttl.duration_since(UNIX_EPOCH)?;

    let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1)?;
    let ec = EcKey::generate(&group)?;
    let key_pair = PKey::from_ec_key(ec)?;

    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_pubkey(&key_pair)?;

    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("CN", id)?;
    let x509_name = x509_name.build();
    req_builder.set_subject_name(&x509_name)?;

    req_builder.sign(&key_pair, MessageDigest::sha256())?;
    let req = req_builder.build();

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(2)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(req.subject_name())?;
    cert_builder.set_issuer_name(ca_cert.subject_name())?;
    cert_builder.set_pubkey(&key_pair)?;
    let not_before = Asn1Time::days_from_now(0)?;
    cert_builder.set_not_before(&not_before)?;
    let not_after = Asn1Time::from_unix(ttl_unix.as_secs().try_into()?)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().build()?)?;
    cert_builder.append_extension(KeyUsage::new().critical().digital_signature().build()?)?;

    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    let auth_key_identifier = AuthorityKeyIdentifier::new()
        .keyid(false)
        .issuer(false)
        .build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension(auth_key_identifier)?;

    cert_builder.sign(ca_key_pair, MessageDigest::sha256())?;
    let cert = cert_builder.build();

    Ok((cert, key_pair))
}

// This returns the CA, certificate and private-key as PEM encoded strings.
pub async fn client_cert_for_gateway_id(
    gateway_id: &EUI64,
) -> Result<(SystemTime, String, String, String)> {
    let conf = config::get();

    let ca_cert = fs::read(&conf.gateway.ca_cert)
        .await
        .context("Read gateway ca_cert")?;
    let ca_key = fs::read(&conf.gateway.ca_key)
        .await
        .context("Read gateway ca_key")?;

    let ca_cert = X509::from_pem(&ca_cert).context("Parse gateway ca_cert")?;
    let ca_key = PKey::private_key_from_pem(&ca_key).context("Parse gateway ca_key")?;

    let ttl = SystemTime::now().add(conf.gateway.client_cert_lifetime);

    let (cert, p_key) =
        generate_client_certificate(&gateway_id.to_string(), ttl, &ca_cert, &ca_key)
            .await
            .context("Generate client-certificate")?;

    let ca_pem = ca_cert.to_pem()?;
    let cert_pem = cert.to_pem()?;
    let p_key_pem = p_key.private_key_to_pem_pkcs8()?;

    Ok((
        ttl,
        String::from_utf8(ca_pem).unwrap(),
        String::from_utf8(cert_pem).unwrap(),
        String::from_utf8(p_key_pem).unwrap(),
    ))
}

pub async fn client_cert_for_application_id(
    application_id: &Uuid,
) -> Result<(SystemTime, String, String, String)> {
    let conf = config::get();

    let ca_cert = fs::read(&conf.integration.mqtt.client.ca_cert)
        .await
        .context("Read mqtt ca_cert")?;
    let ca_key = fs::read(&conf.integration.mqtt.client.ca_key)
        .await
        .context("Read mqtt ca_key")?;

    let ca_cert = X509::from_pem(&ca_cert).context("Parse gateway ca_cert")?;
    let ca_key = PKey::private_key_from_pem(&ca_key).context("Parse gateway ca_key")?;

    let ttl = SystemTime::now().add(conf.integration.mqtt.client.client_cert_lifetime);

    let (cert, p_key) =
        generate_client_certificate(&application_id.to_string(), ttl, &ca_cert, &ca_key)
            .await
            .context("Generate client-certificate")?;

    let ca_pem = ca_cert.to_pem()?;
    let cert_pem = cert.to_pem()?;
    let p_key_pem = p_key.private_key_to_pem_pkcs8()?;

    Ok((
        ttl,
        String::from_utf8(ca_pem).unwrap(),
        String::from_utf8(cert_pem).unwrap(),
        String::from_utf8(p_key_pem).unwrap(),
    ))
}
