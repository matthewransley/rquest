use crate::tls::extension::{Extension, OkHttpExtension, SslExtension};
use crate::tls::{Http2Settings, SslBuilderSettings};
use crate::tls::{ImpersonateSettings, TlsResult};
use http::{
    header::{ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, USER_AGENT},
    HeaderMap, HeaderValue,
};

pub(crate) fn get_settings(
    settings: ImpersonateSettings,
    headers: &mut HeaderMap,
) -> TlsResult<SslBuilderSettings> {
    init_headers(headers);
    Ok(SslBuilderSettings {
        ssl_builder: OkHttpExtension::builder()?.configure_cipher_list(&[
            "TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384",
            "TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256",
            "TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA",
            "TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA",
            "TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA",
            "TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA",
            "TLS_RSA_WITH_AES_128_GCM_SHA256",
            "TLS_RSA_WITH_AES_256_GCM_SHA384",
            "TLS_RSA_WITH_AES_128_CBC_SHA",
            "TLS_RSA_WITH_AES_256_CBC_SHA",
            "TLS_RSA_WITH_3DES_EDE_CBC_SHA",
        ])?,
        pre_shared_key: settings.pre_share_key,
        http2: Http2Settings {
            initial_stream_window_size: Some(16777216),
            initial_connection_window_size: Some(16777216),
            max_concurrent_streams: None,
            max_header_list_size: None,
            header_table_size: None,
            enable_push: None,
            headers_priority: settings.headers_priority,
            headers_pseudo_order: settings.headers_pseudo_order,
            settings_order: settings.settings_order,
        },
    })
}

fn init_headers(headers: &mut HeaderMap) {
    headers.insert(ACCEPT, HeaderValue::from_static("*/*"));
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("de-DE,de;q=0.9,en-US;q=0.8,en;q=0.7"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("MaiMemo/4.4.50_639 okhttp/3.9 Android/5.0 Channel/WanDouJia Device/alps+M8+Emulator (armeabi-v7a) Screen/4.44 Resolution/480x800 DId/aa6cde19def3806806d5374c4e5fd617 RAM/0.94 ROM/4.91 Theme/Day"));
    headers.insert(
        ACCEPT_ENCODING,
        HeaderValue::from_static("gzip, deflate, br"),
    );
}
