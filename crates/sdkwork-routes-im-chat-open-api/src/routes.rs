use axum::Router;
use conversation_runtime::conversation_state::ConversationStateService;
use conversation_runtime::http::AppState;
use std::sync::Arc;

pub fn build_api_router(state: AppState) -> Router {
    build_api_router_with_query_service(
        state,
        conversation_runtime::conversation_state::default_conversation_state_service(),
    )
}

pub fn build_api_router_with_query_service(
    state: AppState,
    query_service: Arc<ConversationStateService>,
) -> Router {
    conversation_runtime::http::build_domain_api_router(state).merge(
        conversation_runtime::conversation_state::build_conversation_query_api_router(
            query_service,
        ),
    )
}
