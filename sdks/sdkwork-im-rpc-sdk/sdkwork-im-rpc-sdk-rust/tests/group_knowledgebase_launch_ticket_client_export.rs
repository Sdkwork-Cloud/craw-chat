use sdkwork_im_rpc_sdk_rust::sdkwork::communication::internal::v1::{
    ConsumeGroupKnowledgebaseLaunchTicketRequest,
    group_knowledgebase_launch_ticket_service_client::GroupKnowledgebaseLaunchTicketServiceClient,
};

#[test]
fn group_knowledgebase_ticket_client_is_publicly_exported() {
    let _request = ConsumeGroupKnowledgebaseLaunchTicketRequest::default();
    let _client_type = std::any::TypeId::of::<
        GroupKnowledgebaseLaunchTicketServiceClient<tonic::transport::Channel>,
    >();
}
