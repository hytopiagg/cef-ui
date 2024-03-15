use crate::{ref_counted_ptr, X509Certificate};
use bindings::{
    cef_cert_status_t, cef_cert_status_t_CERT_STATUS_AUTHORITY_INVALID,
    cef_cert_status_t_CERT_STATUS_COMMON_NAME_INVALID,
    cef_cert_status_t_CERT_STATUS_CT_COMPLIANCE_FAILED, cef_cert_status_t_CERT_STATUS_DATE_INVALID,
    cef_cert_status_t_CERT_STATUS_INVALID, cef_cert_status_t_CERT_STATUS_IS_EV,
    cef_cert_status_t_CERT_STATUS_NAME_CONSTRAINT_VIOLATION, cef_cert_status_t_CERT_STATUS_NONE,
    cef_cert_status_t_CERT_STATUS_NON_UNIQUE_NAME,
    cef_cert_status_t_CERT_STATUS_NO_REVOCATION_MECHANISM,
    cef_cert_status_t_CERT_STATUS_PINNED_KEY_MISSING, cef_cert_status_t_CERT_STATUS_REVOKED,
    cef_cert_status_t_CERT_STATUS_REV_CHECKING_ENABLED,
    cef_cert_status_t_CERT_STATUS_SHA1_SIGNATURE_PRESENT,
    cef_cert_status_t_CERT_STATUS_UNABLE_TO_CHECK_REVOCATION,
    cef_cert_status_t_CERT_STATUS_VALIDITY_TOO_LONG, cef_cert_status_t_CERT_STATUS_WEAK_KEY,
    cef_cert_status_t_CERT_STATUS_WEAK_SIGNATURE_ALGORITHM, cef_sslinfo_t
};
use bitflags::bitflags;

bitflags! {
    /// Supported certificate status code values. See net\cert\cert_status_flags.h
    /// for more information. CERT_STATUS_NONE is new in CEF because we use an
    /// enum while cert_status_flags.h uses a typedef and static const variables.
    pub struct CertStatus: cef_cert_status_t {
        const None = cef_cert_status_t_CERT_STATUS_NONE;
        const CommonNameInvalid = cef_cert_status_t_CERT_STATUS_COMMON_NAME_INVALID;
        const DateInvalid = cef_cert_status_t_CERT_STATUS_DATE_INVALID;
        const AuthorityInvalid = cef_cert_status_t_CERT_STATUS_AUTHORITY_INVALID;

        // 1 << 3 is reserved for ERR_CERT_CONTAINS_ERRORS (not useful with WinHTTP).
        const NoRevocationMechanism = cef_cert_status_t_CERT_STATUS_NO_REVOCATION_MECHANISM;
        const UnableToCheckRevocation = cef_cert_status_t_CERT_STATUS_UNABLE_TO_CHECK_REVOCATION;
        const Revoked = cef_cert_status_t_CERT_STATUS_REVOKED;
        const Invalid = cef_cert_status_t_CERT_STATUS_INVALID;
        const WeakSignatureAlgorithm = cef_cert_status_t_CERT_STATUS_WEAK_SIGNATURE_ALGORITHM;

        // 1 << 9 was used for CERT_STATUS_NOT_IN_DNS
        const NonUniqueName = cef_cert_status_t_CERT_STATUS_NON_UNIQUE_NAME;
        const WeakKey = cef_cert_status_t_CERT_STATUS_WEAK_KEY;

        // 1 << 12 was used for CERT_STATUS_WEAK_DH_KEY
        const PinnedKeyMissing = cef_cert_status_t_CERT_STATUS_PINNED_KEY_MISSING;
        const NameConstraintViolation = cef_cert_status_t_CERT_STATUS_NAME_CONSTRAINT_VIOLATION;
        const ValidityTooLong = cef_cert_status_t_CERT_STATUS_VALIDITY_TOO_LONG;

        // Bits 16 to 31 are for non-error statuses.
        const IsEv = cef_cert_status_t_CERT_STATUS_IS_EV;
        const RevCheckingEnabled = cef_cert_status_t_CERT_STATUS_REV_CHECKING_ENABLED;

        // Bit 18 was CERT_STATUS_IS_DNSSEC
        const Sha1SignaturePresent = cef_cert_status_t_CERT_STATUS_SHA1_SIGNATURE_PRESENT;
        const CtComplianceFailed = cef_cert_status_t_CERT_STATUS_CT_COMPLIANCE_FAILED;
    }
}

impl From<cef_cert_status_t> for CertStatus {
    fn from(value: cef_cert_status_t) -> Self {
        Self::from(&value)
    }
}

impl From<&cef_cert_status_t> for CertStatus {
    fn from(value: &cef_cert_status_t) -> Self {
        Self::from_bits_truncate(*value)
    }
}

impl From<CertStatus> for cef_cert_status_t {
    fn from(value: CertStatus) -> Self {
        Self::from(&value)
    }
}

impl From<&CertStatus> for cef_cert_status_t {
    fn from(value: &CertStatus) -> Self {
        value.bits()
    }
}

// Structure representing SSL information.
ref_counted_ptr!(SslInfo, cef_sslinfo_t);

impl SslInfo {
    /// Returns a bitmask containing any and all problems verifying the server
    /// certificate.
    pub fn get_cert_status(&self) -> Option<CertStatus> {
        self.0
            .get_cert_status
            .map(|get_cert_status| unsafe { get_cert_status(self.as_ptr()).into() })
    }

    /// Returns the X.509 certificate.
    pub fn get_x509certificate(&self) -> Option<X509Certificate> {
        self.0
            .get_x509certificate
            .map(|get_x509certificate| unsafe {
                X509Certificate::from_ptr_unchecked(get_x509certificate(self.as_ptr()))
            })
    }
}
