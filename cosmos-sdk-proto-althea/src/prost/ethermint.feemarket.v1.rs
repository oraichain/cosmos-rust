/// Params defines the EVM module parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// no base fee forces the EIP-1559 base fee to 0 (needed for 0 price calls)
    #[prost(bool, tag = "1")]
    pub no_base_fee: bool,
    /// base fee change denominator bounds the amount the base fee can change
    /// between blocks.
    #[prost(uint32, tag = "2")]
    pub base_fee_change_denominator: u32,
    /// elasticity multiplier bounds the maximum gas limit an EIP-1559 block may
    /// have.
    #[prost(uint32, tag = "3")]
    pub elasticity_multiplier: u32,
    /// height at which the base fee calculation is enabled.
    #[prost(int64, tag = "5")]
    pub enable_height: i64,
    /// base fee for EIP-1559 blocks.
    #[prost(string, tag = "6")]
    pub base_fee: ::prost::alloc::string::String,
    /// min_gas_price defines the minimum gas price value for cosmos and eth transactions
    #[prost(string, tag = "7")]
    pub min_gas_price: ::prost::alloc::string::String,
    /// min gas denominator bounds the minimum gasUsed to be charged
    /// to senders based on GasLimit
    #[prost(string, tag = "8")]
    pub min_gas_multiplier: ::prost::alloc::string::String,
}
/// QueryParamsRequest defines the request type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse defines the response type for querying x/evm parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params define the evm module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryBaseFeeRequest defines the request type for querying the EIP1559 base
/// fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeRequest {}
/// BaseFeeResponse returns the EIP1559 base fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBaseFeeResponse {
    #[prost(string, tag = "1")]
    pub base_fee: ::prost::alloc::string::String,
}
/// QueryBlockGasRequest defines the request type for querying the EIP1559 base
/// fee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasRequest {}
/// QueryBlockGasResponse returns block gas used for a given height.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockGasResponse {
    #[prost(int64, tag = "1")]
    pub gas: i64,
}
/// Generated client implementations.
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Params queries the parameters of x/feemarket module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/ethermint.feemarket.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BaseFee queries the base fee of the parent block of the current block.
        pub async fn base_fee(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBaseFeeRequest>,
        ) -> Result<tonic::Response<super::QueryBaseFeeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ethermint.feemarket.v1.Query/BaseFee");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BlockGas queries the gas used at a given block height
        pub async fn block_gas(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBlockGasRequest>,
        ) -> Result<tonic::Response<super::QueryBlockGasResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/ethermint.feemarket.v1.Query/BlockGas");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// GenesisState defines the feemarket module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// block gas is the amount of gas wanted on the last block before the upgrade.
    /// Zero by default.
    #[prost(uint64, tag = "3")]
    pub block_gas: u64,
}
