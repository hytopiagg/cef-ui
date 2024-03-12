use bindings::{
    cef_errorcode_t, cef_insets_t, cef_log_items_t, cef_log_severity_t, cef_paint_element_type_t,
    cef_point_t, cef_range_t, cef_rect_t, cef_size_t, cef_state_t, cef_zoom_command_t
};

// Ranges:
//     0- 99 System related errors
//   100-199 Connection related errors
//   200-299 Certificate errors
//   300-399 HTTP errors
//   400-499 Cache errors
//   500-599 ?
//   600-699 FTP errors
//   700-799 Certificate manager errors
//   800-899 DNS resolver errors
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorCode {
    /// No error.
    None,

    /// An asynchronous IO operation is not yet complete.  This usually does not
    /// indicate a fatal error.  Typically this error will be generated as a
    /// notification to wait for some external notification that the IO operation
    /// finally completed.
    IoPending,

    /// A generic failure occurred.
    Failed,

    /// An operation was aborted (due to user action).
    Aborted,

    /// An argument to the function is incorrect.
    InvalidArgument,

    /// The handle or file descriptor is invalid.
    InvalidHandle,

    /// The file or directory cannot be found.
    FileNotFound,

    /// An operation timed out.
    TimedOut,

    /// The file is too large.
    FileTooBig,

    /// An unexpected error.  This may be caused by a programming mistake or an
    /// invalid assumption.
    Unexpected,

    /// Permission to access a resource, other than the network, was denied.
    AccessDenied,

    /// The operation failed because of unimplemented functionality.
    NotImplemented,

    /// There were not enough resources to complete the operation.
    InsufficientResources,

    /// Memory allocation failed.
    OutOfMemory,

    /// The file upload failed because the file's modification time was different
    /// from the expectation.
    UploadFileChanged,

    /// The socket is not connected.
    SocketNotConnected,

    /// The file already exists.
    FileExists,

    /// The path or file name is too long.
    FilePathTooLong,

    /// Not enough room left on the disk.
    FileNoSpace,

    /// The file has a virus.
    FileVirusInfected,

    /// The client chose to block the request.
    BlockedByClient,

    /// The network changed.
    NetworkChanged,

    /// The request was blocked by the URL block list configured by the domain
    /// administrator.
    BlockedByAdministrator,

    /// The socket is already connected.
    SocketIsConnected,

    /// Error -24 was removed (BLOCKED_ENROLLMENT_CHECK_PENDING)

    /// The upload failed because the upload stream needed to be re-read, due to a
    /// retry or a redirect, but the upload stream doesn't support that operation.
    UploadStreamRewindNotSupported,

    /// The request failed because the URLRequestContext is shutting down, or has
    /// been shut down.
    ContextShutDown,

    /// The request failed because the response was delivered along with requirements
    /// which are not met ('X-Frame-Options' and 'Content-Security-Policy' ancestor
    /// checks and 'Cross-Origin-Resource-Policy' for instance).
    BlockedByResponse,

    /// Error -28 was removed (BLOCKED_BY_XSS_AUDITOR).

    /// The request was blocked by system policy disallowing some or all cleartext
    /// requests. Used for NetworkSecurityPolicy on Android.
    CleartextNotPermitted,

    /// The request was blocked by a Content Security Policy
    BlockedByCsp,

    /// The request was blocked because of no H/2 or QUIC session.
    H2OrQuicRequired,

    /// The request was blocked by CORB or ORB.
    BlockedByOrb,

    /// A connection was closed (corresponding to a TCP FIN).
    ConnectionClosed,

    /// A connection was reset (corresponding to a TCP RST).
    ConnectionReset,

    /// A connection attempt was refused.
    ConnectionRefused,

    /// A connection timed out as a result of not receiving an ACK for data sent.
    /// This can include a FIN packet that did not get ACK'd.
    ConnectionAborted,

    /// A connection attempt failed.
    ConnectionFailed,

    /// The host name could not be resolved.
    NameNotResolved,

    /// The Internet connection has been lost.
    InternetDisconnected,

    /// An SSL protocol error occurred.
    SslProtocolError,

    /// The IP address or port number is invalid (e.g., cannot connect to the IP
    /// address 0 or the port 0).
    AddressInvalid,

    /// The IP address is unreachable.  This usually means that there is no route to
    /// the specified host or network.
    AddressUnreachable,

    /// The server requested a client certificate for SSL client authentication.
    SslClientAuthCertNeeded,

    /// A tunnel connection through the proxy could not be established.
    TunnelConnectionFailed,

    /// No SSL protocol versions are enabled.
    NoSslVersionsEnabled,

    /// The client and server don't support a common SSL protocol version or
    /// cipher suite.
    SslVersionOrCipherMismatch,

    /// The server requested a renegotiation (rehandshake).
    SslRenegotiationRequested,

    /// The proxy requested authentication (for tunnel establishment) with an
    /// unsupported method.
    ProxyAuthUnsupported,

    /// Error -116 was removed (CERT_ERROR_IN_SSL_RENEGOTIATION)

    /// The SSL handshake failed because of a bad or missing client certificate.
    BadSslClientAuthCert,

    /// A connection attempt timed out.
    ConnectionTimedOut,

    /// There are too many pending DNS resolves, so a request in the queue was
    /// aborted.
    HostResolverQueueTooLarge,

    /// Failed establishing a connection to the SOCKS proxy server for a target host.
    SocksConnectionFailed,

    /// The SOCKS proxy server failed establishing connection to the target host
    /// because that host is unreachable.
    SocksConnectionHostUnreachable,

    /// The request to negotiate an alternate protocol failed.
    AlpnNegotiationFailed,

    /// The peer sent an SSL no_renegotiation alert message.
    SslNoRenegotiation,

    /// Winsock sometimes reports more data written than passed.  This is probably
    /// due to a broken LSP.
    WinsockUnexpectedWrittenBytes,

    /// An SSL peer sent us a fatal decompression_failure alert. This typically
    /// occurs when a peer selects DEFLATE compression in the mistaken belief that
    /// it supports it.
    SslDecompressionFailureAlert,

    /// An SSL peer sent us a fatal bad_record_mac alert. This has been observed
    /// from servers with buggy DEFLATE support.
    SslBadRecordMacAlert,

    /// The proxy requested authentication (for tunnel establishment).
    ProxyAuthRequested,

    /// Error -129 was removed (SSL_WEAK_SERVER_EPHEMERAL_DH_KEY).

    /// Could not create a connection to the proxy server. An error occurred
    /// either in resolving its name, or in connecting a socket to it.
    /// Note that this does NOT include failures during the actual "CONNECT" method
    /// of an HTTP proxy.
    ProxyConnectionFailed,

    /// A mandatory proxy configuration could not be used. Currently this means
    /// that a mandatory PAC script could not be fetched, parsed or executed.
    MandatoryProxyConfigurationFailed,

    /// -132 was formerly ERR_ESET_ANTI_VIRUS_SSL_INTERCEPTION

    /// We've hit the max socket limit for the socket pool while preconnecting.  We
    /// don't bother trying to preconnect more sockets.
    PreconnectMaxSocketLimit,

    /// The permission to use the SSL client certificate's private key was denied.
    SslClientAuthPrivateKeyAccessDenied,

    /// The SSL client certificate has no private key.
    SslClientAuthCertNoPrivateKey,

    /// The certificate presented by the HTTPS Proxy was invalid.
    ProxyCertificateInvalid,

    /// An error occurred when trying to do a name resolution (DNS).
    NameResolutionFailed,

    /// Permission to access the network was denied. This is used to distinguish
    /// errors that were most likely caused by a firewall from other access denied
    /// errors. See also ERR_ACCESS_DENIED.
    NetworkAccessDenied,

    /// The request throttler module cancelled this request to avoid DDOS.
    TemporarilyThrottled,

    /// A request to create an SSL tunnel connection through the HTTPS proxy
    /// received a 302 (temporary redirect) response.  The response body might
    /// include a description of why the request failed.
    ///
    /// TODO(https:///crbug.com/928551): This is deprecated and should not be used by
    /// new code.
    HttpsProxyTunnelResponseRedirect,

    /// We were unable to sign the CertificateVerify data of an SSL client auth
    /// handshake with the client certificate's private key.
    ///
    /// Possible causes for this include the user implicitly or explicitly
    /// denying access to the private key, the private key may not be valid for
    /// signing, the key may be relying on a cached handle which is no longer
    /// valid, or the CSP won't allow arbitrary data to be signed.
    SslClientAuthSignatureFailed,

    /// The message was too large for the transport.  (for example a UDP message
    /// which exceeds size threshold).
    MsgTooBig,

    /// Error -143 was removed (SPDY_SESSION_ALREADY_EXISTS)

    /// Error -144 was removed (LIMIT_VIOLATION).

    /// Websocket protocol error. Indicates that we are terminating the connection
    /// due to a malformed frame or other protocol violation.
    WsProtocolError,

    /// Error -146 was removed (PROTOCOL_SWITCHED)

    /// Returned when attempting to bind an address that is already in use.
    AddressInUse,

    /// An operation failed because the SSL handshake has not completed.
    SslHandshakeNotCompleted,

    /// SSL peer's public key is invalid.
    SslBadPeerPublicKey,

    /// The certificate didn't match the built-in public key pins for the host name.
    /// The pins are set in net/http/transport_security_state.cc and require that
    /// one of a set of public keys exist on the path from the leaf to the root.
    SslPinnedKeyNotInCertChain,

    /// Server request for client certificate did not contain any types we support.
    ClientAuthCertTypeUnsupported,

    /// Error -152 was removed (ORIGIN_BOUND_CERT_GENERATION_TYPE_MISMATCH)

    /// An SSL peer sent us a fatal decrypt_error alert. This typically occurs when
    /// a peer could not correctly verify a signature (in CertificateVerify or
    /// ServerKeyExchange) or validate a Finished message.
    SslDecryptErrorAlert,

    /// There are too many pending WebSocketJob instances, so the new job was not
    /// pushed to the queue.
    WsThrottleQueueTooLarge,

    /// Error -155 was removed (TOO_MANY_SOCKET_STREAMS)

    /// The SSL server certificate changed in a renegotiation.
    SslServerCertChanged,

    /// Error -157 was removed (SSL_INAPPROPRIATE_FALLBACK).

    /// Error -158 was removed (CT_NO_SCTS_VERIFIED_OK).

    /// The SSL server sent us a fatal unrecognized_name alert.
    SslUnrecognizedNameAlert,

    /// Failed to set the socket's receive buffer size as requested.
    SocketSetReceiveBufferSizeError,

    /// Failed to set the socket's send buffer size as requested.
    SocketSetSendBufferSizeError,

    /// Failed to set the socket's receive buffer size as requested, despite success
    /// return code from setsockopt.
    SocketReceiveBufferSizeUnchangeable,

    /// Failed to set the socket's send buffer size as requested, despite success
    /// return code from setsockopt.
    SocketSendBufferSizeUnchangeable,

    /// Failed to import a client certificate from the platform store into the SSL
    /// library.
    SslClientAuthCertBadFormat,

    /// Error -165 was removed (SSL_FALLBACK_BEYOND_MINIMUM_VERSION).

    /// Resolving a hostname to an IP address list included the IPv4 address
    /// "127.0.53.53". This is a special IP address which ICANN has recommended to
    /// indicate there was a name collision, and alert admins to a potential
    /// problem.
    IcannNameCollision,

    /// The SSL server presented a certificate which could not be decoded. This is
    /// not a certificate error code as no X509Certificate object is available. This
    /// error is fatal.
    SslServerCertBadFormat,

    /// Certificate Transparency: Received a signed tree head that failed to parse.
    CtSthParsingFailed,

    /// Certificate Transparency: Received a signed tree head whose JSON parsing was
    /// OK but was missing some of the fields.
    CtSthIncomplete,

    /// The attempt to reuse a connection to send proxy auth credentials failed
    /// before the AuthController was used to generate credentials. The caller should
    /// reuse the controller with a new connection. This error is only used
    /// internally by the network stack.
    UnableToReuseConnectionForProxyAuth,

    /// Certificate Transparency: Failed to parse the received consistency proof.
    CtConsistencyProofParsingFailed,

    /// The SSL server required an unsupported cipher suite that has since been
    /// removed. This error will temporarily be signaled on a fallback for one or two
    /// releases immediately following a cipher suite's removal, after which the
    /// fallback will be removed.
    SslObsoleteCipher,

    /// When a WebSocket handshake is done successfully and the connection has been
    /// upgraded, the URLRequest is cancelled with this error code.
    WsUpgrade,

    /// Socket ReadIfReady support is not implemented. This error should not be user
    /// visible, because the normal Read() method is used as a fallback.
    ReadIfReadyNotImplemented,

    /// Error -175 was removed (SSL_VERSION_INTERFERENCE).

    /// No socket buffer space is available.
    NoBufferSpace,

    /// There were no common signature algorithms between our client certificate
    /// private key and the server's preferences.
    SslClientAuthNoCommonAlgorithms,

    /// TLS 1.3 early data was rejected by the server. This will be received before
    /// any data is returned from the socket. The request should be retried with
    /// early data disabled.
    EarlyDataRejected,

    /// TLS 1.3 early data was offered, but the server responded with TLS 1.2 or
    /// earlier. This is an internal error code to account for a
    /// backwards-compatibility issue with early data and TLS 1.2. It will be
    /// received before any data is returned from the socket. The request should be
    /// retried with early data disabled.
    ///
    /// See https:///tools.ietf.org/html/rfc8446#appendix-D.3 for details.
    WrongVersionOnEarlyData,

    /// TLS 1.3 was enabled, but a lower version was negotiated and the server
    /// returned a value indicating it supported TLS 1.3. This is part of a security
    /// check in TLS 1.3, but it may also indicate the user is behind a buggy
    /// TLS-terminating proxy which implemented TLS 1.2 incorrectly. (See
    /// https:///crbug.com/boringssl/226.)
    Tls13DowngradeDetected,

    /// The server's certificate has a keyUsage extension incompatible with the
    /// negotiated TLS key exchange method.
    SslKeyUsageIncompatible,

    /// The ECHConfigList fetched over DNS cannot be parsed.
    InvalidEchConfigList,

    /// ECH was enabled, but the server was unable to decrypt the encrypted
    /// ClientHello.
    EchNotNegotiated,

    /// ECH was enabled, the server was unable to decrypt the encrypted ClientHello,
    /// and additionally did not present a certificate valid for the public name.
    EchFallbackCertificateInvalid,

    /// Certificate error codes
    ///
    /// The values of certificate error codes must be consecutive.

    /// The server responded with a certificate whose common name did not match
    /// the host name.  This could mean:
    ///
    /// 1. An attacker has redirected our traffic to their server and is
    ///    presenting a certificate for which they know the private key.
    ///
    /// 2. The server is misconfigured and responding with the wrong cert.
    ///
    /// 3. The user is on a wireless network and is being redirected to the
    ///    network's login page.
    ///
    /// 4. The OS has used a DNS search suffix and the server doesn't have
    ///    a certificate for the abbreviated name in the address bar.
    CertCommonNameInvalid,

    /// The server responded with a certificate that, by our clock, appears to
    /// either not yet be valid or to have expired.  This could mean:
    ///
    /// 1. An attacker is presenting an old certificate for which they have
    ///    managed to obtain the private key.
    ///
    /// 2. The server is misconfigured and is not presenting a valid cert.
    ///
    /// 3. Our clock is wrong.
    ///
    CertDateInvalid,

    /// The server responded with a certificate that is signed by an authority
    /// we don't trust.  The could mean:
    ///
    /// 1. An attacker has substituted the real certificate for a cert that
    ///    contains their public key and is signed by their cousin.
    ///
    /// 2. The server operator has a legitimate certificate from a CA we don't
    ///    know about, but should trust.
    ///
    /// 3. The server is presenting a self-signed certificate, providing no
    ///    defense against active attackers (but foiling passive attackers).
    CertAuthorityInvalid,

    /// The server responded with a certificate that contains errors.
    /// This error is not recoverable.
    ///
    /// MSDN describes this error as follows:
    ///   "The SSL certificate contains errors."
    /// NOTE: It's unclear how this differs from ERR_CERT_INVALID. For consistency,
    /// use that code instead of this one from now on.
    CertContainsErrors,

    /// The certificate has no mechanism for determining if it is revoked.  In
    /// effect, this certificate cannot be revoked.
    CertNoRevocationMechanism,

    /// Revocation information for the security certificate for this site is not
    /// available.  This could mean:
    ///
    /// 1. An attacker has compromised the private key in the certificate and is
    ///    blocking our attempt to find out that the cert was revoked.
    ///
    /// 2. The certificate is unrevoked, but the revocation server is busy or
    ///    unavailable.
    CertUnableToCheckRevocation,

    /// The server responded with a certificate has been revoked.
    /// We have the capability to ignore this error, but it is probably not the
    /// thing to do.
    CertRevoked,

    /// The server responded with a certificate that is invalid.
    /// This error is not recoverable.
    ///
    /// MSDN describes this error as follows:
    ///   "The SSL certificate is invalid."
    CertInvalid,

    /// The server responded with a certificate that is signed using a weak
    /// signature algorithm.
    CertWeakSignatureAlgorithm,

    /// -209 is available: was CERT_NOT_IN_DNS.

    /// The host name specified in the certificate is not unique.
    CertNonUniqueName,

    /// The server responded with a certificate that contains a weak key (e.g.
    /// a too-small RSA key).
    CertWeakKey,

    /// The certificate claimed DNS names that are in violation of name constraints.
    CertNameConstraintViolation,

    /// The certificate's validity period is too long.
    CertValidityTooLong,

    /// Certificate Transparency was required for this connection, but the server
    /// did not provide CT information that complied with the policy.
    CertificateTransparencyRequired,

    /// The certificate chained to a legacy Symantec root that is no longer trusted.
    /// https:///g.co/chrome/symantecpkicerts
    CertSymantecLegacy,

    /// -216 was QUIC_CERT_ROOT_NOT_KNOWN which has been renumbered to not be in the
    /// certificate error range.

    /// The certificate is known to be used for interception by an entity other
    /// the device owner.
    CertKnownInterceptionBlocked,

    /// -218 was SSL_OBSOLETE_VERSION which is not longer used. TLS 1.0/1.1 instead
    /// cause SSL_VERSION_OR_CIPHER_MISMATCH now.

    /// Add new certificate error codes here.
    ///
    /// Update the value of CERT_END whenever you add a new certificate error
    /// code.

    /// The value immediately past the last certificate error code.
    CertEnd,

    /// The URL is invalid.
    InvalidUrl,

    /// The scheme of the URL is disallowed.
    DisallowedUrlScheme,

    /// The scheme of the URL is unknown.
    UnknownUrlScheme,

    /// Attempting to load an URL resulted in a redirect to an invalid URL.
    InvalidRedirect,

    /// Attempting to load an URL resulted in too many redirects.
    TooManyRedirects,

    /// Attempting to load an URL resulted in an unsafe redirect (e.g., a redirect
    /// to file:/// is considered unsafe).
    UnsafeRedirect,

    /// Attempting to load an URL with an unsafe port number.  These are port
    /// numbers that correspond to services, which are not robust to spurious input
    /// that may be constructed as a result of an allowed web construct (e.g., HTTP
    /// looks a lot like SMTP, so form submission to port 25 is denied).
    UnsafePort,

    /// The server's response was invalid.
    InvalidResponse,

    /// Error in chunked transfer encoding.
    InvalidChunkedEncoding,

    /// The server did not support the request method.
    MethodNotSupported,

    /// The response was 407 (Proxy Authentication Required), yet we did not send
    /// the request to a proxy.
    UnexpectedProxyAuth,

    /// The server closed the connection without sending any data.
    EmptyResponse,

    /// The headers section of the response is too large.
    ResponseHeadersTooBig,

    /// Error -326 was removed (PAC_STATUS_NOT_OK)

    /// The evaluation of the PAC script failed.
    PacScriptFailed,

    /// The response was 416 (Requested range not satisfiable) and the server cannot
    /// satisfy the range requested.
    RequestedRangeNotSatisfiable,

    /// The identity used for authentication is invalid.
    MalformedIdentity,

    /// Content decoding of the response body failed.
    ContentDecodingFailed,

    /// An operation could not be completed because all network IO
    /// is suspended.
    NetworkIoSuspended,

    /// FLIP data received without receiving a SYN_REPLY on the stream.
    SynReplyNotReceived,

    /// Converting the response to target encoding failed.
    EncodingConversionFailed,

    /// The server sent an FTP directory listing in a format we do not understand.
    UnrecognizedFtpDirectoryListingFormat,

    /// Obsolete.  Was only logged in NetLog when an HTTP/2 pushed stream expired.
    /// NET_ERROR(INVALID_SPDY_STREAM, -335)

    /// There are no supported proxies in the provided list.
    NoSupportedProxies,

    /// There is an HTTP/2 protocol error.
    Http2ProtocolError,

    /// Credentials could not be established during HTTP Authentication.
    InvalidAuthCredentials,

    /// An HTTP Authentication scheme was tried which is not supported on this
    /// machine.
    UnsupportedAuthScheme,

    /// Detecting the encoding of the response failed.
    EncodingDetectionFailed,

    /// (GSSAPI) No Kerberos credentials were available during HTTP Authentication.
    MissingAuthCredentials,

    /// An unexpected, but documented, SSPI or GSSAPI status code was returned.
    UnexpectedSecurityLibraryStatus,

    /// The environment was not set up correctly for authentication (for
    /// example, no KDC could be found or the principal is unknown.
    MisconfiguredAuthEnvironment,

    /// An undocumented SSPI or GSSAPI status code was returned.
    UndocumentedSecurityLibraryStatus,

    /// The HTTP response was too big to drain.
    ResponseBodyTooBigToDrain,

    /// The HTTP response contained multiple distinct Content-Length headers.
    ResponseHeadersMultipleContentLength,

    /// HTTP/2 headers have been received, but not all of them - status or version
    /// headers are missing, so we're expecting additional frames to complete them.
    IncompleteHttp2Headers,

    /// No PAC URL configuration could be retrieved from DHCP. This can indicate
    /// either a failure to retrieve the DHCP configuration, or that there was no
    /// PAC URL configured in DHCP.
    PacNotInDhcp,

    /// The HTTP response contained multiple Content-Disposition headers.
    ResponseHeadersMultipleContentDisposition,

    /// The HTTP response contained multiple Location headers.
    ResponseHeadersMultipleLocation,

    /// HTTP/2 server refused the request without processing, and sent either a
    /// GOAWAY frame with error code NO_ERROR and Last-Stream-ID lower than the
    /// stream id corresponding to the request indicating that this request has not
    /// been processed yet, or a RST_STREAM frame with error code REFUSED_STREAM.
    /// Client MAY retry (on a different connection).  See RFC7540 Section 8.1.4.
    Http2ServerRefusedStream,

    /// HTTP/2 server didn't respond to the PING message.
    Http2PingFailed,

    /// Obsolete.  Kept here to avoid reuse, as the old error can still appear on
    /// histograms.
    /// NET_ERROR(PIPELINE_EVICTION, -353)

    /// The HTTP response body transferred fewer bytes than were advertised by the
    /// Content-Length header when the connection is closed.
    ContentLengthMismatch,

    /// The HTTP response body is transferred with Chunked-Encoding, but the
    /// terminating zero-length chunk was never sent when the connection is closed.
    IncompleteChunkedEncoding,

    /// There is a QUIC protocol error.
    QuicProtocolError,

    /// The HTTP headers were truncated by an EOF.
    ResponseHeadersTruncated,

    /// The QUIC crypto handshake failed.  This means that the server was unable
    /// to read any requests sent, so they may be resent.
    QuicHandshakeFailed,

    /// Obsolete.  Kept here to avoid reuse, as the old error can still appear on
    /// histograms.
    /// NET_ERROR(REQUEST_FOR_SECURE_RESOURCE_OVER_INSECURE_QUIC, -359)

    /// Transport security is inadequate for the HTTP/2 version.
    Http2InadequateTransportSecurity,

    /// The peer violated HTTP/2 flow control.
    Http2FlowControlError,

    /// The peer sent an improperly sized HTTP/2 frame.
    Http2FrameSizeError,

    /// Decoding or encoding of compressed HTTP/2 headers failed.
    Http2CompressionError,

    /// Proxy Auth Requested without a valid Client Socket Handle.
    ProxyAuthRequestedWithNoConnection,

    /// HTTP_1_1_REQUIRED error code received on HTTP/2 session.
    Http11Required,

    /// HTTP_1_1_REQUIRED error code received on HTTP/2 session to proxy.
    ProxyHttp11Required,

    /// The PAC script terminated fatally and must be reloaded.
    PacScriptTerminated,

    /// Obsolete. Kept here to avoid reuse.
    /// Request is throttled because of a Backoff header.
    /// See: crbug.com/486891.
    /// NET_ERROR(TEMPORARY_BACKOFF, -369)

    /// The server was expected to return an HTTP/1.x response, but did not. Rather
    /// than treat it as HTTP/0.9, this error is returned.
    InvalidHttpResponse,

    /// Initializing content decoding failed.
    ContentDecodingInitFailed,

    /// Received HTTP/2 RST_STREAM frame with NO_ERROR error code.  This error should
    /// be handled internally by HTTP/2 code, and should not make it above the
    /// SpdyStream layer.
    Http2RstStreamNoErrorReceived,

    /// Obsolete. HTTP/2 push is removed.
    /// NET_ERROR(HTTP2_PUSHED_STREAM_NOT_AVAILABLE, -373)

    /// Obsolete. HTTP/2 push is removed.
    /// NET_ERROR(HTTP2_CLAIMED_PUSHED_STREAM_RESET_BY_SERVER, -374)

    /// An HTTP transaction was retried too many times due for authentication or
    /// invalid certificates. This may be due to a bug in the net stack that would
    /// otherwise infinite loop, or if the server or proxy continually requests fresh
    /// credentials or presents a fresh invalid certificate.
    TooManyRetries,

    /// Received an HTTP/2 frame on a closed stream.
    Http2StreamClosed,

    /// Obsolete. HTTP/2 push is removed.
    /// NET_ERROR(HTTP2_CLIENT_REFUSED_STREAM, -377)

    /// Obsolete. HTTP/2 push is removed.
    /// NET_ERROR(HTTP2_PUSHED_RESPONSE_DOES_NOT_MATCH, -378)

    /// The server returned a non-2xx HTTP response code.
    ///
    /// Note that this error is only used by certain APIs that interpret the HTTP
    /// response itself. URLRequest for instance just passes most non-2xx
    /// response back as success.
    HttpResponseCodeFailure,

    /// The certificate presented on a QUIC connection does not chain to a known root
    /// and the origin connected to is not on a list of domains where unknown roots
    /// are allowed.
    QuicCertRootNotKnown,

    /// A GOAWAY frame has been received indicating that the request has not been
    /// processed and is therefore safe to retry on a different connection.
    QuicGoawayRequestCanBeRetried,

    /// The ACCEPT_CH restart has been triggered too many times
    TooManyAcceptChRestarts,

    /// The IP address space of the remote endpoint differed from the previous
    /// observed value during the same request. Any cache entry for the affected
    /// request should be invalidated.
    InconsistentIpAddressSpace,

    /// The IP address space of the cached remote endpoint is blocked by private
    /// network access check.
    CachedIpAddressSpaceBlockedByPrivateNetworkAccessPolicy,

    /// The connection is blocked by private network access checks.
    BlockedByPrivateNetworkAccessChecks,

    /// The cache does not have the requested entry.
    CacheMiss,

    /// Unable to read from the disk cache.
    CacheReadFailure,

    /// Unable to write to the disk cache.
    CacheWriteFailure,

    /// The operation is not supported for this entry.
    CacheOperationNotSupported,

    /// The disk cache is unable to open this entry.
    CacheOpenFailure,

    /// The disk cache is unable to create this entry.
    CacheCreateFailure,

    /// Multiple transactions are racing to create disk cache entries. This is an
    /// internal error returned from the HttpCache to the HttpCacheTransaction that
    /// tells the transaction to restart the entry-creation logic because the state
    /// of the cache has changed.
    CacheRace,

    /// The cache was unable to read a checksum record on an entry. This can be
    /// returned from attempts to read from the cache. It is an internal error,
    /// returned by the SimpleCache backend, but not by any URLRequest methods
    /// or members.
    CacheChecksumReadFailure,

    /// The cache found an entry with an invalid checksum. This can be returned from
    /// attempts to read from the cache. It is an internal error, returned by the
    /// SimpleCache backend, but not by any URLRequest methods or members.
    CacheChecksumMismatch,

    /// Internal error code for the HTTP cache. The cache lock timeout has fired.
    CacheLockTimeout,

    /// Received a challenge after the transaction has read some data, and the
    /// credentials aren't available.  There isn't a way to get them at that point.
    CacheAuthFailureAfterRead,

    /// Internal not-quite error code for the HTTP cache. In-memory hints suggest
    /// that the cache entry would not have been usable with the transaction's
    /// current configuration (e.g. load flags, mode, etc.)
    CacheEntryNotSuitable,

    /// The disk cache is unable to doom this entry.
    CacheDoomFailure,

    /// The disk cache is unable to open or create this entry.
    CacheOpenOrCreateFailure,

    /// The server's response was insecure (e.g. there was a cert error).
    InsecureResponse,

    /// An attempt to import a client certificate failed, as the user's key
    /// database lacked a corresponding private key.
    NoPrivateKeyForCert,

    /// An error adding a certificate to the OS certificate database.
    AddUserCertFailed,

    /// An error occurred while handling a signed exchange.
    InvalidSignedExchange,

    /// An error occurred while handling a Web Bundle source.
    InvalidWebBundle,

    /// A Trust Tokens protocol operation-executing request failed for one of a
    /// number of reasons (precondition failure, internal error, bad response).
    TrustTokenOperationFailed,

    /// When handling a Trust Tokens protocol operation-executing request, the system
    /// was able to execute the request's Trust Tokens operation without sending the
    /// request to its destination: for instance, the results could have been present
    /// in a local cache (for redemption) or the operation could have been diverted
    /// to a local provider (for "platform-provided" issuance).
    TrustTokenOperationSuccessWithoutSendingRequest,

    /// *** Code -600 is reserved (was FTP_PASV_COMMAND_FAILED). ***

    /// A generic error for failed FTP control connection command.
    /// If possible, please use or add a more specific error code.
    FtpFailed,

    /// The server cannot fulfill the request at this point. This is a temporary
    /// error.
    /// FTP response code 421.
    FtpServiceUnavailable,

    /// The server has aborted the transfer.
    /// FTP response code 426.
    FtpTransferAborted,

    /// The file is busy, or some other temporary error condition on opening
    /// the file.
    /// FTP response code 450.
    FtpFileBusy,

    /// Server rejected our command because of syntax errors.
    /// FTP response codes 500, 501.
    FtpSyntaxError,

    /// Server does not support the command we issued.
    /// FTP response codes 502, 504.
    FtpCommandNotSupported,

    /// Server rejected our command because we didn't issue the commands in right
    /// order.
    /// FTP response code 503.
    FtpBadCommandSequence,

    /// PKCS #12 import failed due to incorrect password.
    Pkcs12ImportBadPassword,

    /// PKCS #12 import failed due to other error.
    Pkcs12ImportFailed,

    /// CA import failed - not a CA cert.
    ImportCaCertNotCa,

    /// Import failed - certificate already exists in database.
    /// Note it's a little weird this is an error but reimporting a PKCS12 is ok
    /// (no-op).  That's how Mozilla does it, though.
    ImportCertAlreadyExists,

    /// CA import failed due to some other error.
    ImportCaCertFailed,

    /// Server certificate import failed due to some internal error.
    ImportServerCertFailed,

    /// PKCS #12 import failed due to invalid MAC.
    Pkcs12ImportInvalidMac,

    /// PKCS #12 import failed due to invalid/corrupt file.
    Pkcs12ImportInvalidFile,

    /// PKCS #12 import failed due to unsupported features.
    Pkcs12ImportUnsupported,

    /// Key generation failed.
    KeyGenerationFailed,

    /// Error -711 was removed (ORIGIN_BOUND_CERT_GENERATION_FAILED)

    /// Failure to export private key.
    PrivateKeyExportFailed,

    /// Self-signed certificate generation failed.
    SelfSignedCertGenerationFailed,

    /// The certificate database changed in some way.
    CertDatabaseChanged,

    /// Error -715 was removed (CHANNEL_ID_IMPORT_FAILED)

    /// The certificate verifier configuration changed in some way.
    CertVerifierChanged,

    /// DNS error codes.

    /// DNS resolver received a malformed response.
    DnsMalformedResponse,

    /// DNS server requires TCP
    DnsServerRequiresTcp,

    /// DNS server failed.  This error is returned for all of the following
    /// error conditions:
    /// 1 - Format error - The name server was unable to interpret the query.
    /// 2 - Server failure - The name server was unable to process this query
    ///     due to a problem with the name server.
    /// 4 - Not Implemented - The name server does not support the requested
    ///     kind of query.
    /// 5 - Refused - The name server refuses to perform the specified
    ///     operation for policy reasons.
    DnsServerFailed,

    /// DNS transaction timed out.
    DnsTimedOut,

    /// The entry was not found in cache or other local sources, for lookups where
    /// only local sources were queried.
    /// TODO(ericorth): Consider renaming to DNS_LOCAL_MISS or something like that as
    /// the cache is not necessarily queried either.
    DnsCacheMiss,

    /// Suffix search list rules prevent resolution of the given host name.
    DnsSearchEmpty,

    /// Failed to sort addresses according to RFC3484.
    DnsSortError,

    /// Error -807 was removed (DNS_HTTP_FAILED)

    /// Failed to resolve the hostname of a DNS-over-HTTPS server.
    DnsSecureResolverHostnameResolutionFailed,

    /// DNS identified the request as disallowed for insecure connection (http/ws).
    /// Error should be handled as if an HTTP redirect was received to redirect to
    /// https or wss.
    DnsNameHttpsOnly,

    /// All DNS requests associated with this job have been cancelled.
    DnsRequestCancelled,

    /// The hostname resolution of HTTPS record was expected to be resolved with
    /// alpn values of supported protocols, but did not.
    DnsNoMatchingSupportedAlpn,

    /// The compression dictionary cannot be loaded.
    DictionaryLoadFailed // Error -813 was removed (DICTIONARY_ORIGIN_CHECK_FAILED)
}

impl From<cef_errorcode_t> for ErrorCode {
    fn from(value: cef_errorcode_t) -> Self {
        ErrorCode::from(&value)
    }
}

impl From<&cef_errorcode_t> for ErrorCode {
    fn from(value: &cef_errorcode_t) -> Self {
        match value {
            cef_errorcode_t::ERR_NONE => ErrorCode::None,
            cef_errorcode_t::ERR_IO_PENDING => ErrorCode::IoPending,
            cef_errorcode_t::ERR_FAILED => ErrorCode::Failed,
            cef_errorcode_t::ERR_ABORTED => ErrorCode::Aborted,
            cef_errorcode_t::ERR_INVALID_ARGUMENT => ErrorCode::InvalidArgument,
            cef_errorcode_t::ERR_INVALID_HANDLE => ErrorCode::InvalidHandle,
            cef_errorcode_t::ERR_FILE_NOT_FOUND => ErrorCode::FileNotFound,
            cef_errorcode_t::ERR_TIMED_OUT => ErrorCode::TimedOut,
            cef_errorcode_t::ERR_FILE_TOO_BIG => ErrorCode::FileTooBig,
            cef_errorcode_t::ERR_UNEXPECTED => ErrorCode::Unexpected,
            cef_errorcode_t::ERR_ACCESS_DENIED => ErrorCode::AccessDenied,
            cef_errorcode_t::ERR_NOT_IMPLEMENTED => ErrorCode::NotImplemented,
            cef_errorcode_t::ERR_INSUFFICIENT_RESOURCES => ErrorCode::InsufficientResources,
            cef_errorcode_t::ERR_OUT_OF_MEMORY => ErrorCode::OutOfMemory,
            cef_errorcode_t::ERR_UPLOAD_FILE_CHANGED => ErrorCode::UploadFileChanged,
            cef_errorcode_t::ERR_SOCKET_NOT_CONNECTED => ErrorCode::SocketNotConnected,
            cef_errorcode_t::ERR_FILE_EXISTS => ErrorCode::FileExists,
            cef_errorcode_t::ERR_FILE_PATH_TOO_LONG => ErrorCode::FilePathTooLong,
            cef_errorcode_t::ERR_FILE_NO_SPACE => ErrorCode::FileNoSpace,
            cef_errorcode_t::ERR_FILE_VIRUS_INFECTED => ErrorCode::FileVirusInfected,
            cef_errorcode_t::ERR_BLOCKED_BY_CLIENT => ErrorCode::BlockedByClient,
            cef_errorcode_t::ERR_NETWORK_CHANGED => ErrorCode::NetworkChanged,
            cef_errorcode_t::ERR_BLOCKED_BY_ADMINISTRATOR => ErrorCode::BlockedByAdministrator,
            cef_errorcode_t::ERR_SOCKET_IS_CONNECTED => ErrorCode::SocketIsConnected,
            cef_errorcode_t::ERR_UPLOAD_STREAM_REWIND_NOT_SUPPORTED => ErrorCode::UploadStreamRewindNotSupported,
            cef_errorcode_t::ERR_CONTEXT_SHUT_DOWN => ErrorCode::ContextShutDown,
            cef_errorcode_t::ERR_BLOCKED_BY_RESPONSE => ErrorCode::BlockedByResponse,
            cef_errorcode_t::ERR_CLEARTEXT_NOT_PERMITTED => ErrorCode::CleartextNotPermitted,
            cef_errorcode_t::ERR_BLOCKED_BY_CSP => ErrorCode::BlockedByCsp,
            cef_errorcode_t::ERR_H2_OR_QUIC_REQUIRED => ErrorCode::H2OrQuicRequired,
            cef_errorcode_t::ERR_BLOCKED_BY_ORB => ErrorCode::BlockedByOrb,
            cef_errorcode_t::ERR_CONNECTION_CLOSED => ErrorCode::ConnectionClosed,
            cef_errorcode_t::ERR_CONNECTION_RESET => ErrorCode::ConnectionReset,
            cef_errorcode_t::ERR_CONNECTION_REFUSED => ErrorCode::ConnectionRefused,
            cef_errorcode_t::ERR_CONNECTION_ABORTED => ErrorCode::ConnectionAborted,
            cef_errorcode_t::ERR_CONNECTION_FAILED => ErrorCode::ConnectionFailed,
            cef_errorcode_t::ERR_NAME_NOT_RESOLVED => ErrorCode::NameNotResolved,
            cef_errorcode_t::ERR_INTERNET_DISCONNECTED => ErrorCode::InternetDisconnected,
            cef_errorcode_t::ERR_SSL_PROTOCOL_ERROR => ErrorCode::SslProtocolError,
            cef_errorcode_t::ERR_ADDRESS_INVALID => ErrorCode::AddressInvalid,
            cef_errorcode_t::ERR_ADDRESS_UNREACHABLE => ErrorCode::AddressUnreachable,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_NEEDED => ErrorCode::SslClientAuthCertNeeded,
            cef_errorcode_t::ERR_TUNNEL_CONNECTION_FAILED => ErrorCode::TunnelConnectionFailed,
            cef_errorcode_t::ERR_NO_SSL_VERSIONS_ENABLED => ErrorCode::NoSslVersionsEnabled,
            cef_errorcode_t::ERR_SSL_VERSION_OR_CIPHER_MISMATCH => ErrorCode::SslVersionOrCipherMismatch,
            cef_errorcode_t::ERR_SSL_RENEGOTIATION_REQUESTED => ErrorCode::SslRenegotiationRequested,
            cef_errorcode_t::ERR_PROXY_AUTH_UNSUPPORTED => ErrorCode::ProxyAuthUnsupported,
            cef_errorcode_t::ERR_BAD_SSL_CLIENT_AUTH_CERT => ErrorCode::BadSslClientAuthCert,
            cef_errorcode_t::ERR_CONNECTION_TIMED_OUT => ErrorCode::ConnectionTimedOut,
            cef_errorcode_t::ERR_HOST_RESOLVER_QUEUE_TOO_LARGE => ErrorCode::HostResolverQueueTooLarge,
            cef_errorcode_t::ERR_SOCKS_CONNECTION_FAILED => ErrorCode::SocksConnectionFailed,
            cef_errorcode_t::ERR_SOCKS_CONNECTION_HOST_UNREACHABLE => ErrorCode::SocksConnectionHostUnreachable,
            cef_errorcode_t::ERR_ALPN_NEGOTIATION_FAILED => ErrorCode::AlpnNegotiationFailed,
            cef_errorcode_t::ERR_SSL_NO_RENEGOTIATION => ErrorCode::SslNoRenegotiation,
            cef_errorcode_t::ERR_WINSOCK_UNEXPECTED_WRITTEN_BYTES => ErrorCode::WinsockUnexpectedWrittenBytes,
            cef_errorcode_t::ERR_SSL_DECOMPRESSION_FAILURE_ALERT => ErrorCode::SslDecompressionFailureAlert,
            cef_errorcode_t::ERR_SSL_BAD_RECORD_MAC_ALERT => ErrorCode::SslBadRecordMacAlert,
            cef_errorcode_t::ERR_PROXY_AUTH_REQUESTED => ErrorCode::ProxyAuthRequested,
            cef_errorcode_t::ERR_PROXY_CONNECTION_FAILED => ErrorCode::ProxyConnectionFailed,
            cef_errorcode_t::ERR_MANDATORY_PROXY_CONFIGURATION_FAILED => ErrorCode::MandatoryProxyConfigurationFailed,
            cef_errorcode_t::ERR_PRECONNECT_MAX_SOCKET_LIMIT => ErrorCode::PreconnectMaxSocketLimit,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_PRIVATE_KEY_ACCESS_DENIED => ErrorCode::SslClientAuthPrivateKeyAccessDenied,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_NO_PRIVATE_KEY => ErrorCode::SslClientAuthCertNoPrivateKey,
            cef_errorcode_t::ERR_PROXY_CERTIFICATE_INVALID => ErrorCode::ProxyCertificateInvalid,
            cef_errorcode_t::ERR_NAME_RESOLUTION_FAILED => ErrorCode::NameResolutionFailed,
            cef_errorcode_t::ERR_NETWORK_ACCESS_DENIED => ErrorCode::NetworkAccessDenied,
            cef_errorcode_t::ERR_TEMPORARILY_THROTTLED => ErrorCode::TemporarilyThrottled,
            cef_errorcode_t::ERR_HTTPS_PROXY_TUNNEL_RESPONSE_REDIRECT => ErrorCode::HttpsProxyTunnelResponseRedirect,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_SIGNATURE_FAILED => ErrorCode::SslClientAuthSignatureFailed,
            cef_errorcode_t::ERR_MSG_TOO_BIG => ErrorCode::MsgTooBig,
            cef_errorcode_t::ERR_WS_PROTOCOL_ERROR => ErrorCode::WsProtocolError,
            cef_errorcode_t::ERR_ADDRESS_IN_USE => ErrorCode::AddressInUse,
            cef_errorcode_t::ERR_SSL_HANDSHAKE_NOT_COMPLETED => ErrorCode::SslHandshakeNotCompleted,
            cef_errorcode_t::ERR_SSL_BAD_PEER_PUBLIC_KEY => ErrorCode::SslBadPeerPublicKey,
            cef_errorcode_t::ERR_SSL_PINNED_KEY_NOT_IN_CERT_CHAIN => ErrorCode::SslPinnedKeyNotInCertChain,
            cef_errorcode_t::ERR_CLIENT_AUTH_CERT_TYPE_UNSUPPORTED => ErrorCode::ClientAuthCertTypeUnsupported,
            cef_errorcode_t::ERR_SSL_DECRYPT_ERROR_ALERT => ErrorCode::SslDecryptErrorAlert,
            cef_errorcode_t::ERR_WS_THROTTLE_QUEUE_TOO_LARGE => ErrorCode::WsThrottleQueueTooLarge,
            cef_errorcode_t::ERR_SSL_SERVER_CERT_CHANGED => ErrorCode::SslServerCertChanged,
            cef_errorcode_t::ERR_SSL_UNRECOGNIZED_NAME_ALERT => ErrorCode::SslUnrecognizedNameAlert,
            cef_errorcode_t::ERR_SOCKET_SET_RECEIVE_BUFFER_SIZE_ERROR => ErrorCode::SocketSetReceiveBufferSizeError,
            cef_errorcode_t::ERR_SOCKET_SET_SEND_BUFFER_SIZE_ERROR => ErrorCode::SocketSetSendBufferSizeError,
            cef_errorcode_t::ERR_SOCKET_RECEIVE_BUFFER_SIZE_UNCHANGEABLE => ErrorCode::SocketReceiveBufferSizeUnchangeable,
            cef_errorcode_t::ERR_SOCKET_SEND_BUFFER_SIZE_UNCHANGEABLE => ErrorCode::SocketSendBufferSizeUnchangeable,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_BAD_FORMAT => ErrorCode::SslClientAuthCertBadFormat,
            cef_errorcode_t::ERR_ICANN_NAME_COLLISION => ErrorCode::IcannNameCollision,
            cef_errorcode_t::ERR_SSL_SERVER_CERT_BAD_FORMAT => ErrorCode::SslServerCertBadFormat,
            cef_errorcode_t::ERR_CT_STH_PARSING_FAILED => ErrorCode::CtSthParsingFailed,
            cef_errorcode_t::ERR_CT_STH_INCOMPLETE => ErrorCode::CtSthIncomplete,
            cef_errorcode_t::ERR_UNABLE_TO_REUSE_CONNECTION_FOR_PROXY_AUTH => ErrorCode::UnableToReuseConnectionForProxyAuth,
            cef_errorcode_t::ERR_CT_CONSISTENCY_PROOF_PARSING_FAILED => ErrorCode::CtConsistencyProofParsingFailed,
            cef_errorcode_t::ERR_SSL_OBSOLETE_CIPHER => ErrorCode::SslObsoleteCipher,
            cef_errorcode_t::ERR_WS_UPGRADE => ErrorCode::WsUpgrade,
            cef_errorcode_t::ERR_READ_IF_READY_NOT_IMPLEMENTED => ErrorCode::ReadIfReadyNotImplemented,
            cef_errorcode_t::ERR_NO_BUFFER_SPACE => ErrorCode::NoBufferSpace,
            cef_errorcode_t::ERR_SSL_CLIENT_AUTH_NO_COMMON_ALGORITHMS => ErrorCode::SslClientAuthNoCommonAlgorithms,
            cef_errorcode_t::ERR_EARLY_DATA_REJECTED => ErrorCode::EarlyDataRejected,
            cef_errorcode_t::ERR_WRONG_VERSION_ON_EARLY_DATA => ErrorCode::WrongVersionOnEarlyData,
            cef_errorcode_t::ERR_TLS13_DOWNGRADE_DETECTED => ErrorCode::Tls13DowngradeDetected,
            cef_errorcode_t::ERR_SSL_KEY_USAGE_INCOMPATIBLE => ErrorCode::SslKeyUsageIncompatible,
            cef_errorcode_t::ERR_INVALID_ECH_CONFIG_LIST => ErrorCode::InvalidEchConfigList,
            cef_errorcode_t::ERR_ECH_NOT_NEGOTIATED => ErrorCode::EchNotNegotiated,
            cef_errorcode_t::ERR_ECH_FALLBACK_CERTIFICATE_INVALID => ErrorCode::EchFallbackCertificateInvalid,
            cef_errorcode_t::ERR_CERT_COMMON_NAME_INVALID => ErrorCode::CertCommonNameInvalid,
            cef_errorcode_t::ERR_CERT_DATE_INVALID => ErrorCode::CertDateInvalid,
            cef_errorcode_t::ERR_CERT_AUTHORITY_INVALID => ErrorCode::CertAuthorityInvalid,
            cef_errorcode_t::ERR_CERT_CONTAINS_ERRORS => ErrorCode::CertContainsErrors,
            cef_errorcode_t::ERR_CERT_NO_REVOCATION_MECHANISM => ErrorCode::CertNoRevocationMechanism,
            cef_errorcode_t::ERR_CERT_UNABLE_TO_CHECK_REVOCATION => ErrorCode::CertUnableToCheckRevocation,
            cef_errorcode_t::ERR_CERT_REVOKED => ErrorCode::CertRevoked,
            cef_errorcode_t::ERR_CERT_INVALID => ErrorCode::CertInvalid,
            cef_errorcode_t::ERR_CERT_WEAK_SIGNATURE_ALGORITHM => ErrorCode::CertWeakSignatureAlgorithm,
            cef_errorcode_t::ERR_CERT_NON_UNIQUE_NAME => ErrorCode::CertNonUniqueName,
            cef_errorcode_t::ERR_CERT_WEAK_KEY => ErrorCode::CertWeakKey,
            cef_errorcode_t::ERR_CERT_NAME_CONSTRAINT_VIOLATION => ErrorCode::CertNameConstraintViolation,
            cef_errorcode_t::ERR_CERT_VALIDITY_TOO_LONG => ErrorCode::CertValidityTooLong,
            cef_errorcode_t::ERR_CERTIFICATE_TRANSPARENCY_REQUIRED => ErrorCode::CertificateTransparencyRequired,
            cef_errorcode_t::ERR_CERT_SYMANTEC_LEGACY => ErrorCode::CertSymantecLegacy,
            cef_errorcode_t::ERR_CERT_KNOWN_INTERCEPTION_BLOCKED => ErrorCode::CertKnownInterceptionBlocked,
            cef_errorcode_t::ERR_CERT_END => ErrorCode::CertEnd,
            cef_errorcode_t::ERR_INVALID_URL => ErrorCode::InvalidUrl,
            cef_errorcode_t::ERR_DISALLOWED_URL_SCHEME => ErrorCode::DisallowedUrlScheme,
            cef_errorcode_t::ERR_UNKNOWN_URL_SCHEME => ErrorCode::UnknownUrlScheme,
            cef_errorcode_t::ERR_INVALID_REDIRECT => ErrorCode::InvalidRedirect,
            cef_errorcode_t::ERR_TOO_MANY_REDIRECTS => ErrorCode::TooManyRedirects,
            cef_errorcode_t::ERR_UNSAFE_REDIRECT => ErrorCode::UnsafeRedirect,
            cef_errorcode_t::ERR_UNSAFE_PORT => ErrorCode::UnsafePort,
            cef_errorcode_t::ERR_INVALID_RESPONSE => ErrorCode::InvalidResponse,
            cef_errorcode_t::ERR_INVALID_CHUNKED_ENCODING => ErrorCode::InvalidChunkedEncoding,
            cef_errorcode_t::ERR_METHOD_NOT_SUPPORTED => ErrorCode::MethodNotSupported,
            cef_errorcode_t::ERR_UNEXPECTED_PROXY_AUTH => ErrorCode::UnexpectedProxyAuth,
            cef_errorcode_t::ERR_EMPTY_RESPONSE => ErrorCode::EmptyResponse,
            cef_errorcode_t::ERR_RESPONSE_HEADERS_TOO_BIG => ErrorCode::ResponseHeadersTooBig,
            cef_errorcode_t::ERR_PAC_SCRIPT_FAILED => ErrorCode::PacScriptFailed,
            cef_errorcode_t::ERR_REQUEST_RANGE_NOT_SATISFIABLE => ErrorCode::RequestedRangeNotSatisfiable,
            cef_errorcode_t::ERR_MALFORMED_IDENTITY => ErrorCode::MalformedIdentity,
            cef_errorcode_t::ERR_CONTENT_DECODING_FAILED => ErrorCode::ContentDecodingFailed,
            cef_errorcode_t::ERR_NETWORK_IO_SUSPENDED => ErrorCode::NetworkIoSuspended,
            cef_errorcode_t::ERR_SYN_REPLY_NOT_RECEIVED => ErrorCode::SynReplyNotReceived,
            cef_errorcode_t::ERR_ENCODING_CONVERSION_FAILED => ErrorCode::EncodingConversionFailed,
            cef_errorcode_t::ERR_UNRECOGNIZED_FTP_DIRECTORY_LISTING_FORMAT => ErrorCode::UnrecognizedFtpDirectoryListingFormat,
            cef_errorcode_t::ERR_NO_SUPPORTED_PROXIES => ErrorCode::NoSupportedProxies,
            cef_errorcode_t::ERR_HTTP2_PROTOCOL_ERROR => ErrorCode::Http2ProtocolError,
            cef_errorcode_t::ERR_INVALID_AUTH_CREDENTIALS => ErrorCode::InvalidAuthCredentials,
            cef_errorcode_t::ERR_UNSUPPORTED_AUTH_SCHEME => ErrorCode::UnsupportedAuthScheme,
            cef_errorcode_t::ERR_ENCODING_DETECTION_FAILED => ErrorCode::EncodingDetectionFailed,
            cef_errorcode_t::ERR_MISSING_AUTH_CREDENTIALS => ErrorCode::MissingAuthCredentials,
            cef_errorcode_t::ERR_UNEXPECTED_SECURITY_LIBRARY_STATUS => ErrorCode::UnexpectedSecurityLibraryStatus,
            cef_errorcode_t::ERR_MISCONFIGURED_AUTH_ENVIRONMENT => ErrorCode::MisconfiguredAuthEnvironment,
            cef_errorcode_t::ERR_UNDOCUMENTED_SECURITY_LIBRARY_STATUS => ErrorCode::UndocumentedSecurityLibraryStatus,
            cef_errorcode_t::ERR_RESPONSE_BODY_TOO_BIG_TO_DRAIN => ErrorCode::ResponseBodyTooBigToDrain,
            cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_CONTENT_LENGTH => ErrorCode::ResponseHeadersMultipleContentLength,
            cef_errorcode_t::ERR_INCOMPLETE_HTTP2_HEADERS => ErrorCode::IncompleteHttp2Headers,
            cef_errorcode_t::ERR_PAC_NOT_IN_DHCP => ErrorCode::PacNotInDhcp,
            cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_CONTENT_DISPOSITION => ErrorCode::ResponseHeadersMultipleContentDisposition,
            cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_LOCATION => ErrorCode::ResponseHeadersMultipleLocation,
            cef_errorcode_t::ERR_HTTP2_SERVER_REFUSED_STREAM => ErrorCode::Http2ServerRefusedStream,
            cef_errorcode_t::ERR_HTTP2_PING_FAILED => ErrorCode::Http2PingFailed,
            cef_errorcode_t::ERR_CONTENT_LENGTH_MISMATCH => ErrorCode::ContentLengthMismatch,
            cef_errorcode_t::ERR_INCOMPLETE_CHUNKED_ENCODING => ErrorCode::IncompleteChunkedEncoding,
            cef_errorcode_t::ERR_QUIC_PROTOCOL_ERROR => ErrorCode::QuicProtocolError,
            cef_errorcode_t::ERR_RESPONSE_HEADERS_TRUNCATED => ErrorCode::ResponseHeadersTruncated,
            cef_errorcode_t::ERR_QUIC_HANDSHAKE_FAILED => ErrorCode::QuicHandshakeFailed,
            cef_errorcode_t::ERR_HTTP2_INADEQUATE_TRANSPORT_SECURITY => ErrorCode::Http2InadequateTransportSecurity,
            cef_errorcode_t::ERR_HTTP2_FLOW_CONTROL_ERROR => ErrorCode::Http2FlowControlError,
            cef_errorcode_t::ERR_HTTP2_FRAME_SIZE_ERROR => ErrorCode::Http2FrameSizeError,
            cef_errorcode_t::ERR_HTTP2_COMPRESSION_ERROR => ErrorCode::Http2CompressionError,
            cef_errorcode_t::ERR_PROXY_AUTH_REQUESTED_WITH_NO_CONNECTION => ErrorCode::ProxyAuthRequestedWithNoConnection,
            cef_errorcode_t::ERR_HTTP_1_1_REQUIRED => ErrorCode::Http11Required,
            cef_errorcode_t::ERR_PROXY_HTTP_1_1_REQUIRED => ErrorCode::ProxyHttp11Required,
            cef_errorcode_t::ERR_PAC_SCRIPT_TERMINATED => ErrorCode::PacScriptTerminated,
            cef_errorcode_t::ERR_INVALID_HTTP_RESPONSE => ErrorCode::InvalidHttpResponse,
            cef_errorcode_t::ERR_CONTENT_DECODING_INIT_FAILED => ErrorCode::ContentDecodingInitFailed,
            cef_errorcode_t::ERR_HTTP2_RST_STREAM_NO_ERROR_RECEIVED => ErrorCode::Http2RstStreamNoErrorReceived,
            cef_errorcode_t::ERR_TOO_MANY_RETRIES => ErrorCode::TooManyRetries,
            cef_errorcode_t::ERR_HTTP2_STREAM_CLOSED => ErrorCode::Http2StreamClosed,
            cef_errorcode_t::ERR_HTTP_RESPONSE_CODE_FAILURE => ErrorCode::HttpResponseCodeFailure,
            cef_errorcode_t::ERR_QUIC_CERT_ROOT_NOT_KNOWN => ErrorCode::QuicCertRootNotKnown,
            cef_errorcode_t::ERR_QUIC_GOAWAY_REQUEST_CAN_BE_RETRIED => ErrorCode::QuicGoawayRequestCanBeRetried,
            cef_errorcode_t::ERR_TOO_MANY_ACCEPT_CH_RESTARTS => ErrorCode::TooManyAcceptChRestarts,
            cef_errorcode_t::ERR_INCONSISTENT_IP_ADDRESS_SPACE => ErrorCode::InconsistentIpAddressSpace,
            cef_errorcode_t::ERR_CACHED_IP_ADDRESS_SPACE_BLOCKED_BY_PRIVATE_NETWORK_ACCESS_POLICY => ErrorCode::CachedIpAddressSpaceBlockedByPrivateNetworkAccessPolicy,
            cef_errorcode_t::ERR_BLOCKED_BY_PRIVATE_NETWORK_ACCESS_CHECKS => ErrorCode::BlockedByPrivateNetworkAccessChecks,
            cef_errorcode_t::ERR_CACHE_MISS => ErrorCode::CacheMiss,
            cef_errorcode_t::ERR_CACHE_READ_FAILURE => ErrorCode::CacheReadFailure,
            cef_errorcode_t::ERR_CACHE_WRITE_FAILURE => ErrorCode::CacheWriteFailure,
            cef_errorcode_t::ERR_CACHE_OPERATION_NOT_SUPPORTED => ErrorCode::CacheOperationNotSupported,
            cef_errorcode_t::ERR_CACHE_OPEN_FAILURE => ErrorCode::CacheOpenFailure,
            cef_errorcode_t::ERR_CACHE_CREATE_FAILURE => ErrorCode::CacheCreateFailure,
            cef_errorcode_t::ERR_CACHE_RACE => ErrorCode::CacheRace,
            cef_errorcode_t::ERR_CACHE_CHECKSUM_READ_FAILURE => ErrorCode::CacheChecksumReadFailure,
            cef_errorcode_t::ERR_CACHE_CHECKSUM_MISMATCH => ErrorCode::CacheChecksumMismatch,
            cef_errorcode_t::ERR_CACHE_LOCK_TIMEOUT => ErrorCode::CacheLockTimeout,
            cef_errorcode_t::ERR_CACHE_AUTH_FAILURE_AFTER_READ => ErrorCode::CacheAuthFailureAfterRead,
            cef_errorcode_t::ERR_CACHE_ENTRY_NOT_SUITABLE => ErrorCode::CacheEntryNotSuitable,
            cef_errorcode_t::ERR_CACHE_DOOM_FAILURE => ErrorCode::CacheDoomFailure,
            cef_errorcode_t::ERR_CACHE_OPEN_OR_CREATE_FAILURE => ErrorCode::CacheOpenOrCreateFailure,
            cef_errorcode_t::ERR_INSECURE_RESPONSE => ErrorCode::InsecureResponse,
            cef_errorcode_t::ERR_NO_PRIVATE_KEY_FOR_CERT => ErrorCode::NoPrivateKeyForCert,
            cef_errorcode_t::ERR_ADD_USER_CERT_FAILED => ErrorCode::AddUserCertFailed,
            cef_errorcode_t::ERR_INVALID_SIGNED_EXCHANGE => ErrorCode::InvalidSignedExchange,
            cef_errorcode_t::ERR_INVALID_WEB_BUNDLE => ErrorCode::InvalidWebBundle,
            cef_errorcode_t::ERR_TRUST_TOKEN_OPERATION_FAILED => ErrorCode::TrustTokenOperationFailed,
            cef_errorcode_t::ERR_TRUST_TOKEN_OPERATION_SUCCESS_WITHOUT_SENDING_REQUEST => ErrorCode::TrustTokenOperationSuccessWithoutSendingRequest,
            cef_errorcode_t::ERR_FTP_FAILED => ErrorCode::FtpFailed,
            cef_errorcode_t::ERR_FTP_SERVICE_UNAVAILABLE => ErrorCode::FtpServiceUnavailable,
            cef_errorcode_t::ERR_FTP_TRANSFER_ABORTED => ErrorCode::FtpTransferAborted,
            cef_errorcode_t::ERR_FTP_FILE_BUSY => ErrorCode::FtpFileBusy,
            cef_errorcode_t::ERR_FTP_SYNTAX_ERROR => ErrorCode::FtpSyntaxError,
            cef_errorcode_t::ERR_FTP_COMMAND_NOT_SUPPORTED => ErrorCode::FtpCommandNotSupported,
            cef_errorcode_t::ERR_FTP_BAD_COMMAND_SEQUENCE => ErrorCode::FtpBadCommandSequence,
            cef_errorcode_t::ERR_PKCS12_IMPORT_BAD_PASSWORD => ErrorCode::Pkcs12ImportBadPassword,
            cef_errorcode_t::ERR_PKCS12_IMPORT_FAILED => ErrorCode::Pkcs12ImportFailed,
            cef_errorcode_t::ERR_IMPORT_CA_CERT_NOT_CA => ErrorCode::ImportCaCertNotCa,
            cef_errorcode_t::ERR_IMPORT_CERT_ALREADY_EXISTS => ErrorCode::ImportCertAlreadyExists,
            cef_errorcode_t::ERR_IMPORT_CA_CERT_FAILED => ErrorCode::ImportCaCertFailed,
            cef_errorcode_t::ERR_IMPORT_SERVER_CERT_FAILED => ErrorCode::ImportServerCertFailed,
            cef_errorcode_t::ERR_PKCS12_IMPORT_INVALID_MAC => ErrorCode::Pkcs12ImportInvalidMac,
            cef_errorcode_t::ERR_PKCS12_IMPORT_INVALID_FILE => ErrorCode::Pkcs12ImportInvalidFile,
            cef_errorcode_t::ERR_PKCS12_IMPORT_UNSUPPORTED => ErrorCode::Pkcs12ImportUnsupported,
            cef_errorcode_t::ERR_KEY_GENERATION_FAILED => ErrorCode::KeyGenerationFailed,
            cef_errorcode_t::ERR_PRIVATE_KEY_EXPORT_FAILED => ErrorCode::PrivateKeyExportFailed,
            cef_errorcode_t::ERR_SELF_SIGNED_CERT_GENERATION_FAILED => ErrorCode::SelfSignedCertGenerationFailed,
            cef_errorcode_t::ERR_CERT_DATABASE_CHANGED => ErrorCode::CertDatabaseChanged,
            cef_errorcode_t::ERR_CERT_VERIFIER_CHANGED => ErrorCode::CertVerifierChanged,
            cef_errorcode_t::ERR_DNS_MALFORMED_RESPONSE => ErrorCode::DnsMalformedResponse,
            cef_errorcode_t::ERR_DNS_SERVER_REQUIRES_TCP => ErrorCode::DnsServerRequiresTcp,
            cef_errorcode_t::ERR_DNS_SERVER_FAILED => ErrorCode::DnsServerFailed,
            cef_errorcode_t::ERR_DNS_TIMED_OUT => ErrorCode::DnsTimedOut,
            cef_errorcode_t::ERR_DNS_CACHE_MISS => ErrorCode::DnsCacheMiss,
            cef_errorcode_t::ERR_DNS_SEARCH_EMPTY => ErrorCode::DnsSearchEmpty,
            cef_errorcode_t::ERR_DNS_SORT_ERROR => ErrorCode::DnsSortError,
            cef_errorcode_t::ERR_DNS_SECURE_RESOLVER_HOSTNAME_RESOLUTION_FAILED => ErrorCode::DnsSecureResolverHostnameResolutionFailed,
            cef_errorcode_t::ERR_DNS_NAME_HTTPS_ONLY => ErrorCode::DnsNameHttpsOnly,
            cef_errorcode_t::ERR_DNS_REQUEST_CANCELLED => ErrorCode::DnsRequestCancelled,
            cef_errorcode_t::ERR_DNS_NO_MATCHING_SUPPORTED_ALPN => ErrorCode::DnsNoMatchingSupportedAlpn,
            cef_errorcode_t::ERR_DICTIONARY_LOAD_FAILED => ErrorCode::DictionaryLoadFailed
        }
    }
}

impl From<ErrorCode> for cef_errorcode_t {
    fn from(value: ErrorCode) -> Self {
        cef_errorcode_t::from(&value)
    }
}

impl From<&ErrorCode> for cef_errorcode_t {
    fn from(value: &ErrorCode) -> Self {
        match value {
            ErrorCode::None => cef_errorcode_t::ERR_NONE,
            ErrorCode::IoPending => cef_errorcode_t::ERR_IO_PENDING,
            ErrorCode::Failed => cef_errorcode_t::ERR_FAILED,
            ErrorCode::Aborted => cef_errorcode_t::ERR_ABORTED,
            ErrorCode::InvalidArgument => cef_errorcode_t::ERR_INVALID_ARGUMENT,
            ErrorCode::InvalidHandle => cef_errorcode_t::ERR_INVALID_HANDLE,
            ErrorCode::FileNotFound => cef_errorcode_t::ERR_FILE_NOT_FOUND,
            ErrorCode::TimedOut => cef_errorcode_t::ERR_TIMED_OUT,
            ErrorCode::FileTooBig => cef_errorcode_t::ERR_FILE_TOO_BIG,
            ErrorCode::Unexpected => cef_errorcode_t::ERR_UNEXPECTED,
            ErrorCode::AccessDenied => cef_errorcode_t::ERR_ACCESS_DENIED,
            ErrorCode::NotImplemented => cef_errorcode_t::ERR_NOT_IMPLEMENTED,
            ErrorCode::InsufficientResources => cef_errorcode_t::ERR_INSUFFICIENT_RESOURCES,
            ErrorCode::OutOfMemory => cef_errorcode_t::ERR_OUT_OF_MEMORY,
            ErrorCode::UploadFileChanged => cef_errorcode_t::ERR_UPLOAD_FILE_CHANGED,
            ErrorCode::SocketNotConnected => cef_errorcode_t::ERR_SOCKET_NOT_CONNECTED,
            ErrorCode::FileExists => cef_errorcode_t::ERR_FILE_EXISTS,
            ErrorCode::FilePathTooLong => cef_errorcode_t::ERR_FILE_PATH_TOO_LONG,
            ErrorCode::FileNoSpace => cef_errorcode_t::ERR_FILE_NO_SPACE,
            ErrorCode::FileVirusInfected => cef_errorcode_t::ERR_FILE_VIRUS_INFECTED,
            ErrorCode::BlockedByClient => cef_errorcode_t::ERR_BLOCKED_BY_CLIENT,
            ErrorCode::NetworkChanged => cef_errorcode_t::ERR_NETWORK_CHANGED,
            ErrorCode::BlockedByAdministrator => cef_errorcode_t::ERR_BLOCKED_BY_ADMINISTRATOR,
            ErrorCode::SocketIsConnected => cef_errorcode_t::ERR_SOCKET_IS_CONNECTED,
            ErrorCode::UploadStreamRewindNotSupported => cef_errorcode_t::ERR_UPLOAD_STREAM_REWIND_NOT_SUPPORTED,
            ErrorCode::ContextShutDown => cef_errorcode_t::ERR_CONTEXT_SHUT_DOWN,
            ErrorCode::BlockedByResponse => cef_errorcode_t::ERR_BLOCKED_BY_RESPONSE,
            ErrorCode::CleartextNotPermitted => cef_errorcode_t::ERR_CLEARTEXT_NOT_PERMITTED,
            ErrorCode::BlockedByCsp => cef_errorcode_t::ERR_BLOCKED_BY_CSP,
            ErrorCode::H2OrQuicRequired => cef_errorcode_t::ERR_H2_OR_QUIC_REQUIRED,
            ErrorCode::BlockedByOrb => cef_errorcode_t::ERR_BLOCKED_BY_ORB,
            ErrorCode::ConnectionClosed => cef_errorcode_t::ERR_CONNECTION_CLOSED,
            ErrorCode::ConnectionReset => cef_errorcode_t::ERR_CONNECTION_RESET,
            ErrorCode::ConnectionRefused => cef_errorcode_t::ERR_CONNECTION_REFUSED,
            ErrorCode::ConnectionAborted => cef_errorcode_t::ERR_CONNECTION_ABORTED,
            ErrorCode::ConnectionFailed => cef_errorcode_t::ERR_CONNECTION_FAILED,
            ErrorCode::NameNotResolved => cef_errorcode_t::ERR_NAME_NOT_RESOLVED,
            ErrorCode::InternetDisconnected => cef_errorcode_t::ERR_INTERNET_DISCONNECTED,
            ErrorCode::SslProtocolError => cef_errorcode_t::ERR_SSL_PROTOCOL_ERROR,
            ErrorCode::AddressInvalid => cef_errorcode_t::ERR_ADDRESS_INVALID,
            ErrorCode::AddressUnreachable => cef_errorcode_t::ERR_ADDRESS_UNREACHABLE,
            ErrorCode::SslClientAuthCertNeeded => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_NEEDED,
            ErrorCode::TunnelConnectionFailed => cef_errorcode_t::ERR_TUNNEL_CONNECTION_FAILED,
            ErrorCode::NoSslVersionsEnabled => cef_errorcode_t::ERR_NO_SSL_VERSIONS_ENABLED,
            ErrorCode::SslVersionOrCipherMismatch => cef_errorcode_t::ERR_SSL_VERSION_OR_CIPHER_MISMATCH,
            ErrorCode::SslRenegotiationRequested => cef_errorcode_t::ERR_SSL_RENEGOTIATION_REQUESTED,
            ErrorCode::ProxyAuthUnsupported => cef_errorcode_t::ERR_PROXY_AUTH_UNSUPPORTED,
            ErrorCode::BadSslClientAuthCert => cef_errorcode_t::ERR_BAD_SSL_CLIENT_AUTH_CERT,
            ErrorCode::ConnectionTimedOut => cef_errorcode_t::ERR_CONNECTION_TIMED_OUT,
            ErrorCode::HostResolverQueueTooLarge => cef_errorcode_t::ERR_HOST_RESOLVER_QUEUE_TOO_LARGE,
            ErrorCode::SocksConnectionFailed => cef_errorcode_t::ERR_SOCKS_CONNECTION_FAILED,
            ErrorCode::SocksConnectionHostUnreachable => cef_errorcode_t::ERR_SOCKS_CONNECTION_HOST_UNREACHABLE,
            ErrorCode::AlpnNegotiationFailed => cef_errorcode_t::ERR_ALPN_NEGOTIATION_FAILED,
            ErrorCode::SslNoRenegotiation => cef_errorcode_t::ERR_SSL_NO_RENEGOTIATION,
            ErrorCode::WinsockUnexpectedWrittenBytes => cef_errorcode_t::ERR_WINSOCK_UNEXPECTED_WRITTEN_BYTES,
            ErrorCode::SslDecompressionFailureAlert => cef_errorcode_t::ERR_SSL_DECOMPRESSION_FAILURE_ALERT,
            ErrorCode::SslBadRecordMacAlert => cef_errorcode_t::ERR_SSL_BAD_RECORD_MAC_ALERT,
            ErrorCode::ProxyAuthRequested => cef_errorcode_t::ERR_PROXY_AUTH_REQUESTED,
            ErrorCode::ProxyConnectionFailed => cef_errorcode_t::ERR_PROXY_CONNECTION_FAILED,
            ErrorCode::MandatoryProxyConfigurationFailed => cef_errorcode_t::ERR_MANDATORY_PROXY_CONFIGURATION_FAILED,
            ErrorCode::PreconnectMaxSocketLimit => cef_errorcode_t::ERR_PRECONNECT_MAX_SOCKET_LIMIT,
            ErrorCode::SslClientAuthPrivateKeyAccessDenied => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_PRIVATE_KEY_ACCESS_DENIED,
            ErrorCode::SslClientAuthCertNoPrivateKey => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_NO_PRIVATE_KEY,
            ErrorCode::ProxyCertificateInvalid => cef_errorcode_t::ERR_PROXY_CERTIFICATE_INVALID,
            ErrorCode::NameResolutionFailed => cef_errorcode_t::ERR_NAME_RESOLUTION_FAILED,
            ErrorCode::NetworkAccessDenied => cef_errorcode_t::ERR_NETWORK_ACCESS_DENIED,
            ErrorCode::TemporarilyThrottled => cef_errorcode_t::ERR_TEMPORARILY_THROTTLED,
            ErrorCode::HttpsProxyTunnelResponseRedirect => cef_errorcode_t::ERR_HTTPS_PROXY_TUNNEL_RESPONSE_REDIRECT,
            ErrorCode::SslClientAuthSignatureFailed => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_SIGNATURE_FAILED,
            ErrorCode::MsgTooBig => cef_errorcode_t::ERR_MSG_TOO_BIG,
            ErrorCode::WsProtocolError => cef_errorcode_t::ERR_WS_PROTOCOL_ERROR,
            ErrorCode::AddressInUse => cef_errorcode_t::ERR_ADDRESS_IN_USE,
            ErrorCode::SslHandshakeNotCompleted => cef_errorcode_t::ERR_SSL_HANDSHAKE_NOT_COMPLETED,
            ErrorCode::SslBadPeerPublicKey => cef_errorcode_t::ERR_SSL_BAD_PEER_PUBLIC_KEY,
            ErrorCode::SslPinnedKeyNotInCertChain => cef_errorcode_t::ERR_SSL_PINNED_KEY_NOT_IN_CERT_CHAIN,
            ErrorCode::ClientAuthCertTypeUnsupported => cef_errorcode_t::ERR_CLIENT_AUTH_CERT_TYPE_UNSUPPORTED,
            ErrorCode::SslDecryptErrorAlert => cef_errorcode_t::ERR_SSL_DECRYPT_ERROR_ALERT,
            ErrorCode::WsThrottleQueueTooLarge => cef_errorcode_t::ERR_WS_THROTTLE_QUEUE_TOO_LARGE,
            ErrorCode::SslServerCertChanged => cef_errorcode_t::ERR_SSL_SERVER_CERT_CHANGED,
            ErrorCode::SslUnrecognizedNameAlert => cef_errorcode_t::ERR_SSL_UNRECOGNIZED_NAME_ALERT,
            ErrorCode::SocketSetReceiveBufferSizeError => cef_errorcode_t::ERR_SOCKET_SET_RECEIVE_BUFFER_SIZE_ERROR,
            ErrorCode::SocketSetSendBufferSizeError => cef_errorcode_t::ERR_SOCKET_SET_SEND_BUFFER_SIZE_ERROR,
            ErrorCode::SocketReceiveBufferSizeUnchangeable => cef_errorcode_t::ERR_SOCKET_RECEIVE_BUFFER_SIZE_UNCHANGEABLE,
            ErrorCode::SocketSendBufferSizeUnchangeable => cef_errorcode_t::ERR_SOCKET_SEND_BUFFER_SIZE_UNCHANGEABLE,
            ErrorCode::SslClientAuthCertBadFormat => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_CERT_BAD_FORMAT,
            ErrorCode::IcannNameCollision => cef_errorcode_t::ERR_ICANN_NAME_COLLISION,
            ErrorCode::SslServerCertBadFormat => cef_errorcode_t::ERR_SSL_SERVER_CERT_BAD_FORMAT,
            ErrorCode::CtSthParsingFailed => cef_errorcode_t::ERR_CT_STH_PARSING_FAILED,
            ErrorCode::CtSthIncomplete => cef_errorcode_t::ERR_CT_STH_INCOMPLETE,
            ErrorCode::UnableToReuseConnectionForProxyAuth => cef_errorcode_t::ERR_UNABLE_TO_REUSE_CONNECTION_FOR_PROXY_AUTH,
            ErrorCode::CtConsistencyProofParsingFailed => cef_errorcode_t::ERR_CT_CONSISTENCY_PROOF_PARSING_FAILED,
            ErrorCode::SslObsoleteCipher => cef_errorcode_t::ERR_SSL_OBSOLETE_CIPHER,
            ErrorCode::WsUpgrade => cef_errorcode_t::ERR_WS_UPGRADE,
            ErrorCode::ReadIfReadyNotImplemented => cef_errorcode_t::ERR_READ_IF_READY_NOT_IMPLEMENTED,
            ErrorCode::NoBufferSpace => cef_errorcode_t::ERR_NO_BUFFER_SPACE,
            ErrorCode::SslClientAuthNoCommonAlgorithms => cef_errorcode_t::ERR_SSL_CLIENT_AUTH_NO_COMMON_ALGORITHMS,
            ErrorCode::EarlyDataRejected => cef_errorcode_t::ERR_EARLY_DATA_REJECTED,
            ErrorCode::WrongVersionOnEarlyData => cef_errorcode_t::ERR_WRONG_VERSION_ON_EARLY_DATA,
            ErrorCode::Tls13DowngradeDetected => cef_errorcode_t::ERR_TLS13_DOWNGRADE_DETECTED,
            ErrorCode::SslKeyUsageIncompatible => cef_errorcode_t::ERR_SSL_KEY_USAGE_INCOMPATIBLE,
            ErrorCode::InvalidEchConfigList => cef_errorcode_t::ERR_INVALID_ECH_CONFIG_LIST,
            ErrorCode::EchNotNegotiated => cef_errorcode_t::ERR_ECH_NOT_NEGOTIATED,
            ErrorCode::EchFallbackCertificateInvalid => cef_errorcode_t::ERR_ECH_FALLBACK_CERTIFICATE_INVALID,
            ErrorCode::CertCommonNameInvalid => cef_errorcode_t::ERR_CERT_COMMON_NAME_INVALID,
            ErrorCode::CertDateInvalid => cef_errorcode_t::ERR_CERT_DATE_INVALID,
            ErrorCode::CertAuthorityInvalid => cef_errorcode_t::ERR_CERT_AUTHORITY_INVALID,
            ErrorCode::CertContainsErrors => cef_errorcode_t::ERR_CERT_CONTAINS_ERRORS,
            ErrorCode::CertNoRevocationMechanism => cef_errorcode_t::ERR_CERT_NO_REVOCATION_MECHANISM,
            ErrorCode::CertUnableToCheckRevocation => cef_errorcode_t::ERR_CERT_UNABLE_TO_CHECK_REVOCATION,
            ErrorCode::CertRevoked => cef_errorcode_t::ERR_CERT_REVOKED,
            ErrorCode::CertInvalid => cef_errorcode_t::ERR_CERT_INVALID,
            ErrorCode::CertWeakSignatureAlgorithm => cef_errorcode_t::ERR_CERT_WEAK_SIGNATURE_ALGORITHM,
            ErrorCode::CertNonUniqueName => cef_errorcode_t::ERR_CERT_NON_UNIQUE_NAME,
            ErrorCode::CertWeakKey => cef_errorcode_t::ERR_CERT_WEAK_KEY,
            ErrorCode::CertNameConstraintViolation => cef_errorcode_t::ERR_CERT_NAME_CONSTRAINT_VIOLATION,
            ErrorCode::CertValidityTooLong => cef_errorcode_t::ERR_CERT_VALIDITY_TOO_LONG,
            ErrorCode::CertificateTransparencyRequired => cef_errorcode_t::ERR_CERTIFICATE_TRANSPARENCY_REQUIRED,
            ErrorCode::CertSymantecLegacy => cef_errorcode_t::ERR_CERT_SYMANTEC_LEGACY,
            ErrorCode::CertKnownInterceptionBlocked => cef_errorcode_t::ERR_CERT_KNOWN_INTERCEPTION_BLOCKED,
            ErrorCode::CertEnd => cef_errorcode_t::ERR_CERT_END,
            ErrorCode::InvalidUrl => cef_errorcode_t::ERR_INVALID_URL,
            ErrorCode::DisallowedUrlScheme => cef_errorcode_t::ERR_DISALLOWED_URL_SCHEME,
            ErrorCode::UnknownUrlScheme => cef_errorcode_t::ERR_UNKNOWN_URL_SCHEME,
            ErrorCode::InvalidRedirect => cef_errorcode_t::ERR_INVALID_REDIRECT,
            ErrorCode::TooManyRedirects => cef_errorcode_t::ERR_TOO_MANY_REDIRECTS,
            ErrorCode::UnsafeRedirect => cef_errorcode_t::ERR_UNSAFE_REDIRECT,
            ErrorCode::UnsafePort => cef_errorcode_t::ERR_UNSAFE_PORT,
            ErrorCode::InvalidResponse => cef_errorcode_t::ERR_INVALID_RESPONSE,
            ErrorCode::InvalidChunkedEncoding => cef_errorcode_t::ERR_INVALID_CHUNKED_ENCODING,
            ErrorCode::MethodNotSupported => cef_errorcode_t::ERR_METHOD_NOT_SUPPORTED,
            ErrorCode::UnexpectedProxyAuth => cef_errorcode_t::ERR_UNEXPECTED_PROXY_AUTH,
            ErrorCode::EmptyResponse => cef_errorcode_t::ERR_EMPTY_RESPONSE,
            ErrorCode::ResponseHeadersTooBig => cef_errorcode_t::ERR_RESPONSE_HEADERS_TOO_BIG,
            ErrorCode::PacScriptFailed => cef_errorcode_t::ERR_PAC_SCRIPT_FAILED,
            ErrorCode::RequestedRangeNotSatisfiable => cef_errorcode_t::ERR_REQUEST_RANGE_NOT_SATISFIABLE,
            ErrorCode::MalformedIdentity => cef_errorcode_t::ERR_MALFORMED_IDENTITY,
            ErrorCode::ContentDecodingFailed => cef_errorcode_t::ERR_CONTENT_DECODING_FAILED,
            ErrorCode::NetworkIoSuspended => cef_errorcode_t::ERR_NETWORK_IO_SUSPENDED,
            ErrorCode::SynReplyNotReceived => cef_errorcode_t::ERR_SYN_REPLY_NOT_RECEIVED,
            ErrorCode::EncodingConversionFailed => cef_errorcode_t::ERR_ENCODING_CONVERSION_FAILED,
            ErrorCode::UnrecognizedFtpDirectoryListingFormat => cef_errorcode_t::ERR_UNRECOGNIZED_FTP_DIRECTORY_LISTING_FORMAT,
            ErrorCode::NoSupportedProxies => cef_errorcode_t::ERR_NO_SUPPORTED_PROXIES,
            ErrorCode::Http2ProtocolError => cef_errorcode_t::ERR_HTTP2_PROTOCOL_ERROR,
            ErrorCode::InvalidAuthCredentials => cef_errorcode_t::ERR_INVALID_AUTH_CREDENTIALS,
            ErrorCode::UnsupportedAuthScheme => cef_errorcode_t::ERR_UNSUPPORTED_AUTH_SCHEME,
            ErrorCode::EncodingDetectionFailed => cef_errorcode_t::ERR_ENCODING_DETECTION_FAILED,
            ErrorCode::MissingAuthCredentials => cef_errorcode_t::ERR_MISSING_AUTH_CREDENTIALS,
            ErrorCode::UnexpectedSecurityLibraryStatus => cef_errorcode_t::ERR_UNEXPECTED_SECURITY_LIBRARY_STATUS,
            ErrorCode::MisconfiguredAuthEnvironment => cef_errorcode_t::ERR_MISCONFIGURED_AUTH_ENVIRONMENT,
            ErrorCode::UndocumentedSecurityLibraryStatus => cef_errorcode_t::ERR_UNDOCUMENTED_SECURITY_LIBRARY_STATUS,
            ErrorCode::ResponseBodyTooBigToDrain => cef_errorcode_t::ERR_RESPONSE_BODY_TOO_BIG_TO_DRAIN,
            ErrorCode::ResponseHeadersMultipleContentLength => cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_CONTENT_LENGTH,
            ErrorCode::IncompleteHttp2Headers => cef_errorcode_t::ERR_INCOMPLETE_HTTP2_HEADERS,
            ErrorCode::PacNotInDhcp => cef_errorcode_t::ERR_PAC_NOT_IN_DHCP,
            ErrorCode::ResponseHeadersMultipleContentDisposition => cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_CONTENT_DISPOSITION,
            ErrorCode::ResponseHeadersMultipleLocation => cef_errorcode_t::ERR_RESPONSE_HEADERS_MULTIPLE_LOCATION,
            ErrorCode::Http2ServerRefusedStream => cef_errorcode_t::ERR_HTTP2_SERVER_REFUSED_STREAM,
            ErrorCode::Http2PingFailed => cef_errorcode_t::ERR_HTTP2_PING_FAILED,
            ErrorCode::ContentLengthMismatch => cef_errorcode_t::ERR_CONTENT_LENGTH_MISMATCH,
            ErrorCode::IncompleteChunkedEncoding => cef_errorcode_t::ERR_INCOMPLETE_CHUNKED_ENCODING,
            ErrorCode::QuicProtocolError => cef_errorcode_t::ERR_QUIC_PROTOCOL_ERROR,
            ErrorCode::ResponseHeadersTruncated => cef_errorcode_t::ERR_RESPONSE_HEADERS_TRUNCATED,
            ErrorCode::QuicHandshakeFailed => cef_errorcode_t::ERR_QUIC_HANDSHAKE_FAILED,
            ErrorCode::Http2InadequateTransportSecurity => cef_errorcode_t::ERR_HTTP2_INADEQUATE_TRANSPORT_SECURITY,
            ErrorCode::Http2FlowControlError => cef_errorcode_t::ERR_HTTP2_FLOW_CONTROL_ERROR,
            ErrorCode::Http2FrameSizeError => cef_errorcode_t::ERR_HTTP2_FRAME_SIZE_ERROR,
            ErrorCode::Http2CompressionError => cef_errorcode_t::ERR_HTTP2_COMPRESSION_ERROR,
            ErrorCode::ProxyAuthRequestedWithNoConnection => cef_errorcode_t::ERR_PROXY_AUTH_REQUESTED_WITH_NO_CONNECTION,
            ErrorCode::Http11Required => cef_errorcode_t::ERR_HTTP_1_1_REQUIRED,
            ErrorCode::ProxyHttp11Required => cef_errorcode_t::ERR_PROXY_HTTP_1_1_REQUIRED,
            ErrorCode::PacScriptTerminated => cef_errorcode_t::ERR_PAC_SCRIPT_TERMINATED,
            ErrorCode::InvalidHttpResponse => cef_errorcode_t::ERR_INVALID_HTTP_RESPONSE,
            ErrorCode::ContentDecodingInitFailed => cef_errorcode_t::ERR_CONTENT_DECODING_INIT_FAILED,
            ErrorCode::Http2RstStreamNoErrorReceived => cef_errorcode_t::ERR_HTTP2_RST_STREAM_NO_ERROR_RECEIVED,
            ErrorCode::TooManyRetries => cef_errorcode_t::ERR_TOO_MANY_RETRIES,
            ErrorCode::Http2StreamClosed => cef_errorcode_t::ERR_HTTP2_STREAM_CLOSED,
            ErrorCode::HttpResponseCodeFailure => cef_errorcode_t::ERR_HTTP_RESPONSE_CODE_FAILURE,
            ErrorCode::QuicCertRootNotKnown => cef_errorcode_t::ERR_QUIC_CERT_ROOT_NOT_KNOWN,
            ErrorCode::QuicGoawayRequestCanBeRetried => cef_errorcode_t::ERR_QUIC_GOAWAY_REQUEST_CAN_BE_RETRIED,
            ErrorCode::TooManyAcceptChRestarts => cef_errorcode_t::ERR_TOO_MANY_ACCEPT_CH_RESTARTS,
            ErrorCode::InconsistentIpAddressSpace => cef_errorcode_t::ERR_INCONSISTENT_IP_ADDRESS_SPACE,
            ErrorCode::CachedIpAddressSpaceBlockedByPrivateNetworkAccessPolicy => cef_errorcode_t::ERR_CACHED_IP_ADDRESS_SPACE_BLOCKED_BY_PRIVATE_NETWORK_ACCESS_POLICY,
            ErrorCode::BlockedByPrivateNetworkAccessChecks => cef_errorcode_t::ERR_BLOCKED_BY_PRIVATE_NETWORK_ACCESS_CHECKS,
            ErrorCode::CacheMiss => cef_errorcode_t::ERR_CACHE_MISS,
            ErrorCode::CacheReadFailure => cef_errorcode_t::ERR_CACHE_READ_FAILURE,
            ErrorCode::CacheWriteFailure => cef_errorcode_t::ERR_CACHE_WRITE_FAILURE,
            ErrorCode::CacheOperationNotSupported => cef_errorcode_t::ERR_CACHE_OPERATION_NOT_SUPPORTED,
            ErrorCode::CacheOpenFailure => cef_errorcode_t::ERR_CACHE_OPEN_FAILURE,
            ErrorCode::CacheCreateFailure => cef_errorcode_t::ERR_CACHE_CREATE_FAILURE,
            ErrorCode::CacheRace => cef_errorcode_t::ERR_CACHE_RACE,
            ErrorCode::CacheChecksumReadFailure => cef_errorcode_t::ERR_CACHE_CHECKSUM_READ_FAILURE,
            ErrorCode::CacheChecksumMismatch => cef_errorcode_t::ERR_CACHE_CHECKSUM_MISMATCH,
            ErrorCode::CacheLockTimeout => cef_errorcode_t::ERR_CACHE_LOCK_TIMEOUT,
            ErrorCode::CacheAuthFailureAfterRead => cef_errorcode_t::ERR_CACHE_AUTH_FAILURE_AFTER_READ,
            ErrorCode::CacheEntryNotSuitable => cef_errorcode_t::ERR_CACHE_ENTRY_NOT_SUITABLE,
            ErrorCode::CacheDoomFailure => cef_errorcode_t::ERR_CACHE_DOOM_FAILURE,
            ErrorCode::CacheOpenOrCreateFailure => cef_errorcode_t::ERR_CACHE_OPEN_OR_CREATE_FAILURE,
            ErrorCode::InsecureResponse => cef_errorcode_t::ERR_INSECURE_RESPONSE,
            ErrorCode::NoPrivateKeyForCert => cef_errorcode_t::ERR_NO_PRIVATE_KEY_FOR_CERT,
            ErrorCode::AddUserCertFailed => cef_errorcode_t::ERR_ADD_USER_CERT_FAILED,
            ErrorCode::InvalidSignedExchange => cef_errorcode_t::ERR_INVALID_SIGNED_EXCHANGE,
            ErrorCode::InvalidWebBundle => cef_errorcode_t::ERR_INVALID_WEB_BUNDLE,
            ErrorCode::TrustTokenOperationFailed => cef_errorcode_t::ERR_TRUST_TOKEN_OPERATION_FAILED,
            ErrorCode::TrustTokenOperationSuccessWithoutSendingRequest => cef_errorcode_t::ERR_TRUST_TOKEN_OPERATION_SUCCESS_WITHOUT_SENDING_REQUEST,
            ErrorCode::FtpFailed => cef_errorcode_t::ERR_FTP_FAILED,
            ErrorCode::FtpServiceUnavailable => cef_errorcode_t::ERR_FTP_SERVICE_UNAVAILABLE,
            ErrorCode::FtpTransferAborted => cef_errorcode_t::ERR_FTP_TRANSFER_ABORTED,
            ErrorCode::FtpFileBusy => cef_errorcode_t::ERR_FTP_FILE_BUSY,
            ErrorCode::FtpSyntaxError => cef_errorcode_t::ERR_FTP_SYNTAX_ERROR,
            ErrorCode::FtpCommandNotSupported => cef_errorcode_t::ERR_FTP_COMMAND_NOT_SUPPORTED,
            ErrorCode::FtpBadCommandSequence => cef_errorcode_t::ERR_FTP_BAD_COMMAND_SEQUENCE,
            ErrorCode::Pkcs12ImportBadPassword => cef_errorcode_t::ERR_PKCS12_IMPORT_BAD_PASSWORD,
            ErrorCode::Pkcs12ImportFailed => cef_errorcode_t::ERR_PKCS12_IMPORT_FAILED,
            ErrorCode::ImportCaCertNotCa => cef_errorcode_t::ERR_IMPORT_CA_CERT_NOT_CA,
            ErrorCode::ImportCertAlreadyExists => cef_errorcode_t::ERR_IMPORT_CERT_ALREADY_EXISTS,
            ErrorCode::ImportCaCertFailed => cef_errorcode_t::ERR_IMPORT_CA_CERT_FAILED,
            ErrorCode::ImportServerCertFailed => cef_errorcode_t::ERR_IMPORT_SERVER_CERT_FAILED,
            ErrorCode::Pkcs12ImportInvalidMac => cef_errorcode_t::ERR_PKCS12_IMPORT_INVALID_MAC,
            ErrorCode::Pkcs12ImportInvalidFile => cef_errorcode_t::ERR_PKCS12_IMPORT_INVALID_FILE,
            ErrorCode::Pkcs12ImportUnsupported => cef_errorcode_t::ERR_PKCS12_IMPORT_UNSUPPORTED,
            ErrorCode::KeyGenerationFailed => cef_errorcode_t::ERR_KEY_GENERATION_FAILED,
            ErrorCode::PrivateKeyExportFailed => cef_errorcode_t::ERR_PRIVATE_KEY_EXPORT_FAILED,
            ErrorCode::SelfSignedCertGenerationFailed => cef_errorcode_t::ERR_SELF_SIGNED_CERT_GENERATION_FAILED,
            ErrorCode::CertDatabaseChanged => cef_errorcode_t::ERR_CERT_DATABASE_CHANGED,
            ErrorCode::CertVerifierChanged => cef_errorcode_t::ERR_CERT_VERIFIER_CHANGED,
            ErrorCode::DnsMalformedResponse => cef_errorcode_t::ERR_DNS_MALFORMED_RESPONSE,
            ErrorCode::DnsServerRequiresTcp => cef_errorcode_t::ERR_DNS_SERVER_REQUIRES_TCP,
            ErrorCode::DnsServerFailed => cef_errorcode_t::ERR_DNS_SERVER_FAILED,
            ErrorCode::DnsTimedOut => cef_errorcode_t::ERR_DNS_TIMED_OUT,
            ErrorCode::DnsCacheMiss => cef_errorcode_t::ERR_DNS_CACHE_MISS,
            ErrorCode::DnsSearchEmpty => cef_errorcode_t::ERR_DNS_SEARCH_EMPTY,
            ErrorCode::DnsSortError => cef_errorcode_t::ERR_DNS_SORT_ERROR,
            ErrorCode::DnsSecureResolverHostnameResolutionFailed => cef_errorcode_t::ERR_DNS_SECURE_RESOLVER_HOSTNAME_RESOLUTION_FAILED,
            ErrorCode::DnsNameHttpsOnly => cef_errorcode_t::ERR_DNS_NAME_HTTPS_ONLY,
            ErrorCode::DnsRequestCancelled => cef_errorcode_t::ERR_DNS_REQUEST_CANCELLED,
            ErrorCode::DnsNoMatchingSupportedAlpn => cef_errorcode_t::ERR_DNS_NO_MATCHING_SUPPORTED_ALPN,
            ErrorCode::DictionaryLoadFailed => cef_errorcode_t::ERR_DICTIONARY_LOAD_FAILED
        }
    }
}

/// Represents the state of a setting.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    /// Use the default state for the setting.
    Default,

    /// Enable or allow the setting.
    Enabled,

    /// Disable or disallow the setting.
    Disabled
}

impl Default for State {
    fn default() -> Self {
        State::Default
    }
}

impl From<cef_state_t> for State {
    fn from(value: cef_state_t) -> Self {
        State::from(&value)
    }
}

impl From<&cef_state_t> for State {
    fn from(value: &cef_state_t) -> Self {
        match value {
            cef_state_t::STATE_DEFAULT => Self::Default,
            cef_state_t::STATE_ENABLED => Self::Enabled,
            cef_state_t::STATE_DISABLED => Self::Disabled
        }
    }
}

impl From<State> for cef_state_t {
    fn from(value: State) -> Self {
        cef_state_t::from(&value)
    }
}

impl From<&State> for cef_state_t {
    fn from(value: &State) -> Self {
        match value {
            State::Default => Self::STATE_DEFAULT,
            State::Enabled => Self::STATE_ENABLED,
            State::Disabled => Self::STATE_DISABLED
        }
    }
}

/// Log severity levels.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogSeverity {
    /// Default logging (currently info).
    Default,

    /// Verbose logging.
    Verbose,

    /// Info logging.
    Info,

    /// Warning logging.
    Warning,

    /// Error logging.
    Error,

    /// Fatal logging.
    Fatal,

    /// Disable logging to file for all messages, and to
    /// stderr for messages with severity less than fatal.
    Disable
}

impl Default for LogSeverity {
    fn default() -> Self {
        LogSeverity::Default
    }
}

impl From<cef_log_severity_t> for LogSeverity {
    fn from(value: cef_log_severity_t) -> Self {
        LogSeverity::from(&value)
    }
}

impl From<&cef_log_severity_t> for LogSeverity {
    fn from(value: &cef_log_severity_t) -> Self {
        match value {
            cef_log_severity_t::LOGSEVERITY_DEFAULT => Self::Default,
            cef_log_severity_t::LOGSEVERITY_VERBOSE => Self::Verbose,
            cef_log_severity_t::LOGSEVERITY_INFO => Self::Info,
            cef_log_severity_t::LOGSEVERITY_WARNING => Self::Warning,
            cef_log_severity_t::LOGSEVERITY_ERROR => Self::Error,
            cef_log_severity_t::LOGSEVERITY_FATAL => Self::Fatal,
            cef_log_severity_t::LOGSEVERITY_DISABLE => Self::Disable
        }
    }
}

impl From<LogSeverity> for cef_log_severity_t {
    fn from(value: LogSeverity) -> Self {
        cef_log_severity_t::from(&value)
    }
}

impl From<&LogSeverity> for cef_log_severity_t {
    fn from(value: &LogSeverity) -> Self {
        match value {
            LogSeverity::Default => Self::LOGSEVERITY_DEFAULT,
            LogSeverity::Verbose => Self::LOGSEVERITY_VERBOSE,
            LogSeverity::Info => Self::LOGSEVERITY_INFO,
            LogSeverity::Warning => Self::LOGSEVERITY_WARNING,
            LogSeverity::Error => Self::LOGSEVERITY_ERROR,
            LogSeverity::Fatal => Self::LOGSEVERITY_FATAL,
            LogSeverity::Disable => Self::LOGSEVERITY_DISABLE
        }
    }
}

/// Log items prepended to each log line.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LogItems {
    /// Prepend the default list of items.
    Default,

    /// Prepend no items.
    None,

    /// Prepend the process ID.
    FlagProcessId,

    /// Prepend the thread ID.
    FlagThreadId,

    /// Prepend the timestamp.
    FlagTimeStamp,

    /// Prepend the tick count.
    FlagTickCount
}

impl Default for LogItems {
    fn default() -> Self {
        LogItems::Default
    }
}

impl From<cef_log_items_t> for LogItems {
    fn from(value: cef_log_items_t) -> Self {
        LogItems::from(&value)
    }
}

impl From<&cef_log_items_t> for LogItems {
    fn from(value: &cef_log_items_t) -> Self {
        match value {
            cef_log_items_t::LOG_ITEMS_DEFAULT => Self::Default,
            cef_log_items_t::LOG_ITEMS_NONE => Self::None,
            cef_log_items_t::LOG_ITEMS_FLAG_PROCESS_ID => Self::FlagProcessId,
            cef_log_items_t::LOG_ITEMS_FLAG_THREAD_ID => Self::FlagThreadId,
            cef_log_items_t::LOG_ITEMS_FLAG_TIME_STAMP => Self::FlagTimeStamp,
            cef_log_items_t::LOG_ITEMS_FLAG_TICK_COUNT => Self::FlagTickCount
        }
    }
}

impl From<LogItems> for cef_log_items_t {
    fn from(value: LogItems) -> Self {
        cef_log_items_t::from(&value)
    }
}

impl From<&LogItems> for cef_log_items_t {
    fn from(value: &LogItems) -> Self {
        match value {
            LogItems::Default => Self::LOG_ITEMS_DEFAULT,
            LogItems::None => Self::LOG_ITEMS_NONE,
            LogItems::FlagProcessId => Self::LOG_ITEMS_FLAG_PROCESS_ID,
            LogItems::FlagThreadId => Self::LOG_ITEMS_FLAG_THREAD_ID,
            LogItems::FlagTimeStamp => Self::LOG_ITEMS_FLAG_TIME_STAMP,
            LogItems::FlagTickCount => Self::LOG_ITEMS_FLAG_TICK_COUNT
        }
    }
}

/// Specifies the zoom commands supported by CefBrowserHost::Zoom.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ZoomCommand {
    /// Zoom out.
    Out,

    /// Reset the zoom level to the default.
    Reset,

    /// Zoom in.
    In
}

impl From<cef_zoom_command_t> for ZoomCommand {
    fn from(value: cef_zoom_command_t) -> Self {
        ZoomCommand::from(&value)
    }
}

impl From<&cef_zoom_command_t> for ZoomCommand {
    fn from(value: &cef_zoom_command_t) -> Self {
        match value {
            cef_zoom_command_t::CEF_ZOOM_COMMAND_OUT => ZoomCommand::Out,
            cef_zoom_command_t::CEF_ZOOM_COMMAND_RESET => ZoomCommand::Reset,
            cef_zoom_command_t::CEF_ZOOM_COMMAND_IN => ZoomCommand::In
        }
    }
}

impl From<ZoomCommand> for cef_zoom_command_t {
    fn from(value: ZoomCommand) -> Self {
        cef_zoom_command_t::from(&value)
    }
}

impl From<&ZoomCommand> for cef_zoom_command_t {
    fn from(value: &ZoomCommand) -> Self {
        match value {
            ZoomCommand::Out => cef_zoom_command_t::CEF_ZOOM_COMMAND_OUT,
            ZoomCommand::Reset => cef_zoom_command_t::CEF_ZOOM_COMMAND_RESET,
            ZoomCommand::In => cef_zoom_command_t::CEF_ZOOM_COMMAND_IN
        }
    }
}

/// Paint element types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PaintElementType {
    /// View.
    View,

    /// Popup.
    Popup
}

impl From<cef_paint_element_type_t> for PaintElementType {
    fn from(value: cef_paint_element_type_t) -> Self {
        PaintElementType::from(&value)
    }
}

impl From<&cef_paint_element_type_t> for PaintElementType {
    fn from(value: &cef_paint_element_type_t) -> Self {
        match value {
            cef_paint_element_type_t::PET_VIEW => PaintElementType::View,
            cef_paint_element_type_t::PET_POPUP => PaintElementType::Popup
        }
    }
}

impl From<PaintElementType> for cef_paint_element_type_t {
    fn from(value: PaintElementType) -> Self {
        cef_paint_element_type_t::from(&value)
    }
}

impl From<&PaintElementType> for cef_paint_element_type_t {
    fn from(value: &PaintElementType) -> Self {
        match value {
            PaintElementType::View => cef_paint_element_type_t::PET_VIEW,
            PaintElementType::Popup => cef_paint_element_type_t::PET_POPUP
        }
    }
}

/// Structure representing a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl From<cef_point_t> for Point {
    fn from(value: cef_point_t) -> Self {
        Point::from(&value)
    }
}

impl From<&cef_point_t> for Point {
    fn from(value: &cef_point_t) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

impl From<Point> for cef_point_t {
    fn from(value: Point) -> Self {
        cef_point_t::from(&value)
    }
}

impl From<&Point> for cef_point_t {
    fn from(value: &Point) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

/// Structure representing a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x:      i32,
    pub y:      i32,
    pub width:  i32,
    pub height: i32
}

impl From<cef_rect_t> for Rect {
    fn from(value: cef_rect_t) -> Self {
        Rect::from(&value)
    }
}

impl From<&cef_rect_t> for Rect {
    fn from(value: &cef_rect_t) -> Self {
        Self {
            x:      value.x,
            y:      value.y,
            width:  value.width,
            height: value.height
        }
    }
}

impl From<Rect> for cef_rect_t {
    fn from(value: Rect) -> Self {
        cef_rect_t::from(&value)
    }
}

impl From<&Rect> for cef_rect_t {
    fn from(value: &Rect) -> Self {
        Self {
            x:      value.x,
            y:      value.y,
            width:  value.width,
            height: value.height
        }
    }
}

/// Structure representing a size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Size {
    pub width:  i32,
    pub height: i32
}

impl From<cef_size_t> for Size {
    fn from(value: cef_size_t) -> Self {
        Size::from(&value)
    }
}

impl From<&cef_size_t> for Size {
    fn from(value: &cef_size_t) -> Self {
        Self {
            width:  value.width,
            height: value.height
        }
    }
}

impl From<Size> for cef_size_t {
    fn from(value: Size) -> Self {
        cef_size_t::from(&value)
    }
}

impl From<&Size> for cef_size_t {
    fn from(value: &Size) -> Self {
        Self {
            width:  value.width,
            height: value.height
        }
    }
}

/// Structure representing insets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Insets {
    pub top:    i32,
    pub left:   i32,
    pub bottom: i32,
    pub right:  i32
}

impl From<cef_insets_t> for Insets {
    fn from(value: cef_insets_t) -> Self {
        Insets::from(&value)
    }
}

impl From<&cef_insets_t> for Insets {
    fn from(value: &cef_insets_t) -> Self {
        Self {
            top:    value.top,
            left:   value.left,
            bottom: value.bottom,
            right:  value.right
        }
    }
}

impl From<Insets> for cef_insets_t {
    fn from(value: Insets) -> Self {
        cef_insets_t::from(&value)
    }
}

impl From<&Insets> for cef_insets_t {
    fn from(value: &Insets) -> Self {
        Self {
            top:    value.top,
            left:   value.left,
            bottom: value.bottom,
            right:  value.right
        }
    }
}

/// Structure representing a range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub from: u32,
    pub to:   u32
}

impl From<cef_range_t> for Range {
    fn from(value: cef_range_t) -> Self {
        Range::from(&value)
    }
}

impl From<&cef_range_t> for Range {
    fn from(value: &cef_range_t) -> Self {
        Self {
            from: value.from,
            to:   value.to
        }
    }
}

impl From<Range> for cef_range_t {
    fn from(value: Range) -> Self {
        cef_range_t::from(&value)
    }
}

impl From<&Range> for cef_range_t {
    fn from(value: &Range) -> Self {
        Self {
            from: value.from,
            to:   value.to
        }
    }
}
