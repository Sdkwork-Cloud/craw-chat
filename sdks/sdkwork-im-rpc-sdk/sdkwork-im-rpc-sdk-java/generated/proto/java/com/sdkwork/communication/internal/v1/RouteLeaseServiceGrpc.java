package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RouteLeaseServiceGrpc {

  private RouteLeaseServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.RouteLeaseService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> getClaimRouteLeaseMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClaimRouteLease",
      requestType = com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest.class,
      responseType = com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> getClaimRouteLeaseMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest, com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> getClaimRouteLeaseMethod;
    if ((getClaimRouteLeaseMethod = RouteLeaseServiceGrpc.getClaimRouteLeaseMethod) == null) {
      synchronized (RouteLeaseServiceGrpc.class) {
        if ((getClaimRouteLeaseMethod = RouteLeaseServiceGrpc.getClaimRouteLeaseMethod) == null) {
          RouteLeaseServiceGrpc.getClaimRouteLeaseMethod = getClaimRouteLeaseMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest, com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClaimRouteLease"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RouteLeaseServiceMethodDescriptorSupplier("ClaimRouteLease"))
              .build();
        }
      }
    }
    return getClaimRouteLeaseMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> getRenewRouteLeaseMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RenewRouteLease",
      requestType = com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest.class,
      responseType = com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> getRenewRouteLeaseMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest, com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> getRenewRouteLeaseMethod;
    if ((getRenewRouteLeaseMethod = RouteLeaseServiceGrpc.getRenewRouteLeaseMethod) == null) {
      synchronized (RouteLeaseServiceGrpc.class) {
        if ((getRenewRouteLeaseMethod = RouteLeaseServiceGrpc.getRenewRouteLeaseMethod) == null) {
          RouteLeaseServiceGrpc.getRenewRouteLeaseMethod = getRenewRouteLeaseMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest, com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RenewRouteLease"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RouteLeaseServiceMethodDescriptorSupplier("RenewRouteLease"))
              .build();
        }
      }
    }
    return getRenewRouteLeaseMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> getReleaseRouteLeaseMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ReleaseRouteLease",
      requestType = com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest.class,
      responseType = com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest,
      com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> getReleaseRouteLeaseMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest, com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> getReleaseRouteLeaseMethod;
    if ((getReleaseRouteLeaseMethod = RouteLeaseServiceGrpc.getReleaseRouteLeaseMethod) == null) {
      synchronized (RouteLeaseServiceGrpc.class) {
        if ((getReleaseRouteLeaseMethod = RouteLeaseServiceGrpc.getReleaseRouteLeaseMethod) == null) {
          RouteLeaseServiceGrpc.getReleaseRouteLeaseMethod = getReleaseRouteLeaseMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest, com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ReleaseRouteLease"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RouteLeaseServiceMethodDescriptorSupplier("ReleaseRouteLease"))
              .build();
        }
      }
    }
    return getReleaseRouteLeaseMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRouteLeasesRequest,
      com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> getListRouteLeasesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListRouteLeases",
      requestType = com.sdkwork.communication.internal.v1.ListRouteLeasesRequest.class,
      responseType = com.sdkwork.communication.internal.v1.ListRouteLeasesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRouteLeasesRequest,
      com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> getListRouteLeasesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRouteLeasesRequest, com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> getListRouteLeasesMethod;
    if ((getListRouteLeasesMethod = RouteLeaseServiceGrpc.getListRouteLeasesMethod) == null) {
      synchronized (RouteLeaseServiceGrpc.class) {
        if ((getListRouteLeasesMethod = RouteLeaseServiceGrpc.getListRouteLeasesMethod) == null) {
          RouteLeaseServiceGrpc.getListRouteLeasesMethod = getListRouteLeasesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.ListRouteLeasesRequest, com.sdkwork.communication.internal.v1.ListRouteLeasesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListRouteLeases"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ListRouteLeasesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ListRouteLeasesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RouteLeaseServiceMethodDescriptorSupplier("ListRouteLeases"))
              .build();
        }
      }
    }
    return getListRouteLeasesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RouteLeaseServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceStub>() {
        @java.lang.Override
        public RouteLeaseServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouteLeaseServiceStub(channel, callOptions);
        }
      };
    return RouteLeaseServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RouteLeaseServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceBlockingV2Stub>() {
        @java.lang.Override
        public RouteLeaseServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouteLeaseServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RouteLeaseServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RouteLeaseServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceBlockingStub>() {
        @java.lang.Override
        public RouteLeaseServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouteLeaseServiceBlockingStub(channel, callOptions);
        }
      };
    return RouteLeaseServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RouteLeaseServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RouteLeaseServiceFutureStub>() {
        @java.lang.Override
        public RouteLeaseServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RouteLeaseServiceFutureStub(channel, callOptions);
        }
      };
    return RouteLeaseServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void claimRouteLease(com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClaimRouteLeaseMethod(), responseObserver);
    }

    /**
     */
    default void renewRouteLease(com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRenewRouteLeaseMethod(), responseObserver);
    }

    /**
     */
    default void releaseRouteLease(com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getReleaseRouteLeaseMethod(), responseObserver);
    }

    /**
     */
    default void listRouteLeases(com.sdkwork.communication.internal.v1.ListRouteLeasesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListRouteLeasesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RouteLeaseService.
   */
  public static abstract class RouteLeaseServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RouteLeaseServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RouteLeaseService.
   */
  public static final class RouteLeaseServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RouteLeaseServiceStub> {
    private RouteLeaseServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouteLeaseServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouteLeaseServiceStub(channel, callOptions);
    }

    /**
     */
    public void claimRouteLease(com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getClaimRouteLeaseMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void renewRouteLease(com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRenewRouteLeaseMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void releaseRouteLease(com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getReleaseRouteLeaseMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listRouteLeases(com.sdkwork.communication.internal.v1.ListRouteLeasesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListRouteLeasesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RouteLeaseService.
   */
  public static final class RouteLeaseServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RouteLeaseServiceBlockingV2Stub> {
    private RouteLeaseServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouteLeaseServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouteLeaseServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse claimRouteLease(com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getClaimRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse renewRouteLease(com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRenewRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse releaseRouteLease(com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getReleaseRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ListRouteLeasesResponse listRouteLeases(com.sdkwork.communication.internal.v1.ListRouteLeasesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListRouteLeasesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RouteLeaseService.
   */
  public static final class RouteLeaseServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RouteLeaseServiceBlockingStub> {
    private RouteLeaseServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouteLeaseServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouteLeaseServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse claimRouteLease(com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getClaimRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse renewRouteLease(com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRenewRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse releaseRouteLease(com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getReleaseRouteLeaseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ListRouteLeasesResponse listRouteLeases(com.sdkwork.communication.internal.v1.ListRouteLeasesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListRouteLeasesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RouteLeaseService.
   */
  public static final class RouteLeaseServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RouteLeaseServiceFutureStub> {
    private RouteLeaseServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RouteLeaseServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RouteLeaseServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse> claimRouteLease(
        com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getClaimRouteLeaseMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse> renewRouteLease(
        com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRenewRouteLeaseMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse> releaseRouteLease(
        com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getReleaseRouteLeaseMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.ListRouteLeasesResponse> listRouteLeases(
        com.sdkwork.communication.internal.v1.ListRouteLeasesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListRouteLeasesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CLAIM_ROUTE_LEASE = 0;
  private static final int METHODID_RENEW_ROUTE_LEASE = 1;
  private static final int METHODID_RELEASE_ROUTE_LEASE = 2;
  private static final int METHODID_LIST_ROUTE_LEASES = 3;

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
        case METHODID_CLAIM_ROUTE_LEASE:
          serviceImpl.claimRouteLease((com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse>) responseObserver);
          break;
        case METHODID_RENEW_ROUTE_LEASE:
          serviceImpl.renewRouteLease((com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse>) responseObserver);
          break;
        case METHODID_RELEASE_ROUTE_LEASE:
          serviceImpl.releaseRouteLease((com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse>) responseObserver);
          break;
        case METHODID_LIST_ROUTE_LEASES:
          serviceImpl.listRouteLeases((com.sdkwork.communication.internal.v1.ListRouteLeasesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRouteLeasesResponse>) responseObserver);
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
          getClaimRouteLeaseMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.ClaimRouteLeaseRequest,
              com.sdkwork.communication.internal.v1.ClaimRouteLeaseResponse>(
                service, METHODID_CLAIM_ROUTE_LEASE)))
        .addMethod(
          getRenewRouteLeaseMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.RenewRouteLeaseRequest,
              com.sdkwork.communication.internal.v1.RenewRouteLeaseResponse>(
                service, METHODID_RENEW_ROUTE_LEASE)))
        .addMethod(
          getReleaseRouteLeaseMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.ReleaseRouteLeaseRequest,
              com.sdkwork.communication.internal.v1.ReleaseRouteLeaseResponse>(
                service, METHODID_RELEASE_ROUTE_LEASE)))
        .addMethod(
          getListRouteLeasesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.ListRouteLeasesRequest,
              com.sdkwork.communication.internal.v1.ListRouteLeasesResponse>(
                service, METHODID_LIST_ROUTE_LEASES)))
        .build();
  }

  private static abstract class RouteLeaseServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RouteLeaseServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.DistributedRuntimeService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RouteLeaseService");
    }
  }

  private static final class RouteLeaseServiceFileDescriptorSupplier
      extends RouteLeaseServiceBaseDescriptorSupplier {
    RouteLeaseServiceFileDescriptorSupplier() {}
  }

  private static final class RouteLeaseServiceMethodDescriptorSupplier
      extends RouteLeaseServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RouteLeaseServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RouteLeaseServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RouteLeaseServiceFileDescriptorSupplier())
              .addMethod(getClaimRouteLeaseMethod())
              .addMethod(getRenewRouteLeaseMethod())
              .addMethod(getReleaseRouteLeaseMethod())
              .addMethod(getListRouteLeasesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
