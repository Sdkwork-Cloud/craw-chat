use crate::service_binding::RpcServiceBinding;

pub const RPC_SDK_FAMILY: &str = "sdkwork-im-rpc-sdk";

pub const RPC_SERVICE_BINDINGS: &[RpcServiceBinding] = &[
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.PresenceService",
        package: "sdkwork.communication.app.v3",
        service: "PresenceService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.RealtimeService",
        package: "sdkwork.communication.app.v3",
        service: "RealtimeService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.ConversationService",
        package: "sdkwork.communication.app.v3",
        service: "ConversationService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.ContactService",
        package: "sdkwork.communication.app.v3",
        service: "ContactService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.MessageService",
        package: "sdkwork.communication.app.v3",
        service: "MessageService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.RoomService",
        package: "sdkwork.communication.app.v3",
        service: "RoomService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.SocialService",
        package: "sdkwork.communication.app.v3",
        service: "SocialService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.StreamService",
        package: "sdkwork.communication.app.v3",
        service: "StreamService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.CallService",
        package: "sdkwork.communication.app.v3",
        service: "CallService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.NotificationService",
        package: "sdkwork.communication.app.v3",
        service: "NotificationService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.app.v3.AutomationService",
        package: "sdkwork.communication.app.v3",
        service: "AutomationService",
        surface: "app",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.CommunicationOpsService",
        package: "sdkwork.communication.backend.v3",
        service: "CommunicationOpsService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.RealtimeNodeAdminService",
        package: "sdkwork.communication.backend.v3",
        service: "RealtimeNodeAdminService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.CommunicationControlService",
        package: "sdkwork.communication.backend.v3",
        service: "CommunicationControlService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.SocialAdminService",
        package: "sdkwork.communication.backend.v3",
        service: "SocialAdminService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.SocialRuntimeAdminService",
        package: "sdkwork.communication.backend.v3",
        service: "SocialRuntimeAdminService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.backend.v3.AuditAdminService",
        package: "sdkwork.communication.backend.v3",
        service: "AuditAdminService",
        surface: "backend",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.internal.v1.RuntimeTopologyService",
        package: "sdkwork.communication.internal.v1",
        service: "RuntimeTopologyService",
        surface: "internal",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.internal.v1.RouteLeaseService",
        package: "sdkwork.communication.internal.v1",
        service: "RouteLeaseService",
        surface: "internal",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.internal.v1.DomainEventRelayService",
        package: "sdkwork.communication.internal.v1",
        service: "DomainEventRelayService",
        surface: "internal",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.internal.v1.RoomOrchestrationService",
        package: "sdkwork.communication.internal.v1",
        service: "RoomOrchestrationService",
        surface: "internal",
    },
    RpcServiceBinding {
        service_key: "sdkwork.communication.internal.v1.MessageDispatchService",
        package: "sdkwork.communication.internal.v1",
        service: "MessageDispatchService",
        surface: "internal",
    },
];
