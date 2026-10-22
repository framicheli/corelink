pub enum NodeAPIError {
    Rpc { code: RpcErrorCode, message: String }, // RPC Error answer from node
    Transport(TransportError),                   // Node unreachable
    Decode(String),                              // Error parsing the reply
    Unsupported(String),                         // Backend / Core version not supported
}

/// Core's RPC Error Codes
pub enum RpcErrorCode {
    // JSON-RPC 2.0
    InvalidRequest, // -32600
    MethodNotFound, // -32601
    InvalidParams,  // -32602
    Internal,       // -32603
    Parse,          // -32700

    // General
    Mis,                  // -1
    Type,                 // -3
    InvalidAddressOrKey,  // -5
    OutOfMemory,          // -7
    InvalidParameter,     // -8
    Database,             // -20
    Deserialization,      // -22
    VerifyError,          // -25 broadcast: tx invalid
    VerifyRejected,       // -26 broadcast: policy reject
    VerifyAlreadyInChain, // -27
    InWarmup,             // -28 retry
    MethodDeprecated,     // -32

    // P2P
    ClientNotConnected,        // -9
    ClientInInitialDownload,   // -10
    ClientNodeAlreadyAdded,    // -23
    ClientNodeNotAdded,        // -24
    ClientNodeNotConnected,    // -29
    ClientInvalidIpOrSubnet,   // -30
    ClientP2pDisabled,         // -31
    ClientMempoolDisabled,     // -33
    ClientNodeCapacityReached, // -34

    // Wallet
    WalletError,               // -4
    WalletInsufficientFunds,   // -6
    WalletInvalidLabelName,    // -11
    WalletKeypoolRanOut,       // -12
    WalletUnlockNeeded,        // -13
    WalletPassphraseIncorrect, // -14
    WalletWrongEncState,       // -15
    WalletEncryptionFailed,    // -16
    WalletAlreadyUnlocked,     // -17
    WalletNotFound,            // -18
    WalletNotSpecified,        // -19
    WalletAlreadyLoaded,       // -35
    WalletAlreadyExists,       // -36
    Other(i64),
}

pub enum TransportError {
    Connect(String), // refused, DNS, TLS
    Timeout,
    Auth,      // HTTP 401/403
    Http(u16), // Other non 2xx with a non JSON body
    Other(String),
}

impl TransportError {
    /// For `Connect` and `Timeout`
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Connect(_) | Self::Timeout)
    }
}
