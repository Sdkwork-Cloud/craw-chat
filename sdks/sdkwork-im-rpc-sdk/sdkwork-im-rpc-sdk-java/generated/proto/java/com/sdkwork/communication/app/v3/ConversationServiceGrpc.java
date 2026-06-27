package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class ConversationServiceGrpc {

  private ConversationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.ConversationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationRequest,
      com.sdkwork.communication.app.v3.CreateConversationResponse> getCreateConversationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateConversation",
      requestType = com.sdkwork.communication.app.v3.CreateConversationRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateConversationResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationRequest,
      com.sdkwork.communication.app.v3.CreateConversationResponse> getCreateConversationMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationRequest, com.sdkwork.communication.app.v3.CreateConversationResponse> getCreateConversationMethod;
    if ((getCreateConversationMethod = ConversationServiceGrpc.getCreateConversationMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getCreateConversationMethod = ConversationServiceGrpc.getCreateConversationMethod) == null) {
          ConversationServiceGrpc.getCreateConversationMethod = getCreateConversationMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateConversationRequest, com.sdkwork.communication.app.v3.CreateConversationResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateConversation"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateConversationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateConversationResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("CreateConversation"))
              .build();
        }
      }
    }
    return getCreateConversationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentDialogRequest,
      com.sdkwork.communication.app.v3.CreateAgentDialogResponse> getCreateAgentDialogMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAgentDialog",
      requestType = com.sdkwork.communication.app.v3.CreateAgentDialogRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateAgentDialogResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentDialogRequest,
      com.sdkwork.communication.app.v3.CreateAgentDialogResponse> getCreateAgentDialogMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentDialogRequest, com.sdkwork.communication.app.v3.CreateAgentDialogResponse> getCreateAgentDialogMethod;
    if ((getCreateAgentDialogMethod = ConversationServiceGrpc.getCreateAgentDialogMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getCreateAgentDialogMethod = ConversationServiceGrpc.getCreateAgentDialogMethod) == null) {
          ConversationServiceGrpc.getCreateAgentDialogMethod = getCreateAgentDialogMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateAgentDialogRequest, com.sdkwork.communication.app.v3.CreateAgentDialogResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAgentDialog"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentDialogRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentDialogResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("CreateAgentDialog"))
              .build();
        }
      }
    }
    return getCreateAgentDialogMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentHandoffRequest,
      com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> getCreateAgentHandoffMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAgentHandoff",
      requestType = com.sdkwork.communication.app.v3.CreateAgentHandoffRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateAgentHandoffResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentHandoffRequest,
      com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> getCreateAgentHandoffMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentHandoffRequest, com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> getCreateAgentHandoffMethod;
    if ((getCreateAgentHandoffMethod = ConversationServiceGrpc.getCreateAgentHandoffMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getCreateAgentHandoffMethod = ConversationServiceGrpc.getCreateAgentHandoffMethod) == null) {
          ConversationServiceGrpc.getCreateAgentHandoffMethod = getCreateAgentHandoffMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateAgentHandoffRequest, com.sdkwork.communication.app.v3.CreateAgentHandoffResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAgentHandoff"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentHandoffRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentHandoffResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("CreateAgentHandoff"))
              .build();
        }
      }
    }
    return getCreateAgentHandoffMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateSystemChannelRequest,
      com.sdkwork.communication.app.v3.CreateSystemChannelResponse> getCreateSystemChannelMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateSystemChannel",
      requestType = com.sdkwork.communication.app.v3.CreateSystemChannelRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateSystemChannelResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateSystemChannelRequest,
      com.sdkwork.communication.app.v3.CreateSystemChannelResponse> getCreateSystemChannelMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateSystemChannelRequest, com.sdkwork.communication.app.v3.CreateSystemChannelResponse> getCreateSystemChannelMethod;
    if ((getCreateSystemChannelMethod = ConversationServiceGrpc.getCreateSystemChannelMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getCreateSystemChannelMethod = ConversationServiceGrpc.getCreateSystemChannelMethod) == null) {
          ConversationServiceGrpc.getCreateSystemChannelMethod = getCreateSystemChannelMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateSystemChannelRequest, com.sdkwork.communication.app.v3.CreateSystemChannelResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateSystemChannel"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateSystemChannelRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateSystemChannelResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("CreateSystemChannel"))
              .build();
        }
      }
    }
    return getCreateSystemChannelMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateThreadRequest,
      com.sdkwork.communication.app.v3.CreateThreadResponse> getCreateThreadMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateThread",
      requestType = com.sdkwork.communication.app.v3.CreateThreadRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateThreadResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateThreadRequest,
      com.sdkwork.communication.app.v3.CreateThreadResponse> getCreateThreadMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateThreadRequest, com.sdkwork.communication.app.v3.CreateThreadResponse> getCreateThreadMethod;
    if ((getCreateThreadMethod = ConversationServiceGrpc.getCreateThreadMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getCreateThreadMethod = ConversationServiceGrpc.getCreateThreadMethod) == null) {
          ConversationServiceGrpc.getCreateThreadMethod = getCreateThreadMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateThreadRequest, com.sdkwork.communication.app.v3.CreateThreadResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateThread"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateThreadRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateThreadResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("CreateThread"))
              .build();
        }
      }
    }
    return getCreateThreadMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.BindDirectChatRequest,
      com.sdkwork.communication.app.v3.BindDirectChatResponse> getBindDirectChatMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "BindDirectChat",
      requestType = com.sdkwork.communication.app.v3.BindDirectChatRequest.class,
      responseType = com.sdkwork.communication.app.v3.BindDirectChatResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.BindDirectChatRequest,
      com.sdkwork.communication.app.v3.BindDirectChatResponse> getBindDirectChatMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.BindDirectChatRequest, com.sdkwork.communication.app.v3.BindDirectChatResponse> getBindDirectChatMethod;
    if ((getBindDirectChatMethod = ConversationServiceGrpc.getBindDirectChatMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getBindDirectChatMethod = ConversationServiceGrpc.getBindDirectChatMethod) == null) {
          ConversationServiceGrpc.getBindDirectChatMethod = getBindDirectChatMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.BindDirectChatRequest, com.sdkwork.communication.app.v3.BindDirectChatResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "BindDirectChat"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.BindDirectChatRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.BindDirectChatResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("BindDirectChat"))
              .build();
        }
      }
    }
    return getBindDirectChatMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationResponse> getRetrieveConversationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveConversation",
      requestType = com.sdkwork.communication.app.v3.RetrieveConversationRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveConversationResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationResponse> getRetrieveConversationMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationRequest, com.sdkwork.communication.app.v3.RetrieveConversationResponse> getRetrieveConversationMethod;
    if ((getRetrieveConversationMethod = ConversationServiceGrpc.getRetrieveConversationMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRetrieveConversationMethod = ConversationServiceGrpc.getRetrieveConversationMethod) == null) {
          ConversationServiceGrpc.getRetrieveConversationMethod = getRetrieveConversationMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveConversationRequest, com.sdkwork.communication.app.v3.RetrieveConversationResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveConversation"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RetrieveConversation"))
              .build();
        }
      }
    }
    return getRetrieveConversationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveInboxRequest,
      com.sdkwork.communication.app.v3.RetrieveInboxResponse> getRetrieveInboxMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveInbox",
      requestType = com.sdkwork.communication.app.v3.RetrieveInboxRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveInboxResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveInboxRequest,
      com.sdkwork.communication.app.v3.RetrieveInboxResponse> getRetrieveInboxMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveInboxRequest, com.sdkwork.communication.app.v3.RetrieveInboxResponse> getRetrieveInboxMethod;
    if ((getRetrieveInboxMethod = ConversationServiceGrpc.getRetrieveInboxMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRetrieveInboxMethod = ConversationServiceGrpc.getRetrieveInboxMethod) == null) {
          ConversationServiceGrpc.getRetrieveInboxMethod = getRetrieveInboxMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveInboxRequest, com.sdkwork.communication.app.v3.RetrieveInboxResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveInbox"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveInboxRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveInboxResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RetrieveInbox"))
              .build();
        }
      }
    }
    return getRetrieveInboxMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMembersRequest,
      com.sdkwork.communication.app.v3.ListConversationMembersResponse> getListConversationMembersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListConversationMembers",
      requestType = com.sdkwork.communication.app.v3.ListConversationMembersRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListConversationMembersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMembersRequest,
      com.sdkwork.communication.app.v3.ListConversationMembersResponse> getListConversationMembersMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMembersRequest, com.sdkwork.communication.app.v3.ListConversationMembersResponse> getListConversationMembersMethod;
    if ((getListConversationMembersMethod = ConversationServiceGrpc.getListConversationMembersMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getListConversationMembersMethod = ConversationServiceGrpc.getListConversationMembersMethod) == null) {
          ConversationServiceGrpc.getListConversationMembersMethod = getListConversationMembersMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListConversationMembersRequest, com.sdkwork.communication.app.v3.ListConversationMembersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListConversationMembers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMembersRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMembersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("ListConversationMembers"))
              .build();
        }
      }
    }
    return getListConversationMembersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AddConversationMemberRequest,
      com.sdkwork.communication.app.v3.AddConversationMemberResponse> getAddConversationMemberMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AddConversationMember",
      requestType = com.sdkwork.communication.app.v3.AddConversationMemberRequest.class,
      responseType = com.sdkwork.communication.app.v3.AddConversationMemberResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AddConversationMemberRequest,
      com.sdkwork.communication.app.v3.AddConversationMemberResponse> getAddConversationMemberMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AddConversationMemberRequest, com.sdkwork.communication.app.v3.AddConversationMemberResponse> getAddConversationMemberMethod;
    if ((getAddConversationMemberMethod = ConversationServiceGrpc.getAddConversationMemberMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getAddConversationMemberMethod = ConversationServiceGrpc.getAddConversationMemberMethod) == null) {
          ConversationServiceGrpc.getAddConversationMemberMethod = getAddConversationMemberMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AddConversationMemberRequest, com.sdkwork.communication.app.v3.AddConversationMemberResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AddConversationMember"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AddConversationMemberRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AddConversationMemberResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("AddConversationMember"))
              .build();
        }
      }
    }
    return getAddConversationMemberMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveConversationMemberRequest,
      com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> getRemoveConversationMemberMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveConversationMember",
      requestType = com.sdkwork.communication.app.v3.RemoveConversationMemberRequest.class,
      responseType = com.sdkwork.communication.app.v3.RemoveConversationMemberResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveConversationMemberRequest,
      com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> getRemoveConversationMemberMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveConversationMemberRequest, com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> getRemoveConversationMemberMethod;
    if ((getRemoveConversationMemberMethod = ConversationServiceGrpc.getRemoveConversationMemberMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRemoveConversationMemberMethod = ConversationServiceGrpc.getRemoveConversationMemberMethod) == null) {
          ConversationServiceGrpc.getRemoveConversationMemberMethod = getRemoveConversationMemberMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RemoveConversationMemberRequest, com.sdkwork.communication.app.v3.RemoveConversationMemberResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveConversationMember"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RemoveConversationMemberRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RemoveConversationMemberResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RemoveConversationMember"))
              .build();
        }
      }
    }
    return getRemoveConversationMemberMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.TransferConversationOwnerRequest,
      com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> getTransferConversationOwnerMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "TransferConversationOwner",
      requestType = com.sdkwork.communication.app.v3.TransferConversationOwnerRequest.class,
      responseType = com.sdkwork.communication.app.v3.TransferConversationOwnerResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.TransferConversationOwnerRequest,
      com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> getTransferConversationOwnerMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.TransferConversationOwnerRequest, com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> getTransferConversationOwnerMethod;
    if ((getTransferConversationOwnerMethod = ConversationServiceGrpc.getTransferConversationOwnerMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getTransferConversationOwnerMethod = ConversationServiceGrpc.getTransferConversationOwnerMethod) == null) {
          ConversationServiceGrpc.getTransferConversationOwnerMethod = getTransferConversationOwnerMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.TransferConversationOwnerRequest, com.sdkwork.communication.app.v3.TransferConversationOwnerResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "TransferConversationOwner"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.TransferConversationOwnerRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.TransferConversationOwnerResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("TransferConversationOwner"))
              .build();
        }
      }
    }
    return getTransferConversationOwnerMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest,
      com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> getChangeConversationMemberRoleMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ChangeConversationMemberRole",
      requestType = com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest.class,
      responseType = com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest,
      com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> getChangeConversationMemberRoleMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest, com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> getChangeConversationMemberRoleMethod;
    if ((getChangeConversationMemberRoleMethod = ConversationServiceGrpc.getChangeConversationMemberRoleMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getChangeConversationMemberRoleMethod = ConversationServiceGrpc.getChangeConversationMemberRoleMethod) == null) {
          ConversationServiceGrpc.getChangeConversationMemberRoleMethod = getChangeConversationMemberRoleMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest, com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ChangeConversationMemberRole"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("ChangeConversationMemberRole"))
              .build();
        }
      }
    }
    return getChangeConversationMemberRoleMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveConversationRequest,
      com.sdkwork.communication.app.v3.LeaveConversationResponse> getLeaveConversationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LeaveConversation",
      requestType = com.sdkwork.communication.app.v3.LeaveConversationRequest.class,
      responseType = com.sdkwork.communication.app.v3.LeaveConversationResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveConversationRequest,
      com.sdkwork.communication.app.v3.LeaveConversationResponse> getLeaveConversationMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveConversationRequest, com.sdkwork.communication.app.v3.LeaveConversationResponse> getLeaveConversationMethod;
    if ((getLeaveConversationMethod = ConversationServiceGrpc.getLeaveConversationMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getLeaveConversationMethod = ConversationServiceGrpc.getLeaveConversationMethod) == null) {
          ConversationServiceGrpc.getLeaveConversationMethod = getLeaveConversationMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.LeaveConversationRequest, com.sdkwork.communication.app.v3.LeaveConversationResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LeaveConversation"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.LeaveConversationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.LeaveConversationResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("LeaveConversation"))
              .build();
        }
      }
    }
    return getLeaveConversationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> getRetrieveConversationPreferencesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveConversationPreferences",
      requestType = com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> getRetrieveConversationPreferencesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest, com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> getRetrieveConversationPreferencesMethod;
    if ((getRetrieveConversationPreferencesMethod = ConversationServiceGrpc.getRetrieveConversationPreferencesMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRetrieveConversationPreferencesMethod = ConversationServiceGrpc.getRetrieveConversationPreferencesMethod) == null) {
          ConversationServiceGrpc.getRetrieveConversationPreferencesMethod = getRetrieveConversationPreferencesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest, com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveConversationPreferences"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RetrieveConversationPreferences"))
              .build();
        }
      }
    }
    return getRetrieveConversationPreferencesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest,
      com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> getUpdateConversationPreferencesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UpdateConversationPreferences",
      requestType = com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest.class,
      responseType = com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest,
      com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> getUpdateConversationPreferencesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest, com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> getUpdateConversationPreferencesMethod;
    if ((getUpdateConversationPreferencesMethod = ConversationServiceGrpc.getUpdateConversationPreferencesMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getUpdateConversationPreferencesMethod = ConversationServiceGrpc.getUpdateConversationPreferencesMethod) == null) {
          ConversationServiceGrpc.getUpdateConversationPreferencesMethod = getUpdateConversationPreferencesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest, com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UpdateConversationPreferences"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("UpdateConversationPreferences"))
              .build();
        }
      }
    }
    return getUpdateConversationPreferencesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> getRetrieveConversationProfileMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveConversationProfile",
      requestType = com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest,
      com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> getRetrieveConversationProfileMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest, com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> getRetrieveConversationProfileMethod;
    if ((getRetrieveConversationProfileMethod = ConversationServiceGrpc.getRetrieveConversationProfileMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRetrieveConversationProfileMethod = ConversationServiceGrpc.getRetrieveConversationProfileMethod) == null) {
          ConversationServiceGrpc.getRetrieveConversationProfileMethod = getRetrieveConversationProfileMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest, com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveConversationProfile"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RetrieveConversationProfile"))
              .build();
        }
      }
    }
    return getRetrieveConversationProfileMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationProfileRequest,
      com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> getUpdateConversationProfileMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UpdateConversationProfile",
      requestType = com.sdkwork.communication.app.v3.UpdateConversationProfileRequest.class,
      responseType = com.sdkwork.communication.app.v3.UpdateConversationProfileResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationProfileRequest,
      com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> getUpdateConversationProfileMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateConversationProfileRequest, com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> getUpdateConversationProfileMethod;
    if ((getUpdateConversationProfileMethod = ConversationServiceGrpc.getUpdateConversationProfileMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getUpdateConversationProfileMethod = ConversationServiceGrpc.getUpdateConversationProfileMethod) == null) {
          ConversationServiceGrpc.getUpdateConversationProfileMethod = getUpdateConversationProfileMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UpdateConversationProfileRequest, com.sdkwork.communication.app.v3.UpdateConversationProfileResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UpdateConversationProfile"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateConversationProfileRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateConversationProfileResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("UpdateConversationProfile"))
              .build();
        }
      }
    }
    return getUpdateConversationProfileMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveReadCursorRequest,
      com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> getRetrieveReadCursorMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveReadCursor",
      requestType = com.sdkwork.communication.app.v3.RetrieveReadCursorRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveReadCursorResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveReadCursorRequest,
      com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> getRetrieveReadCursorMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveReadCursorRequest, com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> getRetrieveReadCursorMethod;
    if ((getRetrieveReadCursorMethod = ConversationServiceGrpc.getRetrieveReadCursorMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getRetrieveReadCursorMethod = ConversationServiceGrpc.getRetrieveReadCursorMethod) == null) {
          ConversationServiceGrpc.getRetrieveReadCursorMethod = getRetrieveReadCursorMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveReadCursorRequest, com.sdkwork.communication.app.v3.RetrieveReadCursorResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveReadCursor"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveReadCursorRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveReadCursorResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("RetrieveReadCursor"))
              .build();
        }
      }
    }
    return getRetrieveReadCursorMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateReadCursorRequest,
      com.sdkwork.communication.app.v3.UpdateReadCursorResponse> getUpdateReadCursorMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UpdateReadCursor",
      requestType = com.sdkwork.communication.app.v3.UpdateReadCursorRequest.class,
      responseType = com.sdkwork.communication.app.v3.UpdateReadCursorResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateReadCursorRequest,
      com.sdkwork.communication.app.v3.UpdateReadCursorResponse> getUpdateReadCursorMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateReadCursorRequest, com.sdkwork.communication.app.v3.UpdateReadCursorResponse> getUpdateReadCursorMethod;
    if ((getUpdateReadCursorMethod = ConversationServiceGrpc.getUpdateReadCursorMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getUpdateReadCursorMethod = ConversationServiceGrpc.getUpdateReadCursorMethod) == null) {
          ConversationServiceGrpc.getUpdateReadCursorMethod = getUpdateReadCursorMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UpdateReadCursorRequest, com.sdkwork.communication.app.v3.UpdateReadCursorResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UpdateReadCursor"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateReadCursorRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateReadCursorResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("UpdateReadCursor"))
              .build();
        }
      }
    }
    return getUpdateReadCursorMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest,
      com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> getListConversationMemberDirectoryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListConversationMemberDirectory",
      requestType = com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest,
      com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> getListConversationMemberDirectoryMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest, com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> getListConversationMemberDirectoryMethod;
    if ((getListConversationMemberDirectoryMethod = ConversationServiceGrpc.getListConversationMemberDirectoryMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getListConversationMemberDirectoryMethod = ConversationServiceGrpc.getListConversationMemberDirectoryMethod) == null) {
          ConversationServiceGrpc.getListConversationMemberDirectoryMethod = getListConversationMemberDirectoryMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest, com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListConversationMemberDirectory"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("ListConversationMemberDirectory"))
              .build();
        }
      }
    }
    return getListConversationMemberDirectoryMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListPinnedMessagesRequest,
      com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> getListPinnedMessagesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListPinnedMessages",
      requestType = com.sdkwork.communication.app.v3.ListPinnedMessagesRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListPinnedMessagesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListPinnedMessagesRequest,
      com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> getListPinnedMessagesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListPinnedMessagesRequest, com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> getListPinnedMessagesMethod;
    if ((getListPinnedMessagesMethod = ConversationServiceGrpc.getListPinnedMessagesMethod) == null) {
      synchronized (ConversationServiceGrpc.class) {
        if ((getListPinnedMessagesMethod = ConversationServiceGrpc.getListPinnedMessagesMethod) == null) {
          ConversationServiceGrpc.getListPinnedMessagesMethod = getListPinnedMessagesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListPinnedMessagesRequest, com.sdkwork.communication.app.v3.ListPinnedMessagesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListPinnedMessages"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListPinnedMessagesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListPinnedMessagesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ConversationServiceMethodDescriptorSupplier("ListPinnedMessages"))
              .build();
        }
      }
    }
    return getListPinnedMessagesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ConversationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ConversationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ConversationServiceStub>() {
        @java.lang.Override
        public ConversationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ConversationServiceStub(channel, callOptions);
        }
      };
    return ConversationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static ConversationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ConversationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ConversationServiceBlockingV2Stub>() {
        @java.lang.Override
        public ConversationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ConversationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return ConversationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ConversationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ConversationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ConversationServiceBlockingStub>() {
        @java.lang.Override
        public ConversationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ConversationServiceBlockingStub(channel, callOptions);
        }
      };
    return ConversationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ConversationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ConversationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ConversationServiceFutureStub>() {
        @java.lang.Override
        public ConversationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ConversationServiceFutureStub(channel, callOptions);
        }
      };
    return ConversationServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createConversation(com.sdkwork.communication.app.v3.CreateConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateConversationMethod(), responseObserver);
    }

    /**
     */
    default void createAgentDialog(com.sdkwork.communication.app.v3.CreateAgentDialogRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentDialogResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAgentDialogMethod(), responseObserver);
    }

    /**
     */
    default void createAgentHandoff(com.sdkwork.communication.app.v3.CreateAgentHandoffRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAgentHandoffMethod(), responseObserver);
    }

    /**
     */
    default void createSystemChannel(com.sdkwork.communication.app.v3.CreateSystemChannelRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateSystemChannelResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateSystemChannelMethod(), responseObserver);
    }

    /**
     */
    default void createThread(com.sdkwork.communication.app.v3.CreateThreadRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateThreadResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateThreadMethod(), responseObserver);
    }

    /**
     */
    default void bindDirectChat(com.sdkwork.communication.app.v3.BindDirectChatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.BindDirectChatResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getBindDirectChatMethod(), responseObserver);
    }

    /**
     */
    default void retrieveConversation(com.sdkwork.communication.app.v3.RetrieveConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveConversationMethod(), responseObserver);
    }

    /**
     */
    default void retrieveInbox(com.sdkwork.communication.app.v3.RetrieveInboxRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveInboxResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveInboxMethod(), responseObserver);
    }

    /**
     */
    default void listConversationMembers(com.sdkwork.communication.app.v3.ListConversationMembersRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMembersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListConversationMembersMethod(), responseObserver);
    }

    /**
     */
    default void addConversationMember(com.sdkwork.communication.app.v3.AddConversationMemberRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AddConversationMemberResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAddConversationMemberMethod(), responseObserver);
    }

    /**
     */
    default void removeConversationMember(com.sdkwork.communication.app.v3.RemoveConversationMemberRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveConversationMemberMethod(), responseObserver);
    }

    /**
     */
    default void transferConversationOwner(com.sdkwork.communication.app.v3.TransferConversationOwnerRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTransferConversationOwnerMethod(), responseObserver);
    }

    /**
     */
    default void changeConversationMemberRole(com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getChangeConversationMemberRoleMethod(), responseObserver);
    }

    /**
     */
    default void leaveConversation(com.sdkwork.communication.app.v3.LeaveConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveConversationResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLeaveConversationMethod(), responseObserver);
    }

    /**
     */
    default void retrieveConversationPreferences(com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveConversationPreferencesMethod(), responseObserver);
    }

    /**
     */
    default void updateConversationPreferences(com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateConversationPreferencesMethod(), responseObserver);
    }

    /**
     */
    default void retrieveConversationProfile(com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveConversationProfileMethod(), responseObserver);
    }

    /**
     */
    default void updateConversationProfile(com.sdkwork.communication.app.v3.UpdateConversationProfileRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateConversationProfileMethod(), responseObserver);
    }

    /**
     */
    default void retrieveReadCursor(com.sdkwork.communication.app.v3.RetrieveReadCursorRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveReadCursorMethod(), responseObserver);
    }

    /**
     */
    default void updateReadCursor(com.sdkwork.communication.app.v3.UpdateReadCursorRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateReadCursorResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateReadCursorMethod(), responseObserver);
    }

    /**
     */
    default void listConversationMemberDirectory(com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListConversationMemberDirectoryMethod(), responseObserver);
    }

    /**
     */
    default void listPinnedMessages(com.sdkwork.communication.app.v3.ListPinnedMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListPinnedMessagesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service ConversationService.
   */
  public static abstract class ConversationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return ConversationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service ConversationService.
   */
  public static final class ConversationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<ConversationServiceStub> {
    private ConversationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ConversationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ConversationServiceStub(channel, callOptions);
    }

    /**
     */
    public void createConversation(com.sdkwork.communication.app.v3.CreateConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateConversationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAgentDialog(com.sdkwork.communication.app.v3.CreateAgentDialogRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentDialogResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAgentDialogMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAgentHandoff(com.sdkwork.communication.app.v3.CreateAgentHandoffRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAgentHandoffMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createSystemChannel(com.sdkwork.communication.app.v3.CreateSystemChannelRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateSystemChannelResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateSystemChannelMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createThread(com.sdkwork.communication.app.v3.CreateThreadRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateThreadResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateThreadMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void bindDirectChat(com.sdkwork.communication.app.v3.BindDirectChatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.BindDirectChatResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getBindDirectChatMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveConversation(com.sdkwork.communication.app.v3.RetrieveConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveConversationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveInbox(com.sdkwork.communication.app.v3.RetrieveInboxRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveInboxResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveInboxMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listConversationMembers(com.sdkwork.communication.app.v3.ListConversationMembersRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMembersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListConversationMembersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void addConversationMember(com.sdkwork.communication.app.v3.AddConversationMemberRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AddConversationMemberResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAddConversationMemberMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeConversationMember(com.sdkwork.communication.app.v3.RemoveConversationMemberRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveConversationMemberMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void transferConversationOwner(com.sdkwork.communication.app.v3.TransferConversationOwnerRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTransferConversationOwnerMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void changeConversationMemberRole(com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getChangeConversationMemberRoleMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void leaveConversation(com.sdkwork.communication.app.v3.LeaveConversationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveConversationResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLeaveConversationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveConversationPreferences(com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveConversationPreferencesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void updateConversationPreferences(com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateConversationPreferencesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveConversationProfile(com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveConversationProfileMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void updateConversationProfile(com.sdkwork.communication.app.v3.UpdateConversationProfileRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateConversationProfileMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveReadCursor(com.sdkwork.communication.app.v3.RetrieveReadCursorRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveReadCursorMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void updateReadCursor(com.sdkwork.communication.app.v3.UpdateReadCursorRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateReadCursorResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateReadCursorMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listConversationMemberDirectory(com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListConversationMemberDirectoryMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listPinnedMessages(com.sdkwork.communication.app.v3.ListPinnedMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListPinnedMessagesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service ConversationService.
   */
  public static final class ConversationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<ConversationServiceBlockingV2Stub> {
    private ConversationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ConversationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ConversationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateConversationResponse createConversation(com.sdkwork.communication.app.v3.CreateConversationRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentDialogResponse createAgentDialog(com.sdkwork.communication.app.v3.CreateAgentDialogRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAgentDialogMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentHandoffResponse createAgentHandoff(com.sdkwork.communication.app.v3.CreateAgentHandoffRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAgentHandoffMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateSystemChannelResponse createSystemChannel(com.sdkwork.communication.app.v3.CreateSystemChannelRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateSystemChannelMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateThreadResponse createThread(com.sdkwork.communication.app.v3.CreateThreadRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateThreadMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.BindDirectChatResponse bindDirectChat(com.sdkwork.communication.app.v3.BindDirectChatRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getBindDirectChatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationResponse retrieveConversation(com.sdkwork.communication.app.v3.RetrieveConversationRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveInboxResponse retrieveInbox(com.sdkwork.communication.app.v3.RetrieveInboxRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveInboxMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMembersResponse listConversationMembers(com.sdkwork.communication.app.v3.ListConversationMembersRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListConversationMembersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AddConversationMemberResponse addConversationMember(com.sdkwork.communication.app.v3.AddConversationMemberRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAddConversationMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RemoveConversationMemberResponse removeConversationMember(com.sdkwork.communication.app.v3.RemoveConversationMemberRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRemoveConversationMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.TransferConversationOwnerResponse transferConversationOwner(com.sdkwork.communication.app.v3.TransferConversationOwnerRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getTransferConversationOwnerMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse changeConversationMemberRole(com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getChangeConversationMemberRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.LeaveConversationResponse leaveConversation(com.sdkwork.communication.app.v3.LeaveConversationRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLeaveConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse retrieveConversationPreferences(com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveConversationPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse updateConversationPreferences(com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateConversationPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse retrieveConversationProfile(com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveConversationProfileMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateConversationProfileResponse updateConversationProfile(com.sdkwork.communication.app.v3.UpdateConversationProfileRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateConversationProfileMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveReadCursorResponse retrieveReadCursor(com.sdkwork.communication.app.v3.RetrieveReadCursorRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveReadCursorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateReadCursorResponse updateReadCursor(com.sdkwork.communication.app.v3.UpdateReadCursorRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateReadCursorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse listConversationMemberDirectory(com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListConversationMemberDirectoryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListPinnedMessagesResponse listPinnedMessages(com.sdkwork.communication.app.v3.ListPinnedMessagesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListPinnedMessagesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service ConversationService.
   */
  public static final class ConversationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<ConversationServiceBlockingStub> {
    private ConversationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ConversationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ConversationServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateConversationResponse createConversation(com.sdkwork.communication.app.v3.CreateConversationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentDialogResponse createAgentDialog(com.sdkwork.communication.app.v3.CreateAgentDialogRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAgentDialogMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentHandoffResponse createAgentHandoff(com.sdkwork.communication.app.v3.CreateAgentHandoffRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAgentHandoffMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateSystemChannelResponse createSystemChannel(com.sdkwork.communication.app.v3.CreateSystemChannelRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateSystemChannelMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateThreadResponse createThread(com.sdkwork.communication.app.v3.CreateThreadRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateThreadMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.BindDirectChatResponse bindDirectChat(com.sdkwork.communication.app.v3.BindDirectChatRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getBindDirectChatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationResponse retrieveConversation(com.sdkwork.communication.app.v3.RetrieveConversationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveInboxResponse retrieveInbox(com.sdkwork.communication.app.v3.RetrieveInboxRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveInboxMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMembersResponse listConversationMembers(com.sdkwork.communication.app.v3.ListConversationMembersRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListConversationMembersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AddConversationMemberResponse addConversationMember(com.sdkwork.communication.app.v3.AddConversationMemberRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAddConversationMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RemoveConversationMemberResponse removeConversationMember(com.sdkwork.communication.app.v3.RemoveConversationMemberRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveConversationMemberMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.TransferConversationOwnerResponse transferConversationOwner(com.sdkwork.communication.app.v3.TransferConversationOwnerRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTransferConversationOwnerMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse changeConversationMemberRole(com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getChangeConversationMemberRoleMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.LeaveConversationResponse leaveConversation(com.sdkwork.communication.app.v3.LeaveConversationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLeaveConversationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse retrieveConversationPreferences(com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveConversationPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse updateConversationPreferences(com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateConversationPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse retrieveConversationProfile(com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveConversationProfileMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateConversationProfileResponse updateConversationProfile(com.sdkwork.communication.app.v3.UpdateConversationProfileRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateConversationProfileMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveReadCursorResponse retrieveReadCursor(com.sdkwork.communication.app.v3.RetrieveReadCursorRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveReadCursorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateReadCursorResponse updateReadCursor(com.sdkwork.communication.app.v3.UpdateReadCursorRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateReadCursorMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse listConversationMemberDirectory(com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListConversationMemberDirectoryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListPinnedMessagesResponse listPinnedMessages(com.sdkwork.communication.app.v3.ListPinnedMessagesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListPinnedMessagesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service ConversationService.
   */
  public static final class ConversationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<ConversationServiceFutureStub> {
    private ConversationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ConversationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ConversationServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateConversationResponse> createConversation(
        com.sdkwork.communication.app.v3.CreateConversationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateConversationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateAgentDialogResponse> createAgentDialog(
        com.sdkwork.communication.app.v3.CreateAgentDialogRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAgentDialogMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateAgentHandoffResponse> createAgentHandoff(
        com.sdkwork.communication.app.v3.CreateAgentHandoffRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAgentHandoffMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateSystemChannelResponse> createSystemChannel(
        com.sdkwork.communication.app.v3.CreateSystemChannelRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateSystemChannelMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateThreadResponse> createThread(
        com.sdkwork.communication.app.v3.CreateThreadRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateThreadMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.BindDirectChatResponse> bindDirectChat(
        com.sdkwork.communication.app.v3.BindDirectChatRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getBindDirectChatMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveConversationResponse> retrieveConversation(
        com.sdkwork.communication.app.v3.RetrieveConversationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveConversationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveInboxResponse> retrieveInbox(
        com.sdkwork.communication.app.v3.RetrieveInboxRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveInboxMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListConversationMembersResponse> listConversationMembers(
        com.sdkwork.communication.app.v3.ListConversationMembersRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListConversationMembersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AddConversationMemberResponse> addConversationMember(
        com.sdkwork.communication.app.v3.AddConversationMemberRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAddConversationMemberMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RemoveConversationMemberResponse> removeConversationMember(
        com.sdkwork.communication.app.v3.RemoveConversationMemberRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveConversationMemberMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.TransferConversationOwnerResponse> transferConversationOwner(
        com.sdkwork.communication.app.v3.TransferConversationOwnerRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTransferConversationOwnerMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse> changeConversationMemberRole(
        com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getChangeConversationMemberRoleMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.LeaveConversationResponse> leaveConversation(
        com.sdkwork.communication.app.v3.LeaveConversationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLeaveConversationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse> retrieveConversationPreferences(
        com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveConversationPreferencesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse> updateConversationPreferences(
        com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateConversationPreferencesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse> retrieveConversationProfile(
        com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveConversationProfileMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UpdateConversationProfileResponse> updateConversationProfile(
        com.sdkwork.communication.app.v3.UpdateConversationProfileRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateConversationProfileMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveReadCursorResponse> retrieveReadCursor(
        com.sdkwork.communication.app.v3.RetrieveReadCursorRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveReadCursorMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UpdateReadCursorResponse> updateReadCursor(
        com.sdkwork.communication.app.v3.UpdateReadCursorRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateReadCursorMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse> listConversationMemberDirectory(
        com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListConversationMemberDirectoryMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListPinnedMessagesResponse> listPinnedMessages(
        com.sdkwork.communication.app.v3.ListPinnedMessagesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListPinnedMessagesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_CONVERSATION = 0;
  private static final int METHODID_CREATE_AGENT_DIALOG = 1;
  private static final int METHODID_CREATE_AGENT_HANDOFF = 2;
  private static final int METHODID_CREATE_SYSTEM_CHANNEL = 3;
  private static final int METHODID_CREATE_THREAD = 4;
  private static final int METHODID_BIND_DIRECT_CHAT = 5;
  private static final int METHODID_RETRIEVE_CONVERSATION = 6;
  private static final int METHODID_RETRIEVE_INBOX = 7;
  private static final int METHODID_LIST_CONVERSATION_MEMBERS = 8;
  private static final int METHODID_ADD_CONVERSATION_MEMBER = 9;
  private static final int METHODID_REMOVE_CONVERSATION_MEMBER = 10;
  private static final int METHODID_TRANSFER_CONVERSATION_OWNER = 11;
  private static final int METHODID_CHANGE_CONVERSATION_MEMBER_ROLE = 12;
  private static final int METHODID_LEAVE_CONVERSATION = 13;
  private static final int METHODID_RETRIEVE_CONVERSATION_PREFERENCES = 14;
  private static final int METHODID_UPDATE_CONVERSATION_PREFERENCES = 15;
  private static final int METHODID_RETRIEVE_CONVERSATION_PROFILE = 16;
  private static final int METHODID_UPDATE_CONVERSATION_PROFILE = 17;
  private static final int METHODID_RETRIEVE_READ_CURSOR = 18;
  private static final int METHODID_UPDATE_READ_CURSOR = 19;
  private static final int METHODID_LIST_CONVERSATION_MEMBER_DIRECTORY = 20;
  private static final int METHODID_LIST_PINNED_MESSAGES = 21;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_CREATE_CONVERSATION:
          serviceImpl.createConversation((com.sdkwork.communication.app.v3.CreateConversationRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationResponse>) responseObserver);
          break;
        case METHODID_CREATE_AGENT_DIALOG:
          serviceImpl.createAgentDialog((com.sdkwork.communication.app.v3.CreateAgentDialogRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentDialogResponse>) responseObserver);
          break;
        case METHODID_CREATE_AGENT_HANDOFF:
          serviceImpl.createAgentHandoff((com.sdkwork.communication.app.v3.CreateAgentHandoffRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentHandoffResponse>) responseObserver);
          break;
        case METHODID_CREATE_SYSTEM_CHANNEL:
          serviceImpl.createSystemChannel((com.sdkwork.communication.app.v3.CreateSystemChannelRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateSystemChannelResponse>) responseObserver);
          break;
        case METHODID_CREATE_THREAD:
          serviceImpl.createThread((com.sdkwork.communication.app.v3.CreateThreadRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateThreadResponse>) responseObserver);
          break;
        case METHODID_BIND_DIRECT_CHAT:
          serviceImpl.bindDirectChat((com.sdkwork.communication.app.v3.BindDirectChatRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.BindDirectChatResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CONVERSATION:
          serviceImpl.retrieveConversation((com.sdkwork.communication.app.v3.RetrieveConversationRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_INBOX:
          serviceImpl.retrieveInbox((com.sdkwork.communication.app.v3.RetrieveInboxRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveInboxResponse>) responseObserver);
          break;
        case METHODID_LIST_CONVERSATION_MEMBERS:
          serviceImpl.listConversationMembers((com.sdkwork.communication.app.v3.ListConversationMembersRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMembersResponse>) responseObserver);
          break;
        case METHODID_ADD_CONVERSATION_MEMBER:
          serviceImpl.addConversationMember((com.sdkwork.communication.app.v3.AddConversationMemberRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AddConversationMemberResponse>) responseObserver);
          break;
        case METHODID_REMOVE_CONVERSATION_MEMBER:
          serviceImpl.removeConversationMember((com.sdkwork.communication.app.v3.RemoveConversationMemberRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveConversationMemberResponse>) responseObserver);
          break;
        case METHODID_TRANSFER_CONVERSATION_OWNER:
          serviceImpl.transferConversationOwner((com.sdkwork.communication.app.v3.TransferConversationOwnerRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.TransferConversationOwnerResponse>) responseObserver);
          break;
        case METHODID_CHANGE_CONVERSATION_MEMBER_ROLE:
          serviceImpl.changeConversationMemberRole((com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse>) responseObserver);
          break;
        case METHODID_LEAVE_CONVERSATION:
          serviceImpl.leaveConversation((com.sdkwork.communication.app.v3.LeaveConversationRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveConversationResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CONVERSATION_PREFERENCES:
          serviceImpl.retrieveConversationPreferences((com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse>) responseObserver);
          break;
        case METHODID_UPDATE_CONVERSATION_PREFERENCES:
          serviceImpl.updateConversationPreferences((com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CONVERSATION_PROFILE:
          serviceImpl.retrieveConversationProfile((com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse>) responseObserver);
          break;
        case METHODID_UPDATE_CONVERSATION_PROFILE:
          serviceImpl.updateConversationProfile((com.sdkwork.communication.app.v3.UpdateConversationProfileRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateConversationProfileResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_READ_CURSOR:
          serviceImpl.retrieveReadCursor((com.sdkwork.communication.app.v3.RetrieveReadCursorRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveReadCursorResponse>) responseObserver);
          break;
        case METHODID_UPDATE_READ_CURSOR:
          serviceImpl.updateReadCursor((com.sdkwork.communication.app.v3.UpdateReadCursorRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateReadCursorResponse>) responseObserver);
          break;
        case METHODID_LIST_CONVERSATION_MEMBER_DIRECTORY:
          serviceImpl.listConversationMemberDirectory((com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse>) responseObserver);
          break;
        case METHODID_LIST_PINNED_MESSAGES:
          serviceImpl.listPinnedMessages((com.sdkwork.communication.app.v3.ListPinnedMessagesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListPinnedMessagesResponse>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getCreateConversationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateConversationRequest,
              com.sdkwork.communication.app.v3.CreateConversationResponse>(
                service, METHODID_CREATE_CONVERSATION)))
        .addMethod(
          getCreateAgentDialogMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateAgentDialogRequest,
              com.sdkwork.communication.app.v3.CreateAgentDialogResponse>(
                service, METHODID_CREATE_AGENT_DIALOG)))
        .addMethod(
          getCreateAgentHandoffMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateAgentHandoffRequest,
              com.sdkwork.communication.app.v3.CreateAgentHandoffResponse>(
                service, METHODID_CREATE_AGENT_HANDOFF)))
        .addMethod(
          getCreateSystemChannelMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateSystemChannelRequest,
              com.sdkwork.communication.app.v3.CreateSystemChannelResponse>(
                service, METHODID_CREATE_SYSTEM_CHANNEL)))
        .addMethod(
          getCreateThreadMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateThreadRequest,
              com.sdkwork.communication.app.v3.CreateThreadResponse>(
                service, METHODID_CREATE_THREAD)))
        .addMethod(
          getBindDirectChatMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.BindDirectChatRequest,
              com.sdkwork.communication.app.v3.BindDirectChatResponse>(
                service, METHODID_BIND_DIRECT_CHAT)))
        .addMethod(
          getRetrieveConversationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveConversationRequest,
              com.sdkwork.communication.app.v3.RetrieveConversationResponse>(
                service, METHODID_RETRIEVE_CONVERSATION)))
        .addMethod(
          getRetrieveInboxMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveInboxRequest,
              com.sdkwork.communication.app.v3.RetrieveInboxResponse>(
                service, METHODID_RETRIEVE_INBOX)))
        .addMethod(
          getListConversationMembersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListConversationMembersRequest,
              com.sdkwork.communication.app.v3.ListConversationMembersResponse>(
                service, METHODID_LIST_CONVERSATION_MEMBERS)))
        .addMethod(
          getAddConversationMemberMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AddConversationMemberRequest,
              com.sdkwork.communication.app.v3.AddConversationMemberResponse>(
                service, METHODID_ADD_CONVERSATION_MEMBER)))
        .addMethod(
          getRemoveConversationMemberMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RemoveConversationMemberRequest,
              com.sdkwork.communication.app.v3.RemoveConversationMemberResponse>(
                service, METHODID_REMOVE_CONVERSATION_MEMBER)))
        .addMethod(
          getTransferConversationOwnerMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.TransferConversationOwnerRequest,
              com.sdkwork.communication.app.v3.TransferConversationOwnerResponse>(
                service, METHODID_TRANSFER_CONVERSATION_OWNER)))
        .addMethod(
          getChangeConversationMemberRoleMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ChangeConversationMemberRoleRequest,
              com.sdkwork.communication.app.v3.ChangeConversationMemberRoleResponse>(
                service, METHODID_CHANGE_CONVERSATION_MEMBER_ROLE)))
        .addMethod(
          getLeaveConversationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.LeaveConversationRequest,
              com.sdkwork.communication.app.v3.LeaveConversationResponse>(
                service, METHODID_LEAVE_CONVERSATION)))
        .addMethod(
          getRetrieveConversationPreferencesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveConversationPreferencesRequest,
              com.sdkwork.communication.app.v3.RetrieveConversationPreferencesResponse>(
                service, METHODID_RETRIEVE_CONVERSATION_PREFERENCES)))
        .addMethod(
          getUpdateConversationPreferencesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UpdateConversationPreferencesRequest,
              com.sdkwork.communication.app.v3.UpdateConversationPreferencesResponse>(
                service, METHODID_UPDATE_CONVERSATION_PREFERENCES)))
        .addMethod(
          getRetrieveConversationProfileMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveConversationProfileRequest,
              com.sdkwork.communication.app.v3.RetrieveConversationProfileResponse>(
                service, METHODID_RETRIEVE_CONVERSATION_PROFILE)))
        .addMethod(
          getUpdateConversationProfileMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UpdateConversationProfileRequest,
              com.sdkwork.communication.app.v3.UpdateConversationProfileResponse>(
                service, METHODID_UPDATE_CONVERSATION_PROFILE)))
        .addMethod(
          getRetrieveReadCursorMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveReadCursorRequest,
              com.sdkwork.communication.app.v3.RetrieveReadCursorResponse>(
                service, METHODID_RETRIEVE_READ_CURSOR)))
        .addMethod(
          getUpdateReadCursorMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UpdateReadCursorRequest,
              com.sdkwork.communication.app.v3.UpdateReadCursorResponse>(
                service, METHODID_UPDATE_READ_CURSOR)))
        .addMethod(
          getListConversationMemberDirectoryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListConversationMemberDirectoryRequest,
              com.sdkwork.communication.app.v3.ListConversationMemberDirectoryResponse>(
                service, METHODID_LIST_CONVERSATION_MEMBER_DIRECTORY)))
        .addMethod(
          getListPinnedMessagesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListPinnedMessagesRequest,
              com.sdkwork.communication.app.v3.ListPinnedMessagesResponse>(
                service, METHODID_LIST_PINNED_MESSAGES)))
        .build();
  }

  private static abstract class ConversationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ConversationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.ConversationServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("ConversationService");
    }
  }

  private static final class ConversationServiceFileDescriptorSupplier
      extends ConversationServiceBaseDescriptorSupplier {
    ConversationServiceFileDescriptorSupplier() {}
  }

  private static final class ConversationServiceMethodDescriptorSupplier
      extends ConversationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    ConversationServiceMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (ConversationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ConversationServiceFileDescriptorSupplier())
              .addMethod(getCreateConversationMethod())
              .addMethod(getCreateAgentDialogMethod())
              .addMethod(getCreateAgentHandoffMethod())
              .addMethod(getCreateSystemChannelMethod())
              .addMethod(getCreateThreadMethod())
              .addMethod(getBindDirectChatMethod())
              .addMethod(getRetrieveConversationMethod())
              .addMethod(getRetrieveInboxMethod())
              .addMethod(getListConversationMembersMethod())
              .addMethod(getAddConversationMemberMethod())
              .addMethod(getRemoveConversationMemberMethod())
              .addMethod(getTransferConversationOwnerMethod())
              .addMethod(getChangeConversationMemberRoleMethod())
              .addMethod(getLeaveConversationMethod())
              .addMethod(getRetrieveConversationPreferencesMethod())
              .addMethod(getUpdateConversationPreferencesMethod())
              .addMethod(getRetrieveConversationProfileMethod())
              .addMethod(getUpdateConversationProfileMethod())
              .addMethod(getRetrieveReadCursorMethod())
              .addMethod(getUpdateReadCursorMethod())
              .addMethod(getListConversationMemberDirectoryMethod())
              .addMethod(getListPinnedMessagesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
