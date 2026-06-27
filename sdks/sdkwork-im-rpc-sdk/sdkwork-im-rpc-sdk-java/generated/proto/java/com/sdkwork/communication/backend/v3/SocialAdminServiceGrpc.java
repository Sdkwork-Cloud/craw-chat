package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SocialAdminServiceGrpc {

  private SocialAdminServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.SocialAdminService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest,
      com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> getCreateDirectChatBindingMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateDirectChatBinding",
      requestType = com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest,
      com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> getCreateDirectChatBindingMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest, com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> getCreateDirectChatBindingMethod;
    if ((getCreateDirectChatBindingMethod = SocialAdminServiceGrpc.getCreateDirectChatBindingMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateDirectChatBindingMethod = SocialAdminServiceGrpc.getCreateDirectChatBindingMethod) == null) {
          SocialAdminServiceGrpc.getCreateDirectChatBindingMethod = getCreateDirectChatBindingMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest, com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateDirectChatBinding"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateDirectChatBinding"))
              .build();
        }
      }
    }
    return getCreateDirectChatBindingMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest,
      com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> getRetrieveDirectChatMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveDirectChat",
      requestType = com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest,
      com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> getRetrieveDirectChatMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest, com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> getRetrieveDirectChatMethod;
    if ((getRetrieveDirectChatMethod = SocialAdminServiceGrpc.getRetrieveDirectChatMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveDirectChatMethod = SocialAdminServiceGrpc.getRetrieveDirectChatMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveDirectChatMethod = getRetrieveDirectChatMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest, com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveDirectChat"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveDirectChat"))
              .build();
        }
      }
    }
    return getRetrieveDirectChatMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest,
      com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> getCreateExternalConnectionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateExternalConnection",
      requestType = com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest,
      com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> getCreateExternalConnectionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest, com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> getCreateExternalConnectionMethod;
    if ((getCreateExternalConnectionMethod = SocialAdminServiceGrpc.getCreateExternalConnectionMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateExternalConnectionMethod = SocialAdminServiceGrpc.getCreateExternalConnectionMethod) == null) {
          SocialAdminServiceGrpc.getCreateExternalConnectionMethod = getCreateExternalConnectionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest, com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateExternalConnection"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateExternalConnection"))
              .build();
        }
      }
    }
    return getCreateExternalConnectionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest,
      com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> getRetrieveExternalConnectionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveExternalConnection",
      requestType = com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest,
      com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> getRetrieveExternalConnectionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest, com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> getRetrieveExternalConnectionMethod;
    if ((getRetrieveExternalConnectionMethod = SocialAdminServiceGrpc.getRetrieveExternalConnectionMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveExternalConnectionMethod = SocialAdminServiceGrpc.getRetrieveExternalConnectionMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveExternalConnectionMethod = getRetrieveExternalConnectionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest, com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveExternalConnection"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveExternalConnection"))
              .build();
        }
      }
    }
    return getRetrieveExternalConnectionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest,
      com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> getCreateExternalMemberLinkMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateExternalMemberLink",
      requestType = com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest,
      com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> getCreateExternalMemberLinkMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest, com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> getCreateExternalMemberLinkMethod;
    if ((getCreateExternalMemberLinkMethod = SocialAdminServiceGrpc.getCreateExternalMemberLinkMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateExternalMemberLinkMethod = SocialAdminServiceGrpc.getCreateExternalMemberLinkMethod) == null) {
          SocialAdminServiceGrpc.getCreateExternalMemberLinkMethod = getCreateExternalMemberLinkMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest, com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateExternalMemberLink"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateExternalMemberLink"))
              .build();
        }
      }
    }
    return getCreateExternalMemberLinkMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest,
      com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> getRetrieveExternalMemberLinkMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveExternalMemberLink",
      requestType = com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest,
      com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> getRetrieveExternalMemberLinkMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest, com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> getRetrieveExternalMemberLinkMethod;
    if ((getRetrieveExternalMemberLinkMethod = SocialAdminServiceGrpc.getRetrieveExternalMemberLinkMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveExternalMemberLinkMethod = SocialAdminServiceGrpc.getRetrieveExternalMemberLinkMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveExternalMemberLinkMethod = getRetrieveExternalMemberLinkMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest, com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveExternalMemberLink"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveExternalMemberLink"))
              .build();
        }
      }
    }
    return getRetrieveExternalMemberLinkMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> getCreateManagedFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateManagedFriendRequest",
      requestType = com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> getCreateManagedFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> getCreateManagedFriendRequestMethod;
    if ((getCreateManagedFriendRequestMethod = SocialAdminServiceGrpc.getCreateManagedFriendRequestMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateManagedFriendRequestMethod = SocialAdminServiceGrpc.getCreateManagedFriendRequestMethod) == null) {
          SocialAdminServiceGrpc.getCreateManagedFriendRequestMethod = getCreateManagedFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateManagedFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateManagedFriendRequest"))
              .build();
        }
      }
    }
    return getCreateManagedFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> getRetrieveManagedFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveManagedFriendRequest",
      requestType = com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> getRetrieveManagedFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> getRetrieveManagedFriendRequestMethod;
    if ((getRetrieveManagedFriendRequestMethod = SocialAdminServiceGrpc.getRetrieveManagedFriendRequestMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveManagedFriendRequestMethod = SocialAdminServiceGrpc.getRetrieveManagedFriendRequestMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveManagedFriendRequestMethod = getRetrieveManagedFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveManagedFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveManagedFriendRequest"))
              .build();
        }
      }
    }
    return getRetrieveManagedFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> getAcceptManagedFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AcceptManagedFriendRequest",
      requestType = com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest.class,
      responseType = com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> getAcceptManagedFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> getAcceptManagedFriendRequestMethod;
    if ((getAcceptManagedFriendRequestMethod = SocialAdminServiceGrpc.getAcceptManagedFriendRequestMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getAcceptManagedFriendRequestMethod = SocialAdminServiceGrpc.getAcceptManagedFriendRequestMethod) == null) {
          SocialAdminServiceGrpc.getAcceptManagedFriendRequestMethod = getAcceptManagedFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AcceptManagedFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("AcceptManagedFriendRequest"))
              .build();
        }
      }
    }
    return getAcceptManagedFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> getDeclineManagedFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeclineManagedFriendRequest",
      requestType = com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest.class,
      responseType = com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> getDeclineManagedFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> getDeclineManagedFriendRequestMethod;
    if ((getDeclineManagedFriendRequestMethod = SocialAdminServiceGrpc.getDeclineManagedFriendRequestMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getDeclineManagedFriendRequestMethod = SocialAdminServiceGrpc.getDeclineManagedFriendRequestMethod) == null) {
          SocialAdminServiceGrpc.getDeclineManagedFriendRequestMethod = getDeclineManagedFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeclineManagedFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("DeclineManagedFriendRequest"))
              .build();
        }
      }
    }
    return getDeclineManagedFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> getCancelManagedFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CancelManagedFriendRequest",
      requestType = com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest,
      com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> getCancelManagedFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> getCancelManagedFriendRequestMethod;
    if ((getCancelManagedFriendRequestMethod = SocialAdminServiceGrpc.getCancelManagedFriendRequestMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCancelManagedFriendRequestMethod = SocialAdminServiceGrpc.getCancelManagedFriendRequestMethod) == null) {
          SocialAdminServiceGrpc.getCancelManagedFriendRequestMethod = getCancelManagedFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest, com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CancelManagedFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CancelManagedFriendRequest"))
              .build();
        }
      }
    }
    return getCancelManagedFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> getCreateManagedFriendshipMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateManagedFriendship",
      requestType = com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> getCreateManagedFriendshipMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest, com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> getCreateManagedFriendshipMethod;
    if ((getCreateManagedFriendshipMethod = SocialAdminServiceGrpc.getCreateManagedFriendshipMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateManagedFriendshipMethod = SocialAdminServiceGrpc.getCreateManagedFriendshipMethod) == null) {
          SocialAdminServiceGrpc.getCreateManagedFriendshipMethod = getCreateManagedFriendshipMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest, com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateManagedFriendship"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateManagedFriendship"))
              .build();
        }
      }
    }
    return getCreateManagedFriendshipMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> getRetrieveManagedFriendshipMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveManagedFriendship",
      requestType = com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> getRetrieveManagedFriendshipMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest, com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> getRetrieveManagedFriendshipMethod;
    if ((getRetrieveManagedFriendshipMethod = SocialAdminServiceGrpc.getRetrieveManagedFriendshipMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveManagedFriendshipMethod = SocialAdminServiceGrpc.getRetrieveManagedFriendshipMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveManagedFriendshipMethod = getRetrieveManagedFriendshipMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest, com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveManagedFriendship"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveManagedFriendship"))
              .build();
        }
      }
    }
    return getRetrieveManagedFriendshipMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> getRemoveManagedFriendshipMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveManagedFriendship",
      requestType = com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest,
      com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> getRemoveManagedFriendshipMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest, com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> getRemoveManagedFriendshipMethod;
    if ((getRemoveManagedFriendshipMethod = SocialAdminServiceGrpc.getRemoveManagedFriendshipMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRemoveManagedFriendshipMethod = SocialAdminServiceGrpc.getRemoveManagedFriendshipMethod) == null) {
          SocialAdminServiceGrpc.getRemoveManagedFriendshipMethod = getRemoveManagedFriendshipMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest, com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveManagedFriendship"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RemoveManagedFriendship"))
              .build();
        }
      }
    }
    return getRemoveManagedFriendshipMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest,
      com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> getCreateSharedChannelPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateSharedChannelPolicy",
      requestType = com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest,
      com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> getCreateSharedChannelPolicyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest, com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> getCreateSharedChannelPolicyMethod;
    if ((getCreateSharedChannelPolicyMethod = SocialAdminServiceGrpc.getCreateSharedChannelPolicyMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateSharedChannelPolicyMethod = SocialAdminServiceGrpc.getCreateSharedChannelPolicyMethod) == null) {
          SocialAdminServiceGrpc.getCreateSharedChannelPolicyMethod = getCreateSharedChannelPolicyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest, com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateSharedChannelPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateSharedChannelPolicy"))
              .build();
        }
      }
    }
    return getCreateSharedChannelPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest,
      com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> getRetrieveSharedChannelPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveSharedChannelPolicy",
      requestType = com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest,
      com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> getRetrieveSharedChannelPolicyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest, com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> getRetrieveSharedChannelPolicyMethod;
    if ((getRetrieveSharedChannelPolicyMethod = SocialAdminServiceGrpc.getRetrieveSharedChannelPolicyMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveSharedChannelPolicyMethod = SocialAdminServiceGrpc.getRetrieveSharedChannelPolicyMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveSharedChannelPolicyMethod = getRetrieveSharedChannelPolicyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest, com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveSharedChannelPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveSharedChannelPolicy"))
              .build();
        }
      }
    }
    return getRetrieveSharedChannelPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateUserBlockRequest,
      com.sdkwork.communication.backend.v3.CreateUserBlockResponse> getCreateUserBlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateUserBlock",
      requestType = com.sdkwork.communication.backend.v3.CreateUserBlockRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateUserBlockResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateUserBlockRequest,
      com.sdkwork.communication.backend.v3.CreateUserBlockResponse> getCreateUserBlockMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateUserBlockRequest, com.sdkwork.communication.backend.v3.CreateUserBlockResponse> getCreateUserBlockMethod;
    if ((getCreateUserBlockMethod = SocialAdminServiceGrpc.getCreateUserBlockMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getCreateUserBlockMethod = SocialAdminServiceGrpc.getCreateUserBlockMethod) == null) {
          SocialAdminServiceGrpc.getCreateUserBlockMethod = getCreateUserBlockMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateUserBlockRequest, com.sdkwork.communication.backend.v3.CreateUserBlockResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateUserBlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateUserBlockRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateUserBlockResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("CreateUserBlock"))
              .build();
        }
      }
    }
    return getCreateUserBlockMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest,
      com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> getRetrieveUserBlockMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveUserBlock",
      requestType = com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest,
      com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> getRetrieveUserBlockMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest, com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> getRetrieveUserBlockMethod;
    if ((getRetrieveUserBlockMethod = SocialAdminServiceGrpc.getRetrieveUserBlockMethod) == null) {
      synchronized (SocialAdminServiceGrpc.class) {
        if ((getRetrieveUserBlockMethod = SocialAdminServiceGrpc.getRetrieveUserBlockMethod) == null) {
          SocialAdminServiceGrpc.getRetrieveUserBlockMethod = getRetrieveUserBlockMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest, com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveUserBlock"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialAdminServiceMethodDescriptorSupplier("RetrieveUserBlock"))
              .build();
        }
      }
    }
    return getRetrieveUserBlockMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SocialAdminServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceStub>() {
        @java.lang.Override
        public SocialAdminServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialAdminServiceStub(channel, callOptions);
        }
      };
    return SocialAdminServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SocialAdminServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceBlockingV2Stub>() {
        @java.lang.Override
        public SocialAdminServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialAdminServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return SocialAdminServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SocialAdminServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceBlockingStub>() {
        @java.lang.Override
        public SocialAdminServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialAdminServiceBlockingStub(channel, callOptions);
        }
      };
    return SocialAdminServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SocialAdminServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialAdminServiceFutureStub>() {
        @java.lang.Override
        public SocialAdminServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialAdminServiceFutureStub(channel, callOptions);
        }
      };
    return SocialAdminServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createDirectChatBinding(com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateDirectChatBindingMethod(), responseObserver);
    }

    /**
     */
    default void retrieveDirectChat(com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveDirectChatMethod(), responseObserver);
    }

    /**
     */
    default void createExternalConnection(com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateExternalConnectionMethod(), responseObserver);
    }

    /**
     */
    default void retrieveExternalConnection(com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveExternalConnectionMethod(), responseObserver);
    }

    /**
     */
    default void createExternalMemberLink(com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateExternalMemberLinkMethod(), responseObserver);
    }

    /**
     */
    default void retrieveExternalMemberLink(com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveExternalMemberLinkMethod(), responseObserver);
    }

    /**
     */
    default void createManagedFriendRequest(com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateManagedFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void retrieveManagedFriendRequest(com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveManagedFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void acceptManagedFriendRequest(com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAcceptManagedFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void declineManagedFriendRequest(com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeclineManagedFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void cancelManagedFriendRequest(com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCancelManagedFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void createManagedFriendship(com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateManagedFriendshipMethod(), responseObserver);
    }

    /**
     */
    default void retrieveManagedFriendship(com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveManagedFriendshipMethod(), responseObserver);
    }

    /**
     */
    default void removeManagedFriendship(com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveManagedFriendshipMethod(), responseObserver);
    }

    /**
     */
    default void createSharedChannelPolicy(com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateSharedChannelPolicyMethod(), responseObserver);
    }

    /**
     */
    default void retrieveSharedChannelPolicy(com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveSharedChannelPolicyMethod(), responseObserver);
    }

    /**
     */
    default void createUserBlock(com.sdkwork.communication.backend.v3.CreateUserBlockRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateUserBlockResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateUserBlockMethod(), responseObserver);
    }

    /**
     */
    default void retrieveUserBlock(com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveUserBlockMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service SocialAdminService.
   */
  public static abstract class SocialAdminServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SocialAdminServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service SocialAdminService.
   */
  public static final class SocialAdminServiceStub
      extends io.grpc.stub.AbstractAsyncStub<SocialAdminServiceStub> {
    private SocialAdminServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialAdminServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialAdminServiceStub(channel, callOptions);
    }

    /**
     */
    public void createDirectChatBinding(com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateDirectChatBindingMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveDirectChat(com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveDirectChatMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createExternalConnection(com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateExternalConnectionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveExternalConnection(com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveExternalConnectionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createExternalMemberLink(com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateExternalMemberLinkMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveExternalMemberLink(com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveExternalMemberLinkMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createManagedFriendRequest(com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateManagedFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveManagedFriendRequest(com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveManagedFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void acceptManagedFriendRequest(com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAcceptManagedFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void declineManagedFriendRequest(com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeclineManagedFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void cancelManagedFriendRequest(com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCancelManagedFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createManagedFriendship(com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateManagedFriendshipMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveManagedFriendship(com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveManagedFriendshipMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeManagedFriendship(com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveManagedFriendshipMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createSharedChannelPolicy(com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateSharedChannelPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveSharedChannelPolicy(com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveSharedChannelPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createUserBlock(com.sdkwork.communication.backend.v3.CreateUserBlockRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateUserBlockResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateUserBlockMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveUserBlock(com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveUserBlockMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service SocialAdminService.
   */
  public static final class SocialAdminServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SocialAdminServiceBlockingV2Stub> {
    private SocialAdminServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialAdminServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialAdminServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse createDirectChatBinding(com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateDirectChatBindingMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse retrieveDirectChat(com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveDirectChatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse createExternalConnection(com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateExternalConnectionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse retrieveExternalConnection(com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveExternalConnectionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse createExternalMemberLink(com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateExternalMemberLinkMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse retrieveExternalMemberLink(com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveExternalMemberLinkMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse createManagedFriendRequest(com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse retrieveManagedFriendRequest(com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse acceptManagedFriendRequest(com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAcceptManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse declineManagedFriendRequest(com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeclineManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse cancelManagedFriendRequest(com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCancelManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse createManagedFriendship(com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse retrieveManagedFriendship(com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse removeManagedFriendship(com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRemoveManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse createSharedChannelPolicy(com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateSharedChannelPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse retrieveSharedChannelPolicy(com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveSharedChannelPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateUserBlockResponse createUserBlock(com.sdkwork.communication.backend.v3.CreateUserBlockRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateUserBlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse retrieveUserBlock(com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveUserBlockMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service SocialAdminService.
   */
  public static final class SocialAdminServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SocialAdminServiceBlockingStub> {
    private SocialAdminServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialAdminServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialAdminServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse createDirectChatBinding(com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateDirectChatBindingMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse retrieveDirectChat(com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveDirectChatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse createExternalConnection(com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateExternalConnectionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse retrieveExternalConnection(com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveExternalConnectionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse createExternalMemberLink(com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateExternalMemberLinkMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse retrieveExternalMemberLink(com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveExternalMemberLinkMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse createManagedFriendRequest(com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse retrieveManagedFriendRequest(com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse acceptManagedFriendRequest(com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAcceptManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse declineManagedFriendRequest(com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeclineManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse cancelManagedFriendRequest(com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCancelManagedFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse createManagedFriendship(com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse retrieveManagedFriendship(com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse removeManagedFriendship(com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveManagedFriendshipMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse createSharedChannelPolicy(com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateSharedChannelPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse retrieveSharedChannelPolicy(com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveSharedChannelPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateUserBlockResponse createUserBlock(com.sdkwork.communication.backend.v3.CreateUserBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateUserBlockMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse retrieveUserBlock(com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveUserBlockMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service SocialAdminService.
   */
  public static final class SocialAdminServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<SocialAdminServiceFutureStub> {
    private SocialAdminServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialAdminServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialAdminServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse> createDirectChatBinding(
        com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateDirectChatBindingMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse> retrieveDirectChat(
        com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveDirectChatMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse> createExternalConnection(
        com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateExternalConnectionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse> retrieveExternalConnection(
        com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveExternalConnectionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse> createExternalMemberLink(
        com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateExternalMemberLinkMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse> retrieveExternalMemberLink(
        com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveExternalMemberLinkMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse> createManagedFriendRequest(
        com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateManagedFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse> retrieveManagedFriendRequest(
        com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveManagedFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse> acceptManagedFriendRequest(
        com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAcceptManagedFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse> declineManagedFriendRequest(
        com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeclineManagedFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse> cancelManagedFriendRequest(
        com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCancelManagedFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse> createManagedFriendship(
        com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateManagedFriendshipMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse> retrieveManagedFriendship(
        com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveManagedFriendshipMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse> removeManagedFriendship(
        com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveManagedFriendshipMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse> createSharedChannelPolicy(
        com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateSharedChannelPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse> retrieveSharedChannelPolicy(
        com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveSharedChannelPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateUserBlockResponse> createUserBlock(
        com.sdkwork.communication.backend.v3.CreateUserBlockRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateUserBlockMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse> retrieveUserBlock(
        com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveUserBlockMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_DIRECT_CHAT_BINDING = 0;
  private static final int METHODID_RETRIEVE_DIRECT_CHAT = 1;
  private static final int METHODID_CREATE_EXTERNAL_CONNECTION = 2;
  private static final int METHODID_RETRIEVE_EXTERNAL_CONNECTION = 3;
  private static final int METHODID_CREATE_EXTERNAL_MEMBER_LINK = 4;
  private static final int METHODID_RETRIEVE_EXTERNAL_MEMBER_LINK = 5;
  private static final int METHODID_CREATE_MANAGED_FRIEND_REQUEST = 6;
  private static final int METHODID_RETRIEVE_MANAGED_FRIEND_REQUEST = 7;
  private static final int METHODID_ACCEPT_MANAGED_FRIEND_REQUEST = 8;
  private static final int METHODID_DECLINE_MANAGED_FRIEND_REQUEST = 9;
  private static final int METHODID_CANCEL_MANAGED_FRIEND_REQUEST = 10;
  private static final int METHODID_CREATE_MANAGED_FRIENDSHIP = 11;
  private static final int METHODID_RETRIEVE_MANAGED_FRIENDSHIP = 12;
  private static final int METHODID_REMOVE_MANAGED_FRIENDSHIP = 13;
  private static final int METHODID_CREATE_SHARED_CHANNEL_POLICY = 14;
  private static final int METHODID_RETRIEVE_SHARED_CHANNEL_POLICY = 15;
  private static final int METHODID_CREATE_USER_BLOCK = 16;
  private static final int METHODID_RETRIEVE_USER_BLOCK = 17;

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
        case METHODID_CREATE_DIRECT_CHAT_BINDING:
          serviceImpl.createDirectChatBinding((com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_DIRECT_CHAT:
          serviceImpl.retrieveDirectChat((com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse>) responseObserver);
          break;
        case METHODID_CREATE_EXTERNAL_CONNECTION:
          serviceImpl.createExternalConnection((com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_EXTERNAL_CONNECTION:
          serviceImpl.retrieveExternalConnection((com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse>) responseObserver);
          break;
        case METHODID_CREATE_EXTERNAL_MEMBER_LINK:
          serviceImpl.createExternalMemberLink((com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_EXTERNAL_MEMBER_LINK:
          serviceImpl.retrieveExternalMemberLink((com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse>) responseObserver);
          break;
        case METHODID_CREATE_MANAGED_FRIEND_REQUEST:
          serviceImpl.createManagedFriendRequest((com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_MANAGED_FRIEND_REQUEST:
          serviceImpl.retrieveManagedFriendRequest((com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse>) responseObserver);
          break;
        case METHODID_ACCEPT_MANAGED_FRIEND_REQUEST:
          serviceImpl.acceptManagedFriendRequest((com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse>) responseObserver);
          break;
        case METHODID_DECLINE_MANAGED_FRIEND_REQUEST:
          serviceImpl.declineManagedFriendRequest((com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse>) responseObserver);
          break;
        case METHODID_CANCEL_MANAGED_FRIEND_REQUEST:
          serviceImpl.cancelManagedFriendRequest((com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse>) responseObserver);
          break;
        case METHODID_CREATE_MANAGED_FRIENDSHIP:
          serviceImpl.createManagedFriendship((com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_MANAGED_FRIENDSHIP:
          serviceImpl.retrieveManagedFriendship((com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse>) responseObserver);
          break;
        case METHODID_REMOVE_MANAGED_FRIENDSHIP:
          serviceImpl.removeManagedFriendship((com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse>) responseObserver);
          break;
        case METHODID_CREATE_SHARED_CHANNEL_POLICY:
          serviceImpl.createSharedChannelPolicy((com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_SHARED_CHANNEL_POLICY:
          serviceImpl.retrieveSharedChannelPolicy((com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse>) responseObserver);
          break;
        case METHODID_CREATE_USER_BLOCK:
          serviceImpl.createUserBlock((com.sdkwork.communication.backend.v3.CreateUserBlockRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateUserBlockResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_USER_BLOCK:
          serviceImpl.retrieveUserBlock((com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse>) responseObserver);
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
          getCreateDirectChatBindingMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateDirectChatBindingRequest,
              com.sdkwork.communication.backend.v3.CreateDirectChatBindingResponse>(
                service, METHODID_CREATE_DIRECT_CHAT_BINDING)))
        .addMethod(
          getRetrieveDirectChatMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveDirectChatRequest,
              com.sdkwork.communication.backend.v3.RetrieveDirectChatResponse>(
                service, METHODID_RETRIEVE_DIRECT_CHAT)))
        .addMethod(
          getCreateExternalConnectionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateExternalConnectionRequest,
              com.sdkwork.communication.backend.v3.CreateExternalConnectionResponse>(
                service, METHODID_CREATE_EXTERNAL_CONNECTION)))
        .addMethod(
          getRetrieveExternalConnectionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveExternalConnectionRequest,
              com.sdkwork.communication.backend.v3.RetrieveExternalConnectionResponse>(
                service, METHODID_RETRIEVE_EXTERNAL_CONNECTION)))
        .addMethod(
          getCreateExternalMemberLinkMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateExternalMemberLinkRequest,
              com.sdkwork.communication.backend.v3.CreateExternalMemberLinkResponse>(
                service, METHODID_CREATE_EXTERNAL_MEMBER_LINK)))
        .addMethod(
          getRetrieveExternalMemberLinkMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkRequest,
              com.sdkwork.communication.backend.v3.RetrieveExternalMemberLinkResponse>(
                service, METHODID_RETRIEVE_EXTERNAL_MEMBER_LINK)))
        .addMethod(
          getCreateManagedFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateManagedFriendRequestRequest,
              com.sdkwork.communication.backend.v3.CreateManagedFriendRequestResponse>(
                service, METHODID_CREATE_MANAGED_FRIEND_REQUEST)))
        .addMethod(
          getRetrieveManagedFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestRequest,
              com.sdkwork.communication.backend.v3.RetrieveManagedFriendRequestResponse>(
                service, METHODID_RETRIEVE_MANAGED_FRIEND_REQUEST)))
        .addMethod(
          getAcceptManagedFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestRequest,
              com.sdkwork.communication.backend.v3.AcceptManagedFriendRequestResponse>(
                service, METHODID_ACCEPT_MANAGED_FRIEND_REQUEST)))
        .addMethod(
          getDeclineManagedFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestRequest,
              com.sdkwork.communication.backend.v3.DeclineManagedFriendRequestResponse>(
                service, METHODID_DECLINE_MANAGED_FRIEND_REQUEST)))
        .addMethod(
          getCancelManagedFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CancelManagedFriendRequestRequest,
              com.sdkwork.communication.backend.v3.CancelManagedFriendRequestResponse>(
                service, METHODID_CANCEL_MANAGED_FRIEND_REQUEST)))
        .addMethod(
          getCreateManagedFriendshipMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateManagedFriendshipRequest,
              com.sdkwork.communication.backend.v3.CreateManagedFriendshipResponse>(
                service, METHODID_CREATE_MANAGED_FRIENDSHIP)))
        .addMethod(
          getRetrieveManagedFriendshipMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipRequest,
              com.sdkwork.communication.backend.v3.RetrieveManagedFriendshipResponse>(
                service, METHODID_RETRIEVE_MANAGED_FRIENDSHIP)))
        .addMethod(
          getRemoveManagedFriendshipMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RemoveManagedFriendshipRequest,
              com.sdkwork.communication.backend.v3.RemoveManagedFriendshipResponse>(
                service, METHODID_REMOVE_MANAGED_FRIENDSHIP)))
        .addMethod(
          getCreateSharedChannelPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyRequest,
              com.sdkwork.communication.backend.v3.CreateSharedChannelPolicyResponse>(
                service, METHODID_CREATE_SHARED_CHANNEL_POLICY)))
        .addMethod(
          getRetrieveSharedChannelPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyRequest,
              com.sdkwork.communication.backend.v3.RetrieveSharedChannelPolicyResponse>(
                service, METHODID_RETRIEVE_SHARED_CHANNEL_POLICY)))
        .addMethod(
          getCreateUserBlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateUserBlockRequest,
              com.sdkwork.communication.backend.v3.CreateUserBlockResponse>(
                service, METHODID_CREATE_USER_BLOCK)))
        .addMethod(
          getRetrieveUserBlockMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveUserBlockRequest,
              com.sdkwork.communication.backend.v3.RetrieveUserBlockResponse>(
                service, METHODID_RETRIEVE_USER_BLOCK)))
        .build();
  }

  private static abstract class SocialAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SocialAdminServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("SocialAdminService");
    }
  }

  private static final class SocialAdminServiceFileDescriptorSupplier
      extends SocialAdminServiceBaseDescriptorSupplier {
    SocialAdminServiceFileDescriptorSupplier() {}
  }

  private static final class SocialAdminServiceMethodDescriptorSupplier
      extends SocialAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SocialAdminServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SocialAdminServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SocialAdminServiceFileDescriptorSupplier())
              .addMethod(getCreateDirectChatBindingMethod())
              .addMethod(getRetrieveDirectChatMethod())
              .addMethod(getCreateExternalConnectionMethod())
              .addMethod(getRetrieveExternalConnectionMethod())
              .addMethod(getCreateExternalMemberLinkMethod())
              .addMethod(getRetrieveExternalMemberLinkMethod())
              .addMethod(getCreateManagedFriendRequestMethod())
              .addMethod(getRetrieveManagedFriendRequestMethod())
              .addMethod(getAcceptManagedFriendRequestMethod())
              .addMethod(getDeclineManagedFriendRequestMethod())
              .addMethod(getCancelManagedFriendRequestMethod())
              .addMethod(getCreateManagedFriendshipMethod())
              .addMethod(getRetrieveManagedFriendshipMethod())
              .addMethod(getRemoveManagedFriendshipMethod())
              .addMethod(getCreateSharedChannelPolicyMethod())
              .addMethod(getRetrieveSharedChannelPolicyMethod())
              .addMethod(getCreateUserBlockMethod())
              .addMethod(getRetrieveUserBlockMethod())
              .build();
        }
      }
    }
    return result;
  }
}
