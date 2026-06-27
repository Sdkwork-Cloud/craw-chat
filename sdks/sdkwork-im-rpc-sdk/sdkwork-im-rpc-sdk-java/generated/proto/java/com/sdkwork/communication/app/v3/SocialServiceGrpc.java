package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SocialServiceGrpc {

  private SocialServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.SocialService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListSocialUsersRequest,
      com.sdkwork.communication.app.v3.ListSocialUsersResponse> getListSocialUsersMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListSocialUsers",
      requestType = com.sdkwork.communication.app.v3.ListSocialUsersRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListSocialUsersResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListSocialUsersRequest,
      com.sdkwork.communication.app.v3.ListSocialUsersResponse> getListSocialUsersMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListSocialUsersRequest, com.sdkwork.communication.app.v3.ListSocialUsersResponse> getListSocialUsersMethod;
    if ((getListSocialUsersMethod = SocialServiceGrpc.getListSocialUsersMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getListSocialUsersMethod = SocialServiceGrpc.getListSocialUsersMethod) == null) {
          SocialServiceGrpc.getListSocialUsersMethod = getListSocialUsersMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListSocialUsersRequest, com.sdkwork.communication.app.v3.ListSocialUsersResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListSocialUsers"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListSocialUsersRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListSocialUsersResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("ListSocialUsers"))
              .build();
        }
      }
    }
    return getListSocialUsersMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFriendRequestsRequest,
      com.sdkwork.communication.app.v3.ListFriendRequestsResponse> getListFriendRequestsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListFriendRequests",
      requestType = com.sdkwork.communication.app.v3.ListFriendRequestsRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListFriendRequestsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFriendRequestsRequest,
      com.sdkwork.communication.app.v3.ListFriendRequestsResponse> getListFriendRequestsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFriendRequestsRequest, com.sdkwork.communication.app.v3.ListFriendRequestsResponse> getListFriendRequestsMethod;
    if ((getListFriendRequestsMethod = SocialServiceGrpc.getListFriendRequestsMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getListFriendRequestsMethod = SocialServiceGrpc.getListFriendRequestsMethod) == null) {
          SocialServiceGrpc.getListFriendRequestsMethod = getListFriendRequestsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListFriendRequestsRequest, com.sdkwork.communication.app.v3.ListFriendRequestsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListFriendRequests"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListFriendRequestsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListFriendRequestsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("ListFriendRequests"))
              .build();
        }
      }
    }
    return getListFriendRequestsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateFriendRequestRequest,
      com.sdkwork.communication.app.v3.CreateFriendRequestResponse> getCreateFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateFriendRequest",
      requestType = com.sdkwork.communication.app.v3.CreateFriendRequestRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateFriendRequestRequest,
      com.sdkwork.communication.app.v3.CreateFriendRequestResponse> getCreateFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateFriendRequestRequest, com.sdkwork.communication.app.v3.CreateFriendRequestResponse> getCreateFriendRequestMethod;
    if ((getCreateFriendRequestMethod = SocialServiceGrpc.getCreateFriendRequestMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getCreateFriendRequestMethod = SocialServiceGrpc.getCreateFriendRequestMethod) == null) {
          SocialServiceGrpc.getCreateFriendRequestMethod = getCreateFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateFriendRequestRequest, com.sdkwork.communication.app.v3.CreateFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("CreateFriendRequest"))
              .build();
        }
      }
    }
    return getCreateFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptFriendRequestRequest,
      com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> getAcceptFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AcceptFriendRequest",
      requestType = com.sdkwork.communication.app.v3.AcceptFriendRequestRequest.class,
      responseType = com.sdkwork.communication.app.v3.AcceptFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptFriendRequestRequest,
      com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> getAcceptFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptFriendRequestRequest, com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> getAcceptFriendRequestMethod;
    if ((getAcceptFriendRequestMethod = SocialServiceGrpc.getAcceptFriendRequestMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getAcceptFriendRequestMethod = SocialServiceGrpc.getAcceptFriendRequestMethod) == null) {
          SocialServiceGrpc.getAcceptFriendRequestMethod = getAcceptFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AcceptFriendRequestRequest, com.sdkwork.communication.app.v3.AcceptFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AcceptFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AcceptFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AcceptFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("AcceptFriendRequest"))
              .build();
        }
      }
    }
    return getAcceptFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeclineFriendRequestRequest,
      com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> getDeclineFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeclineFriendRequest",
      requestType = com.sdkwork.communication.app.v3.DeclineFriendRequestRequest.class,
      responseType = com.sdkwork.communication.app.v3.DeclineFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeclineFriendRequestRequest,
      com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> getDeclineFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeclineFriendRequestRequest, com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> getDeclineFriendRequestMethod;
    if ((getDeclineFriendRequestMethod = SocialServiceGrpc.getDeclineFriendRequestMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getDeclineFriendRequestMethod = SocialServiceGrpc.getDeclineFriendRequestMethod) == null) {
          SocialServiceGrpc.getDeclineFriendRequestMethod = getDeclineFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.DeclineFriendRequestRequest, com.sdkwork.communication.app.v3.DeclineFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeclineFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeclineFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeclineFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("DeclineFriendRequest"))
              .build();
        }
      }
    }
    return getDeclineFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CancelFriendRequestRequest,
      com.sdkwork.communication.app.v3.CancelFriendRequestResponse> getCancelFriendRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CancelFriendRequest",
      requestType = com.sdkwork.communication.app.v3.CancelFriendRequestRequest.class,
      responseType = com.sdkwork.communication.app.v3.CancelFriendRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CancelFriendRequestRequest,
      com.sdkwork.communication.app.v3.CancelFriendRequestResponse> getCancelFriendRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CancelFriendRequestRequest, com.sdkwork.communication.app.v3.CancelFriendRequestResponse> getCancelFriendRequestMethod;
    if ((getCancelFriendRequestMethod = SocialServiceGrpc.getCancelFriendRequestMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getCancelFriendRequestMethod = SocialServiceGrpc.getCancelFriendRequestMethod) == null) {
          SocialServiceGrpc.getCancelFriendRequestMethod = getCancelFriendRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CancelFriendRequestRequest, com.sdkwork.communication.app.v3.CancelFriendRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CancelFriendRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CancelFriendRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CancelFriendRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("CancelFriendRequest"))
              .build();
        }
      }
    }
    return getCancelFriendRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveFriendshipRequest,
      com.sdkwork.communication.app.v3.RemoveFriendshipResponse> getRemoveFriendshipMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RemoveFriendship",
      requestType = com.sdkwork.communication.app.v3.RemoveFriendshipRequest.class,
      responseType = com.sdkwork.communication.app.v3.RemoveFriendshipResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveFriendshipRequest,
      com.sdkwork.communication.app.v3.RemoveFriendshipResponse> getRemoveFriendshipMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RemoveFriendshipRequest, com.sdkwork.communication.app.v3.RemoveFriendshipResponse> getRemoveFriendshipMethod;
    if ((getRemoveFriendshipMethod = SocialServiceGrpc.getRemoveFriendshipMethod) == null) {
      synchronized (SocialServiceGrpc.class) {
        if ((getRemoveFriendshipMethod = SocialServiceGrpc.getRemoveFriendshipMethod) == null) {
          SocialServiceGrpc.getRemoveFriendshipMethod = getRemoveFriendshipMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RemoveFriendshipRequest, com.sdkwork.communication.app.v3.RemoveFriendshipResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RemoveFriendship"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RemoveFriendshipRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RemoveFriendshipResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialServiceMethodDescriptorSupplier("RemoveFriendship"))
              .build();
        }
      }
    }
    return getRemoveFriendshipMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SocialServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialServiceStub>() {
        @java.lang.Override
        public SocialServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialServiceStub(channel, callOptions);
        }
      };
    return SocialServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SocialServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialServiceBlockingV2Stub>() {
        @java.lang.Override
        public SocialServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return SocialServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SocialServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialServiceBlockingStub>() {
        @java.lang.Override
        public SocialServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialServiceBlockingStub(channel, callOptions);
        }
      };
    return SocialServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SocialServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialServiceFutureStub>() {
        @java.lang.Override
        public SocialServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialServiceFutureStub(channel, callOptions);
        }
      };
    return SocialServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listSocialUsers(com.sdkwork.communication.app.v3.ListSocialUsersRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListSocialUsersResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListSocialUsersMethod(), responseObserver);
    }

    /**
     */
    default void listFriendRequests(com.sdkwork.communication.app.v3.ListFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListFriendRequestsMethod(), responseObserver);
    }

    /**
     */
    default void createFriendRequest(com.sdkwork.communication.app.v3.CreateFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void acceptFriendRequest(com.sdkwork.communication.app.v3.AcceptFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAcceptFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void declineFriendRequest(com.sdkwork.communication.app.v3.DeclineFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeclineFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void cancelFriendRequest(com.sdkwork.communication.app.v3.CancelFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CancelFriendRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCancelFriendRequestMethod(), responseObserver);
    }

    /**
     */
    default void removeFriendship(com.sdkwork.communication.app.v3.RemoveFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveFriendshipResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRemoveFriendshipMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service SocialService.
   */
  public static abstract class SocialServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SocialServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service SocialService.
   */
  public static final class SocialServiceStub
      extends io.grpc.stub.AbstractAsyncStub<SocialServiceStub> {
    private SocialServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialServiceStub(channel, callOptions);
    }

    /**
     */
    public void listSocialUsers(com.sdkwork.communication.app.v3.ListSocialUsersRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListSocialUsersResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListSocialUsersMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listFriendRequests(com.sdkwork.communication.app.v3.ListFriendRequestsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFriendRequestsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListFriendRequestsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createFriendRequest(com.sdkwork.communication.app.v3.CreateFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void acceptFriendRequest(com.sdkwork.communication.app.v3.AcceptFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAcceptFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void declineFriendRequest(com.sdkwork.communication.app.v3.DeclineFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeclineFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void cancelFriendRequest(com.sdkwork.communication.app.v3.CancelFriendRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CancelFriendRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCancelFriendRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void removeFriendship(com.sdkwork.communication.app.v3.RemoveFriendshipRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveFriendshipResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRemoveFriendshipMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service SocialService.
   */
  public static final class SocialServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SocialServiceBlockingV2Stub> {
    private SocialServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListSocialUsersResponse listSocialUsers(com.sdkwork.communication.app.v3.ListSocialUsersRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListSocialUsersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListFriendRequestsResponse listFriendRequests(com.sdkwork.communication.app.v3.ListFriendRequestsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListFriendRequestsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateFriendRequestResponse createFriendRequest(com.sdkwork.communication.app.v3.CreateFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AcceptFriendRequestResponse acceptFriendRequest(com.sdkwork.communication.app.v3.AcceptFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAcceptFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeclineFriendRequestResponse declineFriendRequest(com.sdkwork.communication.app.v3.DeclineFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeclineFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CancelFriendRequestResponse cancelFriendRequest(com.sdkwork.communication.app.v3.CancelFriendRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCancelFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RemoveFriendshipResponse removeFriendship(com.sdkwork.communication.app.v3.RemoveFriendshipRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRemoveFriendshipMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service SocialService.
   */
  public static final class SocialServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SocialServiceBlockingStub> {
    private SocialServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListSocialUsersResponse listSocialUsers(com.sdkwork.communication.app.v3.ListSocialUsersRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListSocialUsersMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListFriendRequestsResponse listFriendRequests(com.sdkwork.communication.app.v3.ListFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListFriendRequestsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateFriendRequestResponse createFriendRequest(com.sdkwork.communication.app.v3.CreateFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AcceptFriendRequestResponse acceptFriendRequest(com.sdkwork.communication.app.v3.AcceptFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAcceptFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeclineFriendRequestResponse declineFriendRequest(com.sdkwork.communication.app.v3.DeclineFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeclineFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CancelFriendRequestResponse cancelFriendRequest(com.sdkwork.communication.app.v3.CancelFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCancelFriendRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RemoveFriendshipResponse removeFriendship(com.sdkwork.communication.app.v3.RemoveFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRemoveFriendshipMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service SocialService.
   */
  public static final class SocialServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<SocialServiceFutureStub> {
    private SocialServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListSocialUsersResponse> listSocialUsers(
        com.sdkwork.communication.app.v3.ListSocialUsersRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListSocialUsersMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListFriendRequestsResponse> listFriendRequests(
        com.sdkwork.communication.app.v3.ListFriendRequestsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListFriendRequestsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateFriendRequestResponse> createFriendRequest(
        com.sdkwork.communication.app.v3.CreateFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AcceptFriendRequestResponse> acceptFriendRequest(
        com.sdkwork.communication.app.v3.AcceptFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAcceptFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.DeclineFriendRequestResponse> declineFriendRequest(
        com.sdkwork.communication.app.v3.DeclineFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeclineFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CancelFriendRequestResponse> cancelFriendRequest(
        com.sdkwork.communication.app.v3.CancelFriendRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCancelFriendRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RemoveFriendshipResponse> removeFriendship(
        com.sdkwork.communication.app.v3.RemoveFriendshipRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRemoveFriendshipMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_SOCIAL_USERS = 0;
  private static final int METHODID_LIST_FRIEND_REQUESTS = 1;
  private static final int METHODID_CREATE_FRIEND_REQUEST = 2;
  private static final int METHODID_ACCEPT_FRIEND_REQUEST = 3;
  private static final int METHODID_DECLINE_FRIEND_REQUEST = 4;
  private static final int METHODID_CANCEL_FRIEND_REQUEST = 5;
  private static final int METHODID_REMOVE_FRIENDSHIP = 6;

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
        case METHODID_LIST_SOCIAL_USERS:
          serviceImpl.listSocialUsers((com.sdkwork.communication.app.v3.ListSocialUsersRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListSocialUsersResponse>) responseObserver);
          break;
        case METHODID_LIST_FRIEND_REQUESTS:
          serviceImpl.listFriendRequests((com.sdkwork.communication.app.v3.ListFriendRequestsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFriendRequestsResponse>) responseObserver);
          break;
        case METHODID_CREATE_FRIEND_REQUEST:
          serviceImpl.createFriendRequest((com.sdkwork.communication.app.v3.CreateFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateFriendRequestResponse>) responseObserver);
          break;
        case METHODID_ACCEPT_FRIEND_REQUEST:
          serviceImpl.acceptFriendRequest((com.sdkwork.communication.app.v3.AcceptFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptFriendRequestResponse>) responseObserver);
          break;
        case METHODID_DECLINE_FRIEND_REQUEST:
          serviceImpl.declineFriendRequest((com.sdkwork.communication.app.v3.DeclineFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeclineFriendRequestResponse>) responseObserver);
          break;
        case METHODID_CANCEL_FRIEND_REQUEST:
          serviceImpl.cancelFriendRequest((com.sdkwork.communication.app.v3.CancelFriendRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CancelFriendRequestResponse>) responseObserver);
          break;
        case METHODID_REMOVE_FRIENDSHIP:
          serviceImpl.removeFriendship((com.sdkwork.communication.app.v3.RemoveFriendshipRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RemoveFriendshipResponse>) responseObserver);
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
          getListSocialUsersMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListSocialUsersRequest,
              com.sdkwork.communication.app.v3.ListSocialUsersResponse>(
                service, METHODID_LIST_SOCIAL_USERS)))
        .addMethod(
          getListFriendRequestsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListFriendRequestsRequest,
              com.sdkwork.communication.app.v3.ListFriendRequestsResponse>(
                service, METHODID_LIST_FRIEND_REQUESTS)))
        .addMethod(
          getCreateFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateFriendRequestRequest,
              com.sdkwork.communication.app.v3.CreateFriendRequestResponse>(
                service, METHODID_CREATE_FRIEND_REQUEST)))
        .addMethod(
          getAcceptFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AcceptFriendRequestRequest,
              com.sdkwork.communication.app.v3.AcceptFriendRequestResponse>(
                service, METHODID_ACCEPT_FRIEND_REQUEST)))
        .addMethod(
          getDeclineFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.DeclineFriendRequestRequest,
              com.sdkwork.communication.app.v3.DeclineFriendRequestResponse>(
                service, METHODID_DECLINE_FRIEND_REQUEST)))
        .addMethod(
          getCancelFriendRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CancelFriendRequestRequest,
              com.sdkwork.communication.app.v3.CancelFriendRequestResponse>(
                service, METHODID_CANCEL_FRIEND_REQUEST)))
        .addMethod(
          getRemoveFriendshipMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RemoveFriendshipRequest,
              com.sdkwork.communication.app.v3.RemoveFriendshipResponse>(
                service, METHODID_REMOVE_FRIENDSHIP)))
        .build();
  }

  private static abstract class SocialServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SocialServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.SocialServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("SocialService");
    }
  }

  private static final class SocialServiceFileDescriptorSupplier
      extends SocialServiceBaseDescriptorSupplier {
    SocialServiceFileDescriptorSupplier() {}
  }

  private static final class SocialServiceMethodDescriptorSupplier
      extends SocialServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SocialServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (SocialServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SocialServiceFileDescriptorSupplier())
              .addMethod(getListSocialUsersMethod())
              .addMethod(getListFriendRequestsMethod())
              .addMethod(getCreateFriendRequestMethod())
              .addMethod(getAcceptFriendRequestMethod())
              .addMethod(getDeclineFriendRequestMethod())
              .addMethod(getCancelFriendRequestMethod())
              .addMethod(getRemoveFriendshipMethod())
              .build();
        }
      }
    }
    return result;
  }
}
