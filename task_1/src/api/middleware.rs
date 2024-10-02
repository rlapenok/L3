use axum::{ extract::{Request, State}, middleware::Next, response::Response};

use crate::{domain::social_network_service::SocialNetworkService, errors::ServerErrors, infrastructe::server_state::ServerState};



pub async fn authorization(State(state):State<ServerState>,mut request:Request,next:Next)->Result<Response,ServerErrors>{
   let token=request.headers().get("Authorization").and_then(|header|{
    header.to_str().ok()
   }).ok_or({
            ServerErrors::MissingAuthorizationHeader
   })?.strip_prefix("Bearer ").ok_or(ServerErrors::MissingBearer)?;
    let token=state.check_token(token)?;
    request.extensions_mut().insert(token.sub);
    let resp=next.run(request).await;
    Ok(resp)
}