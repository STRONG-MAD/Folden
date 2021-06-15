use super::Server;
use generated_types::handler_service_server::HandlerService;
use super::endpoints::trace_handler_stream::TraceHandlerStream;
use super::endpoints::handler_service_endpoint::ServiceEndpoint;
use super::endpoints::{get_directory_status_endpoint, modify_endpoint, register_endpoint, start_handler_endpoint, stop_handler_endpoint, trace_endpoint};

#[tonic::async_trait]
impl HandlerService for Server {
    type TraceHandlerStream = TraceHandlerStream;

    #[tracing::instrument]
    async fn register_to_directory(
        &self,
        request: register_endpoint::Request,
    ) -> Result<register_endpoint::Response, tonic::Status> {
        tracing::info!("Registering handler to directory");
        let mut mapping = self.mapping.write().await;
        let endpoint = register_endpoint::RegisterEndpoint {
            request,
            mapping,
            server: self,
        };
        endpoint.execute()
    }

    #[tracing::instrument]
    async fn get_directory_status(
        &self,
        request: get_directory_status_endpoint::Request,
    ) -> Result<get_directory_status_endpoint::Response, tonic::Status> {
        tracing::info!("Getting directory status");
        let mapping = self.mapping.read().await;
        let endpoint = get_directory_status_endpoint::GetDirectoryStatusEndpoint {
            request,
            mapping,
        };
        endpoint.execute()
    }

    #[tracing::instrument]
    async fn start_handler(
        &self,
        request: start_handler_endpoint::Request,
    ) -> Result<start_handler_endpoint::Response, tonic::Status> {
        tracing::info!("Starting handler");
        let mut mapping = self.mapping.write().await;
        let endpoint = start_handler_endpoint::StarthandlerEndpoint {
            request,
            mapping,
            server: self,
        };
        endpoint.execute()
    }

    #[tracing::instrument]
    async fn stop_handler(
        &self,
        request: stop_handler_endpoint::Request,
    ) -> Result<stop_handler_endpoint::Response, tonic::Status> {
        tracing::info!("Stopping handler");
        let mut mapping = self.mapping.write().await;
        let endpoint = stop_handler_endpoint::StophandlerEndpoint {
            request,
            mapping,
            server: self,
        };
        endpoint.execute()
    }

    #[tracing::instrument]
    async fn modify_handler(
        &self,
        request: modify_endpoint::Request,
    ) -> Result<modify_endpoint::Response, tonic::Status> {
        tracing::info!("Modifying handler");
        let mut mapping = self.mapping.write().await;
        let endpoint = modify_endpoint::ModifyEndpoint {
            request,
            mapping,
            server: self,
        };
        endpoint.execute()
    }

    async fn trace_handler(
        &self,
        request: trace_endpoint::Request,
    ) -> Result<trace_endpoint::Response, tonic::Status> {
        tracing::info!("Tracing directory handler");
        let mapping = self.mapping.read().await;
        let endpoint = trace_endpoint::TraceEndpoint {
            request,
            mapping,
            server: self,
        };
        endpoint.execute()
    }
}
