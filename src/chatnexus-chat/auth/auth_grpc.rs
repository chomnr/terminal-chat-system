use tonic::{Request, Response, Status};

use crate::{chat::{chatnexus_chat::{auth_server::{Auth}, AuthResponse, AuthRequest}}};

use super::AuthService;

#[tonic::async_trait]
impl Auth for AuthService {
    async fn notify_auth_service(
        &self, 
        request: Request<AuthRequest>
    ) -> Result<Response<AuthResponse>, Status> {
        let data = request.get_ref();
        
        todo!()
    }
}