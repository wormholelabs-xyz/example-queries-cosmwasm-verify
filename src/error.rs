use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExampleQueriesError {
    /// The signature array is the wrong length
    #[error("InvalidSigLen")]
    InvalidSigLen,

    /// Not enough signers on the VAA
    #[error("NoQuorum")]
    NoQuorum,

    /// Overran the signature array
    #[error("InvalidSigLen2")]
    InvalidSigLen2,

    /// Wrong guardian index order, order must be ascending
    #[error("WrongGuardianIndexOrder")]
    WrongGuardianIndexOrder,

    /// Some problem with signature decoding from bytes
    #[error("CannotDecodeSignature")]
    CannotDecodeSignature,

    /// Recovered pubkey from signature does not match guardian address
    #[error("GuardianSignatureError")]
    GuardianSignatureError,

    /// Some problem with public key recovery from the signature
    #[error("CannotRecoverKey")]
    CannotRecoverKey,
    
    /// More signatures than active guardians found
    #[error("TooManySignatures")]
    TooManySignatures,
        
    /// Failed to deserialize query response
    #[error("FailedToParseResponse")]
    FailedToParseResponse,
            
    /// Unexpected number of requests in query response
    #[error("UnexpectedNumberOfRequests")]
    UnexpectedNumberOfRequests,
                
    /// Unsupported request type in query response
    #[error("UnsupportedRequestType")]
    UnsupportedRequestType,     
                
    /// Unexpected number of call data in query response
    #[error("UnexpectedNumberOfCallData")]
    UnexpectedNumberOfCallData, 
                
    /// Unexpected call data length in query response
    #[error("UnexpectedCallDataLength")]
    UnexpectedCallDataLength,     
                
    /// Unexpected call type in query response
    #[error("UnexpectedCallType")]
    UnexpectedCallType,       
                
    /// Unexpected number of responses in query response
    #[error("UnexpectedNumberOfResponses")]
    UnexpectedNumberOfResponses,
            
    /// Unexpected number of results in query response
    #[error("UnexpectedNumberOfResults")]
    UnexpectedNumberOfResults,
                        
    /// Result in query response is the wrong length
    #[error("UnexpectedResultLength")]
    UnexpectedResultLength,  
                        
    /// Failed to parse the total supply.
    #[error("InvalidTotalSupply")]
    InvalidTotalSupply,     
                
    /// Unsupported response type in query response
    #[error("UnsupportedResponseType")]
    UnsupportedResponseType, 
}

impl ExampleQueriesError {
    pub fn std(&self) -> StdError {
        StdError::GenericErr {
            msg: format!("{self}"),
        }
    }

    pub fn std_err<T>(&self) -> Result<T, StdError> {
        Err(self.std())
    }
}
