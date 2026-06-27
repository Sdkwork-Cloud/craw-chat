package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class PresenceServiceGrpc {

  private PresenceServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.PresenceService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest,
      com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> getCreatePresenceHeartbeatMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreatePresenceHeartbeat",
      requestType = com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest,
      com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> getCreatePresenceHeartbeatMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest, com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> getCreatePresenceHeartbeatMethod;
    if ((getCreatePresenceHeartbeatMethod = PresenceServiceGrpc.getCreatePresenceHeartbeatMethod) == null) {
      synchronized (PresenceServiceGrpc.class) {
        if ((getCreatePresenceHeartbeatMethod = PresenceServiceGrpc.getCreatePresenceHeartbeatMethod) == null) {
          PresenceServiceGrpc.getCreatePresenceHeartbeatMethod = getCreatePresenceHeartbeatMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest, com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreatePresenceHeartbeat"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PresenceServiceMethodDescriptorSupplier("CreatePresenceHeartbeat"))
              .build();
        }
      }
    }
    return getCreatePresenceHeartbeatMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest,
      com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> getRetrieveMyPresenceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveMyPresence",
      requestType = com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest,
      com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> getRetrieveMyPresenceMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest, com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> getRetrieveMyPresenceMethod;
    if ((getRetrieveMyPresenceMethod = PresenceServiceGrpc.getRetrieveMyPresenceMethod) == null) {
      synchronized (PresenceServiceGrpc.class) {
        if ((getRetrieveMyPresenceMethod = PresenceServiceGrpc.getRetrieveMyPresenceMethod) == null) {
          PresenceServiceGrpc.getRetrieveMyPresenceMethod = getRetrieveMyPresenceMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest, com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveMyPresence"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PresenceServiceMethodDescriptorSupplier("RetrieveMyPresence"))
              .build();
        }
      }
    }
    return getRetrieveMyPresenceMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchPresenceRequest,
      com.sdkwork.communication.app.v3.WatchPresenceResponse> getWatchPresenceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchPresence",
      requestType = com.sdkwork.communication.app.v3.WatchPresenceRequest.class,
      responseType = com.sdkwork.communication.app.v3.WatchPresenceResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchPresenceRequest,
      com.sdkwork.communication.app.v3.WatchPresenceResponse> getWatchPresenceMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchPresenceRequest, com.sdkwork.communication.app.v3.WatchPresenceResponse> getWatchPresenceMethod;
    if ((getWatchPresenceMethod = PresenceServiceGrpc.getWatchPresenceMethod) == null) {
      synchronized (PresenceServiceGrpc.class) {
        if ((getWatchPresenceMethod = PresenceServiceGrpc.getWatchPresenceMethod) == null) {
          PresenceServiceGrpc.getWatchPresenceMethod = getWatchPresenceMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.WatchPresenceRequest, com.sdkwork.communication.app.v3.WatchPresenceResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchPresence"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchPresenceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchPresenceResponse.getDefaultInstance()))
              .setSchemaDescriptor(new PresenceServiceMethodDescriptorSupplier("WatchPresence"))
              .build();
        }
      }
    }
    return getWatchPresenceMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static PresenceServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PresenceServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PresenceServiceStub>() {
        @java.lang.Override
        public PresenceServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PresenceServiceStub(channel, callOptions);
        }
      };
    return PresenceServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static PresenceServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PresenceServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PresenceServiceBlockingV2Stub>() {
        @java.lang.Override
        public PresenceServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PresenceServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return PresenceServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static PresenceServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PresenceServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PresenceServiceBlockingStub>() {
        @java.lang.Override
        public PresenceServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PresenceServiceBlockingStub(channel, callOptions);
        }
      };
    return PresenceServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static PresenceServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<PresenceServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<PresenceServiceFutureStub>() {
        @java.lang.Override
        public PresenceServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new PresenceServiceFutureStub(channel, callOptions);
        }
      };
    return PresenceServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createPresenceHeartbeat(com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreatePresenceHeartbeatMethod(), responseObserver);
    }

    /**
     */
    default void retrieveMyPresence(com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveMyPresenceMethod(), responseObserver);
    }

    /**
     */
    default void watchPresence(com.sdkwork.communication.app.v3.WatchPresenceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchPresenceResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchPresenceMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service PresenceService.
   */
  public static abstract class PresenceServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return PresenceServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service PresenceService.
   */
  public static final class PresenceServiceStub
      extends io.grpc.stub.AbstractAsyncStub<PresenceServiceStub> {
    private PresenceServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PresenceServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PresenceServiceStub(channel, callOptions);
    }

    /**
     */
    public void createPresenceHeartbeat(com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreatePresenceHeartbeatMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveMyPresence(com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveMyPresenceMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchPresence(com.sdkwork.communication.app.v3.WatchPresenceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchPresenceResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchPresenceMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service PresenceService.
   */
  public static final class PresenceServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<PresenceServiceBlockingV2Stub> {
    private PresenceServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PresenceServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PresenceServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse createPresenceHeartbeat(com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreatePresenceHeartbeatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse retrieveMyPresence(com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveMyPresenceMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.app.v3.WatchPresenceResponse>
        watchPresence(com.sdkwork.communication.app.v3.WatchPresenceRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchPresenceMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service PresenceService.
   */
  public static final class PresenceServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<PresenceServiceBlockingStub> {
    private PresenceServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PresenceServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PresenceServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse createPresenceHeartbeat(com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreatePresenceHeartbeatMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse retrieveMyPresence(com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveMyPresenceMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.app.v3.WatchPresenceResponse> watchPresence(
        com.sdkwork.communication.app.v3.WatchPresenceRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchPresenceMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service PresenceService.
   */
  public static final class PresenceServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<PresenceServiceFutureStub> {
    private PresenceServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected PresenceServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new PresenceServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse> createPresenceHeartbeat(
        com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreatePresenceHeartbeatMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse> retrieveMyPresence(
        com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveMyPresenceMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_PRESENCE_HEARTBEAT = 0;
  private static final int METHODID_RETRIEVE_MY_PRESENCE = 1;
  private static final int METHODID_WATCH_PRESENCE = 2;

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
        case METHODID_CREATE_PRESENCE_HEARTBEAT:
          serviceImpl.createPresenceHeartbeat((com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_MY_PRESENCE:
          serviceImpl.retrieveMyPresence((com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse>) responseObserver);
          break;
        case METHODID_WATCH_PRESENCE:
          serviceImpl.watchPresence((com.sdkwork.communication.app.v3.WatchPresenceRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchPresenceResponse>) responseObserver);
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
          getCreatePresenceHeartbeatMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreatePresenceHeartbeatRequest,
              com.sdkwork.communication.app.v3.CreatePresenceHeartbeatResponse>(
                service, METHODID_CREATE_PRESENCE_HEARTBEAT)))
        .addMethod(
          getRetrieveMyPresenceMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveMyPresenceRequest,
              com.sdkwork.communication.app.v3.RetrieveMyPresenceResponse>(
                service, METHODID_RETRIEVE_MY_PRESENCE)))
        .addMethod(
          getWatchPresenceMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.WatchPresenceRequest,
              com.sdkwork.communication.app.v3.WatchPresenceResponse>(
                service, METHODID_WATCH_PRESENCE)))
        .build();
  }

  private static abstract class PresenceServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    PresenceServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.RealtimeServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("PresenceService");
    }
  }

  private static final class PresenceServiceFileDescriptorSupplier
      extends PresenceServiceBaseDescriptorSupplier {
    PresenceServiceFileDescriptorSupplier() {}
  }

  private static final class PresenceServiceMethodDescriptorSupplier
      extends PresenceServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    PresenceServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (PresenceServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new PresenceServiceFileDescriptorSupplier())
              .addMethod(getCreatePresenceHeartbeatMethod())
              .addMethod(getRetrieveMyPresenceMethod())
              .addMethod(getWatchPresenceMethod())
              .build();
        }
      }
    }
    return result;
  }
}
