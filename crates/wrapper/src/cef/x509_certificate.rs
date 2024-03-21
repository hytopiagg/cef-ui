use crate::{ref_counted_ptr, BinaryValue, CefString, CefStringList, CefTime};
use bindings::{cef_x509cert_principal_t, cef_x509certificate_t};
use chrono::{DateTime, Utc};

// Structure representing the issuer or subject field of an X.509 certificate.
ref_counted_ptr!(X509CertPrincipal, cef_x509cert_principal_t);

impl X509CertPrincipal {
    /// Returns a name that can be used to represent the issuer. It tries in this
    /// order: Common Name (CN), Organization Name (O) and Organizational Unit
    /// Name (OU) and returns the first non-NULL one found.
    pub fn get_display_name(&self) -> Option<String> {
        self.0
            .get_display_name
            .map(|get_display_name| {
                let s = unsafe { get_display_name(self.as_ptr()) };

                CefString::from_userfree_ptr_unchecked(s).into()
            })
    }

    /// Returns the common name.
    pub fn get_common_name(&self) -> Option<String> {
        self.0
            .get_common_name
            .map(|get_common_name| {
                let s = unsafe { get_common_name(self.as_ptr()) };

                CefString::from_userfree_ptr_unchecked(s).into()
            })
    }

    /// Returns the locality name.
    pub fn get_locality_name(&self) -> Option<String> {
        self.0
            .get_locality_name
            .map(|get_locality_name| {
                let s = unsafe { get_locality_name(self.as_ptr()) };

                CefString::from_userfree_ptr_unchecked(s).into()
            })
    }

    /// Returns the state or province name.
    pub fn get_state_or_province_name(&self) -> Option<String> {
        self.0
            .get_state_or_province_name
            .map(|get_state_or_province_name| {
                let s = unsafe { get_state_or_province_name(self.as_ptr()) };

                CefString::from_userfree_ptr_unchecked(s).into()
            })
    }

    /// Returns the country name.
    pub fn get_country_name(&self) -> Option<String> {
        self.0
            .get_country_name
            .map(|get_country_name| {
                let s = unsafe { get_country_name(self.as_ptr()) };

                CefString::from_userfree_ptr_unchecked(s).into()
            })
    }

    /// Retrieve the list of organization names.
    pub fn get_organization_names(&self) -> Vec<String> {
        self.0
            .get_organization_names
            .map(|get_organization_names| {
                let mut list = CefStringList::new();

                unsafe {
                    get_organization_names(self.as_ptr(), list.as_mut_ptr());
                }

                list.into()
            })
            .unwrap_or_default()
    }

    /// Retrieve the list of organization unit names.
    pub fn get_organization_unit_names(&self) -> Vec<String> {
        self.0
            .get_organization_unit_names
            .map(|get_organization_unit_names| {
                let mut list = CefStringList::new();

                unsafe {
                    get_organization_unit_names(self.as_ptr(), list.as_mut_ptr());
                }

                list.into()
            })
            .unwrap_or_default()
    }
}

// Structure representing a X.509 certificate.
ref_counted_ptr!(X509Certificate, cef_x509certificate_t);

impl X509Certificate {
    /// Returns the subject of the X.509 certificate. For HTTPS server
    /// certificates this represents the web server.  The common name of the
    /// subject should match the host name of the web server.
    pub fn get_subject(&self) -> Option<X509CertPrincipal> {
        self.0
            .get_subject
            .map(|get_subject| unsafe {
                X509CertPrincipal::from_ptr_unchecked(get_subject(self.as_ptr()))
            })
    }

    /// Returns the issuer of the X.509 certificate.
    pub fn get_issuer(&self) -> Option<X509CertPrincipal> {
        self.0
            .get_issuer
            .map(|get_issuer| unsafe {
                X509CertPrincipal::from_ptr_unchecked(get_issuer(self.as_ptr()))
            })
    }

    /// Returns the DER encoded serial number for the X.509 certificate. The value
    /// possibly includes a leading 00 byte.
    pub fn get_serial_number(&self) -> Option<BinaryValue> {
        self.0
            .get_serial_number
            .map(|get_serial_number| unsafe {
                BinaryValue::from_ptr_unchecked(get_serial_number(self.as_ptr()))
            })
    }

    /// Returns the date before which the X.509 certificate is invalid.
    /// CefBaseTime.GetTimeT() will return 0 if no date was specified.
    pub fn get_valid_start(&self) -> Option<DateTime<Utc>> {
        self.0
            .get_valid_start
            .and_then(|get_valid_start| {
                let base_time = unsafe { get_valid_start(self.as_ptr()) };

                CefTime::try_from(base_time)
                    .ok()
                    .map(CefTime::into)
            })
    }

    /// Returns the date after which the X.509 certificate is invalid.
    /// CefBaseTime.GetTimeT() will return 0 if no date was specified.
    pub fn get_valid_expiry(&self) -> Option<DateTime<Utc>> {
        self.0
            .get_valid_expiry
            .and_then(|get_valid_expiry| {
                let base_time = unsafe { get_valid_expiry(self.as_ptr()) };

                CefTime::try_from(base_time)
                    .ok()
                    .map(CefTime::into)
            })
    }

    /// Returns the DER encoded data for the X.509 certificate.
    pub fn get_derencoded(&self) -> Option<BinaryValue> {
        self.0
            .get_derencoded
            .map(|get_derencoded| unsafe {
                BinaryValue::from_ptr_unchecked(get_derencoded(self.as_ptr()))
            })
    }

    /// Returns the PEM encoded data for the X.509 certificate.
    pub fn get_pemencoded(&self) -> Option<BinaryValue> {
        self.0
            .get_pemencoded
            .map(|get_pemencoded| unsafe {
                BinaryValue::from_ptr_unchecked(get_pemencoded(self.as_ptr()))
            })
    }

    /// Returns the number of certificates in the issuer chain. If 0, the
    /// certificate is self-signed.
    pub fn get_issuer_chain_size(&self) -> Option<usize> {
        self.0
            .get_issuer_chain_size
            .map(|get_issuer_chain_size| unsafe { get_issuer_chain_size(self.as_ptr()) })
    }

    /// Returns the DER encoded data for the certificate issuer chain. If we
    /// failed to encode a certificate in the chain it is still present in the
    /// array but is an NULL string.
    pub fn get_derencoded_issuer_chain(&self, _chain: &mut Vec<BinaryValue>) {
        unimplemented!("This is not easy to implement safely.")
    }

    /// Returns the PEM encoded data for the certificate issuer chain. If we
    /// failed to encode a certificate in the chain it is still present in the
    /// array but is an NULL string.
    pub fn get_pemencoded_issuer_chain(&self, _chain: &mut Vec<BinaryValue>) {
        unimplemented!("This is not easy to implement safely.")
    }
}
