use crate::{
    tls::{cert_compression::CertCompressionAlgorithm, extension::TlsExtension, TlsSettings},
    HttpVersionPref,
};
use boring::{
    error::ErrorStack,
    ssl::{SslConnector, SslCurve, SslMethod, SslVersion},
};
use typed_builder::TypedBuilder;

const CIPHER_LIST: [&str; 15] = [
    "TLS_AES_128_GCM_SHA256",
    "TLS_AES_256_GCM_SHA384",
    "TLS_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
    "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
    "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
    "TLS_RSA_WITH_AES_128_GCM_SHA256",
    "TLS_RSA_WITH_AES_256_GCM_SHA384",
    "TLS_RSA_WITH_AES_128_CBC_SHA",
    "TLS_RSA_WITH_AES_256_CBC_SHA",
];

const SIGALGS_LIST: [&str; 8] = [
    "ecdsa_secp256r1_sha256",
    "rsa_pss_rsae_sha256",
    "rsa_pkcs1_sha256",
    "ecdsa_secp384r1_sha384",
    "rsa_pss_rsae_sha384",
    "rsa_pkcs1_sha384",
    "rsa_pss_rsae_sha512",
    "rsa_pkcs1_sha512",
];

pub const NEW_CURVES: &[SslCurve] = &[
    SslCurve::X25519_KYBER768_DRAFT00,
    SslCurve::X25519,
    SslCurve::SECP256R1,
    SslCurve::SECP384R1,
];

#[derive(TypedBuilder)]
pub struct ChromeTlsSettings<'a> {
    // TLS curves
    #[builder(default, setter(into))]
    curves: Option<&'a [SslCurve]>,

    // TLS sigalgs list
    #[builder(default = &SIGALGS_LIST)]
    sigalgs_list: &'a [&'a str],

    // TLS cipher list
    #[builder(default = &CIPHER_LIST)]
    cipher_list: &'a [&'a str],

    // TLS application_settings extension
    #[builder(default = true, setter(into))]
    application_settings: bool,

    // TLS enable ech grease, https://chromestatus.com/feature/6196703843581952
    #[builder(default = false, setter(into))]
    enable_ech_grease: bool,

    // TLS permute extensions
    #[builder(default = false, setter(into))]
    permute_extensions: bool,

    // TLS pre_shared_key extension
    #[builder(default = false, setter(into))]
    pre_shared_key: bool,
}

impl TryInto<TlsSettings> for ChromeTlsSettings<'_> {
    type Error = ErrorStack;

    fn try_into(self) -> Result<TlsSettings, Self::Error> {
        let sigalgs_list = self.sigalgs_list.join(":");
        let cipher_list = self.cipher_list.join(":");
        let curves = self
            .curves
            .map(|c| c.to_owned())
            .unwrap_or_else(|| vec![SslCurve::X25519, SslCurve::SECP256R1, SslCurve::SECP384R1]);

        let connector = Box::new(move || {
            let mut builder = SslConnector::builder(SslMethod::tls_client())?;
            builder.set_grease_enabled(true);
            builder.enable_ocsp_stapling();
            builder.set_curves(&curves)?;
            builder.set_sigalgs_list(&sigalgs_list)?;
            builder.set_cipher_list(&cipher_list)?;
            builder.enable_signed_cert_timestamps();
            builder.set_min_proto_version(Some(SslVersion::TLS1_2))?;
            builder.set_max_proto_version(Some(SslVersion::TLS1_3))?;
            builder.set_permute_extensions(self.permute_extensions);
            builder.configure_add_cert_compression_alg(CertCompressionAlgorithm::Brotli)
        });

        Ok(TlsSettings::builder()
            .connector(connector)
            .http_version_pref(HttpVersionPref::All)
            .permute_extensions(self.permute_extensions)
            .pre_shared_key(self.pre_shared_key)
            .enable_ech_grease(self.enable_ech_grease)
            .application_settings(self.application_settings)
            .build())
    }
}
