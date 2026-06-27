package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RealtimeServiceGrpc {

  private RealtimeServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.RealtimeService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest,
      com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> getSyncRealtimeSubscriptionsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SyncRealtimeSubscriptions",
      requestType = com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest.class,
      responseType = com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest,
      com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> getSyncRealtimeSubscriptionsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest, com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> getSyncRealtimeSubscriptionsMethod;
    if ((getSyncRealtimeSubscriptionsMethod = RealtimeServiceGrpc.getSyncRealtimeSubscriptionsMethod) == null) {
      synchronized (RealtimeServiceGrpc.class) {
        if ((getSyncRealtimeSubscriptionsMethod = RealtimeServiceGrpc.getSyncRealtimeSubscriptionsMethod) == null) {
          RealtimeServiceGrpc.getSyncRealtimeSubscriptionsMethod = getSyncRealtimeSubscriptionsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest, com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SyncRealtimeSubscriptions"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeServiceMethodDescriptorSupplier("SyncRealtimeSubscriptions"))
              .build();
        }
      }
    }
    return getSyncRealtimeSubscriptionsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AckRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> getAckRealtimeEventsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AckRealtimeEvents",
      requestType = com.sdkwork.communication.app.v3.AckRealtimeEventsRequest.class,
      responseType = com.sdkwork.communication.app.v3.AckRealtimeEventsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AckRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> getAckRealtimeEventsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AckRealtimeEventsRequest, com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> getAckRealtimeEventsMethod;
    if ((getAckRealtimeEventsMethod = RealtimeServiceGrpc.getAckRealtimeEventsMethod) == null) {
      synchronized (RealtimeServiceGrpc.class) {
        if ((getAckRealtimeEventsMethod = RealtimeServiceGrpc.getAckRealtimeEventsMethod) == null) {
          RealtimeServiceGrpc.getAckRealtimeEventsMethod = getAckRealtimeEventsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AckRealtimeEventsRequest, com.sdkwork.communication.app.v3.AckRealtimeEventsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AckRealtimeEvents"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AckRealtimeEventsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AckRealtimeEventsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeServiceMethodDescriptorSupplier("AckRealtimeEvents"))
              .build();
        }
      }
    }
    return getAckRealtimeEventsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> getListRealtimeEventsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListRealtimeEvents",
      requestType = com.sdkwork.communication.app.v3.ListRealtimeEventsRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListRealtimeEventsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> getListRealtimeEventsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListRealtimeEventsRequest, com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> getListRealtimeEventsMethod;
    if ((getListRealtimeEventsMethod = RealtimeServiceGrpc.getListRealtimeEventsMethod) == null) {
      synchronized (RealtimeServiceGrpc.class) {
        if ((getListRealtimeEventsMethod = RealtimeServiceGrpc.getListRealtimeEventsMethod) == null) {
          RealtimeServiceGrpc.getListRealtimeEventsMethod = getListRealtimeEventsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListRealtimeEventsRequest, com.sdkwork.communication.app.v3.ListRealtimeEventsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListRealtimeEvents"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListRealtimeEventsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListRealtimeEventsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeServiceMethodDescriptorSupplier("ListRealtimeEvents"))
              .build();
        }
      }
    }
    return getListRealtimeEventsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> getWatchRealtimeEventsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchRealtimeEvents",
      requestType = com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest.class,
      responseType = com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest,
      com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> getWatchRealtimeEventsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest, com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> getWatchRealtimeEventsMethod;
    if ((getWatchRealtimeEventsMethod = RealtimeServiceGrpc.getWatchRealtimeEventsMethod) == null) {
      synchronized (RealtimeServiceGrpc.class) {
        if ((getWatchRealtimeEventsMethod = RealtimeServiceGrpc.getWatchRealtimeEventsMethod) == null) {
          RealtimeServiceGrpc.getWatchRealtimeEventsMethod = getWatchRealtimeEventsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest, com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchRealtimeEvents"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeServiceMethodDescriptorSupplier("WatchRealtimeEvents"))
              .build();
        }
      }
    }
    return getWatchRealtimeEventsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RealtimeServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceStub>() {
        @java.lang.Override
        public RealtimeServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeServiceStub(channel, callOptions);
        }
      };
    return RealtimeServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RealtimeServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceBlockingV2Stub>() {
        @java.lang.Override
        public RealtimeServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RealtimeServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RealtimeServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceBlockingStub>() {
        @java.lang.Override
        public RealtimeServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeServiceBlockingStub(channel, callOptions);
        }
      };
    return RealtimeServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RealtimeServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeServiceFutureStub>() {
        @java.lang.Override
        public RealtimeServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeServiceFutureStub(channel, callOptions);
        }
      };
    return RealtimeServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void syncRealtimeSubscriptions(com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSyncRealtimeSubscriptionsMethod(), responseObserver);
    }

    /**
     */
    default void ackRealtimeEvents(com.sdkwork.communication.app.v3.AckRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAckRealtimeEventsMethod(), responseObserver);
    }

    /**
     */
    default void listRealtimeEvents(com.sdkwork.communication.app.v3.ListRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListRealtimeEventsMethod(), responseObserver);
    }

    /**
     */
    default void watchRealtimeEvents(com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchRealtimeEventsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RealtimeService.
   */
  public static abstract class RealtimeServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RealtimeServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RealtimeService.
   */
  public static final class RealtimeServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RealtimeServiceStub> {
    private RealtimeServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeServiceStub(channel, callOptions);
    }

    /**
     */
    public void syncRealtimeSubscriptions(com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSyncRealtimeSubscriptionsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void ackRealtimeEvents(com.sdkwork.communication.app.v3.AckRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAckRealtimeEventsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listRealtimeEvents(com.sdkwork.communication.app.v3.ListRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListRealtimeEventsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchRealtimeEvents(com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchRealtimeEventsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RealtimeService.
   */
  public static final class RealtimeServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RealtimeServiceBlockingV2Stub> {
    private RealtimeServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse syncRealtimeSubscriptions(com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSyncRealtimeSubscriptionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AckRealtimeEventsResponse ackRealtimeEvents(com.sdkwork.communication.app.v3.AckRealtimeEventsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAckRealtimeEventsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListRealtimeEventsResponse listRealtimeEvents(com.sdkwork.communication.app.v3.ListRealtimeEventsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListRealtimeEventsMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse>
        watchRealtimeEvents(com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchRealtimeEventsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RealtimeService.
   */
  public static final class RealtimeServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RealtimeServiceBlockingStub> {
    private RealtimeServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse syncRealtimeSubscriptions(com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSyncRealtimeSubscriptionsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AckRealtimeEventsResponse ackRealtimeEvents(com.sdkwork.communication.app.v3.AckRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAckRealtimeEventsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListRealtimeEventsResponse listRealtimeEvents(com.sdkwork.communication.app.v3.ListRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListRealtimeEventsMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse> watchRealtimeEvents(
        com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchRealtimeEventsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RealtimeService.
   */
  public static final class RealtimeServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RealtimeServiceFutureStub> {
    private RealtimeServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse> syncRealtimeSubscriptions(
        com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSyncRealtimeSubscriptionsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AckRealtimeEventsResponse> ackRealtimeEvents(
        com.sdkwork.communication.app.v3.AckRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAckRealtimeEventsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListRealtimeEventsResponse> listRealtimeEvents(
        com.sdkwork.communication.app.v3.ListRealtimeEventsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListRealtimeEventsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SYNC_REALTIME_SUBSCRIPTIONS = 0;
  private static final int METHODID_ACK_REALTIME_EVENTS = 1;
  private static final int METHODID_LIST_REALTIME_EVENTS = 2;
  private static final int METHODID_WATCH_REALTIME_EVENTS = 3;

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
        case METHODID_SYNC_REALTIME_SUBSCRIPTIONS:
          serviceImpl.syncRealtimeSubscriptions((com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse>) responseObserver);
          break;
        case METHODID_ACK_REALTIME_EVENTS:
          serviceImpl.ackRealtimeEvents((com.sdkwork.communication.app.v3.AckRealtimeEventsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AckRealtimeEventsResponse>) responseObserver);
          break;
        case METHODID_LIST_REALTIME_EVENTS:
          serviceImpl.listRealtimeEvents((com.sdkwork.communication.app.v3.ListRealtimeEventsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListRealtimeEventsResponse>) responseObserver);
          break;
        case METHODID_WATCH_REALTIME_EVENTS:
          serviceImpl.watchRealtimeEvents((com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse>) responseObserver);
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
          getSyncRealtimeSubscriptionsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsRequest,
              com.sdkwork.communication.app.v3.SyncRealtimeSubscriptionsResponse>(
                service, METHODID_SYNC_REALTIME_SUBSCRIPTIONS)))
        .addMethod(
          getAckRealtimeEventsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AckRealtimeEventsRequest,
              com.sdkwork.communication.app.v3.AckRealtimeEventsResponse>(
                service, METHODID_ACK_REALTIME_EVENTS)))
        .addMethod(
          getListRealtimeEventsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListRealtimeEventsRequest,
              com.sdkwork.communication.app.v3.ListRealtimeEventsResponse>(
                service, METHODID_LIST_REALTIME_EVENTS)))
        .addMethod(
          getWatchRealtimeEventsMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.WatchRealtimeEventsRequest,
              com.sdkwork.communication.app.v3.WatchRealtimeEventsResponse>(
                service, METHODID_WATCH_REALTIME_EVENTS)))
        .build();
  }

  private static abstract class RealtimeServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RealtimeServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.RealtimeServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RealtimeService");
    }
  }

  private static final class RealtimeServiceFileDescriptorSupplier
      extends RealtimeServiceBaseDescriptorSupplier {
    RealtimeServiceFileDescriptorSupplier() {}
  }

  private static final class RealtimeServiceMethodDescriptorSupplier
      extends RealtimeServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RealtimeServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RealtimeServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RealtimeServiceFileDescriptorSupplier())
              .addMethod(getSyncRealtimeSubscriptionsMethod())
              .addMethod(getAckRealtimeEventsMethod())
              .addMethod(getListRealtimeEventsMethod())
              .addMethod(getWatchRealtimeEventsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
