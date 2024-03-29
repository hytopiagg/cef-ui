use crate::{ref_counted_ptr, try_c, X509Certificate};
use anyhow::Result;
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
    cef_cert_status_t_CERT_STATUS_WEAK_SIGNATURE_ALGORITHM, cef_ssl_content_status_t,
    cef_ssl_version_t, cef_sslinfo_t, cef_sslstatus_t
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

/// Supported SSL content status flags. See content/public/common/ssl_status.h
/// for more information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SslContentStatus {
    NormalContent,
    DisplayedInsecureContent,
    RanInsecureContent
}

impl From<cef_ssl_content_status_t> for SslContentStatus {
    fn from(value: cef_ssl_content_status_t) -> Self {
        SslContentStatus::from(&value)
    }
}

impl From<&cef_ssl_content_status_t> for SslContentStatus {
    fn from(value: &cef_ssl_content_status_t) -> Self {
        match value {
            cef_ssl_content_status_t::SSL_CONTENT_NORMAL_CONTENT => SslContentStatus::NormalContent,
            cef_ssl_content_status_t::SSL_CONTENT_DISPLAYED_INSECURE_CONTENT => {
                SslContentStatus::DisplayedInsecureContent
            },
            cef_ssl_content_status_t::SSL_CONTENT_RAN_INSECURE_CONTENT => {
                SslContentStatus::RanInsecureContent
            },
        }
    }
}

impl From<SslContentStatus> for cef_ssl_content_status_t {
    fn from(value: SslContentStatus) -> Self {
        cef_ssl_content_status_t::from(&value)
    }
}

impl From<&SslContentStatus> for cef_ssl_content_status_t {
    fn from(value: &SslContentStatus) -> Self {
        match value {
            SslContentStatus::NormalContent => cef_ssl_content_status_t::SSL_CONTENT_NORMAL_CONTENT,
            SslContentStatus::DisplayedInsecureContent => {
                cef_ssl_content_status_t::SSL_CONTENT_DISPLAYED_INSECURE_CONTENT
            },
            SslContentStatus::RanInsecureContent => {
                cef_ssl_content_status_t::SSL_CONTENT_RAN_INSECURE_CONTENT
            },
        }
    }
}

/// Supported SSL version values. See net/ssl/ssl_connection_status_flags.h
/// for more information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SslVersion {
    Unknown,
    Ssl2,
    Ssl3,
    Tls1,
    Tls11,
    Tls12,
    Tls13,
    Quic
}

impl From<cef_ssl_version_t> for SslVersion {
    fn from(value: cef_ssl_version_t) -> Self {
        SslVersion::from(&value)
    }
}

impl From<&cef_ssl_version_t> for SslVersion {
    fn from(value: &cef_ssl_version_t) -> Self {
        match value {
            cef_ssl_version_t::SSL_CONNECTION_VERSION_UNKNOWN => SslVersion::Unknown,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_SSL2 => SslVersion::Ssl2,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_SSL3 => SslVersion::Ssl3,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1 => SslVersion::Tls1,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_1 => SslVersion::Tls11,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_2 => SslVersion::Tls12,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_3 => SslVersion::Tls13,
            cef_ssl_version_t::SSL_CONNECTION_VERSION_QUIC => SslVersion::Quic
        }
    }
}

impl From<SslVersion> for cef_ssl_version_t {
    fn from(value: SslVersion) -> Self {
        cef_ssl_version_t::from(&value)
    }
}

impl From<&SslVersion> for cef_ssl_version_t {
    fn from(value: &SslVersion) -> Self {
        match value {
            SslVersion::Unknown => cef_ssl_version_t::SSL_CONNECTION_VERSION_UNKNOWN,
            SslVersion::Ssl2 => cef_ssl_version_t::SSL_CONNECTION_VERSION_SSL2,
            SslVersion::Ssl3 => cef_ssl_version_t::SSL_CONNECTION_VERSION_SSL3,
            SslVersion::Tls1 => cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1,
            SslVersion::Tls11 => cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_1,
            SslVersion::Tls12 => cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_2,
            SslVersion::Tls13 => cef_ssl_version_t::SSL_CONNECTION_VERSION_TLS1_3,
            SslVersion::Quic => cef_ssl_version_t::SSL_CONNECTION_VERSION_QUIC
        }
    }
}

// Structure representing SSL information.
ref_counted_ptr!(SslInfo, cef_sslinfo_t);

impl SslInfo {
    /// Returns a bitmask containing any and all problems verifying the server
    /// certificate.
    pub fn get_cert_status(&self) -> Result<CertStatus> {
        try_c!(self, get_cert_status, {
            Ok(get_cert_status(self.as_ptr()).into())
        })
    }

    /// Returns the X.509 certificate.
    pub fn get_x509certificate(&self) -> Result<X509Certificate> {
        try_c!(self, get_x509certificate, {
            Ok(X509Certificate::from_ptr_unchecked(get_x509certificate(
                self.as_ptr()
            )))
        })
    }
}

// Structure representing the SSL information for a navigation entry.
ref_counted_ptr!(SslStatus, cef_sslstatus_t);

impl SslStatus {
    /// Returns true (1) if the status is related to a secure SSL/TLS connection.
    pub fn is_secure_connection(&self) -> Result<bool> {
        try_c!(self, is_secure_connection, {
            Ok(is_secure_connection(self.as_ptr()) != 0)
        })
    }

    /// Returns a bitmask containing any and all problems verifying the server
    /// certificate.
    pub fn get_cert_status(&self) -> Result<CertStatus> {
        try_c!(self, get_cert_status, {
            Ok(get_cert_status(self.as_ptr()).into())
        })
    }

    /// Returns the SSL version used for the SSL connection.
    pub fn get_ssl_version(&self) -> Result<SslVersion> {
        try_c!(self, get_sslversion, {
            Ok(get_sslversion(self.as_ptr()).into())
        })
    }

    /// Returns a bitmask containing the page security content status.
    pub fn get_content_status(&self) -> Result<SslContentStatus> {
        try_c!(self, get_content_status, {
            Ok(get_content_status(self.as_ptr()).into())
        })
    }

    /// Returns the X.509 certificate.
    pub fn get_x509certificate(&self) -> Result<X509Certificate> {
        try_c!(self, get_x509certificate, {
            Ok(X509Certificate::from_ptr_unchecked(get_x509certificate(
                self.as_ptr()
            )))
        })
    }
}
