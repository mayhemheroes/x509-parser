#![no_main]
use libfuzzer_sys::fuzz_target;
use x509_parser::prelude::*;

fuzz_target!(|data: &[u8]| {
    if data.len() > 1 {
        let opt = data[0];
        let mut fuzz_data = &data[1..];
        match opt {
            0=> {
                X509Certificate::from_der(fuzz_data);
            },
            1=> {
                CertificateRevocationList::from_der(fuzz_data);
            },
            _=> ()
        }
    }
});
