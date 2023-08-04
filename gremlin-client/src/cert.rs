// use rustls::client::{ServerCertVerified, ServerCertVerifier};

use rustls::client::{ServerCertVerified, ServerCertVerifier};

pub struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    // fn verify_server_cert(
    //     &self,
    //     roots: &rustls::RootCertStore,
    //     presented_certs: &[rustls::Certificate],
    //     dns_name: webpki::DNSNameRef,
    //     ocsp_response: &[u8],
    // ) -> Result<ServerCertVerified, TLSError> {
    //     Ok(ServerCertVerified::assertion())
    // }
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}
