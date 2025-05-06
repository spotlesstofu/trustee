use std::path::{Path, PathBuf};

use anyhow::{bail, Ok, Result};
use openssl::asn1::Asn1Time;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::{X509NameBuilder, X509};

fn safe_write(path: &Path, contents: Vec<u8>) -> Result<()> {
    if path.exists() {
        bail!("refusing to overwrite file: {:?}", path);
    } else {
        std::fs::write(&path, contents)?;
        Ok(())
    }
}

/// Writes the private and public keys to separate PEM files.
/// `path` is the base path where the keys will be saved.
/// Returns the paths to the private and public key.
fn write_pem(base_path: &Path, private_key: &PKey<Private>) -> Result<(PathBuf, PathBuf)> {
    let private_path = base_path.with_extension("pem");
    let private_pem = private_key.private_key_to_pem_pkcs8()?;
    safe_write(&private_path, private_pem)?;

    let public_path = base_path.with_extension("pub");
    let public_pem = private_key.public_key_to_pem()?;
    safe_write(&public_path, public_pem)?;

    Ok((private_path, public_path))
}

/// Creates a key pair and writes to separate files.
pub fn new_auth_key_pair(base_path: &Path) -> Result<(PathBuf, PathBuf)> {
    let private_key = PKey::generate_ed25519()?;
    write_pem(base_path, &private_key)
}

/// Generates an HTTPS private key.
/// `path` is the directory where the key will be stored.
/// Returns the path to the private key.
pub fn new_https_key_pair(base_path: &Path) -> Result<(PathBuf, PathBuf)> {
    // Generate RSA private key
    let rsa = Rsa::generate(2048)?;
    let private_key = PKey::from_rsa(rsa)?;

    // Write private key to file
    let path = base_path.join("https_key");
    write_pem(path.as_path(), &private_key)
}

/// Generates a self-signed HTTPS certificate.
/// `path` is the directory where the certificate will be stored.
/// `private_key` is the private key used to sign the certificate.
/// Returns the path to the certificate.
pub fn new_https_certificate(path: &Path, private_key: &PathBuf) -> Result<PathBuf> {
    // Load the private key from the provided path
    let private_key = {
        let private_key_data = std::fs::read(private_key)?;
        PKey::private_key_from_pem(&private_key_data)?
    };

    let name = {
        let mut name_builder = X509NameBuilder::new()?;
        name_builder.append_entry_by_text("CN", "localhost")?;
        name_builder.build()
    };

    // Create a self-signed certificate
    let certificate = {
        let mut builder = X509::builder()?;
        builder.set_version(2)?;
        builder.set_subject_name(name.as_ref())?;
        builder.set_issuer_name(name.as_ref())?;
        builder.set_not_before(Asn1Time::days_from_now(0)?.as_ref())?;
        builder.set_not_after(Asn1Time::days_from_now(365)?.as_ref())?;
        builder.set_pubkey(private_key.as_ref())?;
        builder.set_serial_number(
            openssl::asn1::Asn1Integer::from_bn(openssl::bn::BigNum::from_u32(1)?.as_ref())?
                .as_ref(),
        )?;
        builder.sign(private_key.as_ref(), MessageDigest::sha256())?;
        builder.build()
    };

    // Write certificate to file
    let certificate_path = path.join("https_cert.pem");
    std::fs::write(&certificate_path, certificate.to_pem()?)?;

    Ok(certificate_path)
}
