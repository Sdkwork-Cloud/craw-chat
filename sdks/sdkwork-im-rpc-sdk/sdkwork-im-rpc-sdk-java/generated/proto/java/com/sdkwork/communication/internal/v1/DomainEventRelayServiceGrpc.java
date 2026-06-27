package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class DomainEventRelayServiceGrpc {

  private DomainEventRelayServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.DomainEventRelayService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.PublishDomainEventRequest,
      com.sdkwork.communication.internal.v1.PublishDomainEventResponse> getPublishDomainEventMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PublishDomainEvent",
      requestType = com.sdkwork.communication.internal.v1.PublishDomainEventRequest.class,
      responseType = com.sdkwork.communication.internal.v1.PublishDomainEventResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.PublishDomainEventRequest,
      com.sdkwork.communication.internal.v1.PublishDomainEventResponse> getPublishDomainEventMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.PublishDomainEventRequest, com.sdkwork.communication.internal.v1.PublishDomainEventResponse> getPublishDomainEventMethod;
    if ((getPublishDomainEventMethod = DomainEventRelayServiceGrpc.getPublishDomainEventMethod) == null) {
      synchronized (DomainEventRelayServiceGrpc.class) {
        if ((getPublishDomainEventMethod = DomainEventRelayServiceGrpc.getPublishDomainEventMethod) == null) {
          DomainEventRelayServiceGrpc.getPublishDomainEventMethod = getPublishDomainEventMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.PublishDomainEventRequest, com.sdkwork.communication.internal.v1.PublishDomainEventResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PublishDomainEvent"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.PublishDomainEventRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.PublishDomainEventResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DomainEventRelayServiceMethodDescriptorSupplier("PublishDomainEvent"))
              .build();
        }
      }
    }
    return getPublishDomainEventMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.AckDomainEventRequest,
      com.sdkwork.communication.internal.v1.AckDomainEventResponse> getAckDomainEventMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AckDomainEvent",
      requestType = com.sdkwork.communication.internal.v1.AckDomainEventRequest.class,
      responseType = com.sdkwork.communication.internal.v1.AckDomainEventResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.AckDomainEventRequest,
      com.sdkwork.communication.internal.v1.AckDomainEventResponse> getAckDomainEventMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.AckDomainEventRequest, com.sdkwork.communication.internal.v1.AckDomainEventResponse> getAckDomainEventMethod;
    if ((getAckDomainEventMethod = DomainEventRelayServiceGrpc.getAckDomainEventMethod) == null) {
      synchronized (DomainEventRelayServiceGrpc.class) {
        if ((getAckDomainEventMethod = DomainEventRelayServiceGrpc.getAckDomainEventMethod) == null) {
          DomainEventRelayServiceGrpc.getAckDomainEventMethod = getAckDomainEventMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.AckDomainEventRequest, com.sdkwork.communication.internal.v1.AckDomainEventResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AckDomainEvent"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.AckDomainEventRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.AckDomainEventResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DomainEventRelayServiceMethodDescriptorSupplier("AckDomainEvent"))
              .build();
        }
      }
    }
    return getAckDomainEventMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchDomainEventsRequest,
      com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> getWatchDomainEventsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchDomainEvents",
      requestType = com.sdkwork.communication.internal.v1.WatchDomainEventsRequest.class,
      responseType = com.sdkwork.communication.internal.v1.WatchDomainEventsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchDomainEventsRequest,
      com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> getWatchDomainEventsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchDomainEventsRequest, com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> getWatchDomainEventsMethod;
    if ((getWatchDomainEventsMethod = DomainEventRelayServiceGrpc.getWatchDomainEventsMethod) == null) {
      synchronized (DomainEventRelayServiceGrpc.class) {
        if ((getWatchDomainEventsMethod = DomainEventRelayServiceGrpc.getWatchDomainEventsMethod) == null) {
          DomainEventRelayServiceGrpc.getWatchDomainEventsMethod = getWatchDomainEventsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.WatchDomainEventsRequest, com.sdkwork.communication.internal.v1.WatchDomainEventsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchDomainEvents"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.WatchDomainEventsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.WatchDomainEventsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new DomainEventRelayServiceMethodDescriptorSupplier("WatchDomainEvents"))
              .build();
        }
      }
    }
    return getWatchDomainEventsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static DomainEventRelayServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceStub>() {
        @java.lang.Override
        public DomainEventRelayServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DomainEventRelayServiceStub(channel, callOptions);
        }
      };
    return DomainEventRelayServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static DomainEventRelayServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceBlockingV2Stub>() {
        @java.lang.Override
        public DomainEventRelayServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DomainEventRelayServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return DomainEventRelayServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static DomainEventRelayServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceBlockingStub>() {
        @java.lang.Override
        public DomainEventRelayServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DomainEventRelayServiceBlockingStub(channel, callOptions);
        }
      };
    return DomainEventRelayServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static DomainEventRelayServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<DomainEventRelayServiceFutureStub>() {
        @java.lang.Override
        public DomainEventRelayServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new DomainEventRelayServiceFutureStub(channel, callOptions);
        }
      };
    return DomainEventRelayServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void publishDomainEvent(com.sdkwork.communication.internal.v1.PublishDomainEventRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.PublishDomainEventResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPublishDomainEventMethod(), responseObserver);
    }

    /**
     */
    default void ackDomainEvent(com.sdkwork.communication.internal.v1.AckDomainEventRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.AckDomainEventResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAckDomainEventMethod(), responseObserver);
    }

    /**
     */
    default void watchDomainEvents(com.sdkwork.communication.internal.v1.WatchDomainEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchDomainEventsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service DomainEventRelayService.
   */
  public static abstract class DomainEventRelayServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return DomainEventRelayServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service DomainEventRelayService.
   */
  public static final class DomainEventRelayServiceStub
      extends io.grpc.stub.AbstractAsyncStub<DomainEventRelayServiceStub> {
    private DomainEventRelayServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DomainEventRelayServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DomainEventRelayServiceStub(channel, callOptions);
    }

    /**
     */
    public void publishDomainEvent(com.sdkwork.communication.internal.v1.PublishDomainEventRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.PublishDomainEventResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPublishDomainEventMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void ackDomainEvent(com.sdkwork.communication.internal.v1.AckDomainEventRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.AckDomainEventResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAckDomainEventMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchDomainEvents(com.sdkwork.communication.internal.v1.WatchDomainEventsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchDomainEventsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service DomainEventRelayService.
   */
  public static final class DomainEventRelayServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<DomainEventRelayServiceBlockingV2Stub> {
    private DomainEventRelayServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DomainEventRelayServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DomainEventRelayServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.PublishDomainEventResponse publishDomainEvent(com.sdkwork.communication.internal.v1.PublishDomainEventRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPublishDomainEventMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.AckDomainEventResponse ackDomainEvent(com.sdkwork.communication.internal.v1.AckDomainEventRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAckDomainEventMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.internal.v1.WatchDomainEventsResponse>
        watchDomainEvents(com.sdkwork.communication.internal.v1.WatchDomainEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchDomainEventsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service DomainEventRelayService.
   */
  public static final class DomainEventRelayServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<DomainEventRelayServiceBlockingStub> {
    private DomainEventRelayServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DomainEventRelayServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DomainEventRelayServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.PublishDomainEventResponse publishDomainEvent(com.sdkwork.communication.internal.v1.PublishDomainEventRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPublishDomainEventMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.AckDomainEventResponse ackDomainEvent(com.sdkwork.communication.internal.v1.AckDomainEventRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAckDomainEventMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.internal.v1.WatchDomainEventsResponse> watchDomainEvents(
        com.sdkwork.communication.internal.v1.WatchDomainEventsRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchDomainEventsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service DomainEventRelayService.
   */
  public static final class DomainEventRelayServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<DomainEventRelayServiceFutureStub> {
    private DomainEventRelayServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected DomainEventRelayServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new DomainEventRelayServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.PublishDomainEventResponse> publishDomainEvent(
        com.sdkwork.communication.internal.v1.PublishDomainEventRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPublishDomainEventMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.AckDomainEventResponse> ackDomainEvent(
        com.sdkwork.communication.internal.v1.AckDomainEventRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAckDomainEventMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_PUBLISH_DOMAIN_EVENT = 0;
  private static final int METHODID_ACK_DOMAIN_EVENT = 1;
  private static final int METHODID_WATCH_DOMAIN_EVENTS = 2;

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
        case METHODID_PUBLISH_DOMAIN_EVENT:
          serviceImpl.publishDomainEvent((com.sdkwork.communication.internal.v1.PublishDomainEventRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.PublishDomainEventResponse>) responseObserver);
          break;
        case METHODID_ACK_DOMAIN_EVENT:
          serviceImpl.ackDomainEvent((com.sdkwork.communication.internal.v1.AckDomainEventRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.AckDomainEventResponse>) responseObserver);
          break;
        case METHODID_WATCH_DOMAIN_EVENTS:
          serviceImpl.watchDomainEvents((com.sdkwork.communication.internal.v1.WatchDomainEventsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchDomainEventsResponse>) responseObserver);
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
          getPublishDomainEventMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.PublishDomainEventRequest,
              com.sdkwork.communication.internal.v1.PublishDomainEventResponse>(
                service, METHODID_PUBLISH_DOMAIN_EVENT)))
        .addMethod(
          getAckDomainEventMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.AckDomainEventRequest,
              com.sdkwork.communication.internal.v1.AckDomainEventResponse>(
                service, METHODID_ACK_DOMAIN_EVENT)))
        .addMethod(
          getWatchDomainEventsMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.WatchDomainEventsRequest,
              com.sdkwork.communication.internal.v1.WatchDomainEventsResponse>(
                service, METHODID_WATCH_DOMAIN_EVENTS)))
        .build();
  }

  private static abstract class DomainEventRelayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    DomainEventRelayServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.DistributedRuntimeService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("DomainEventRelayService");
    }
  }

  private static final class DomainEventRelayServiceFileDescriptorSupplier
      extends DomainEventRelayServiceBaseDescriptorSupplier {
    DomainEventRelayServiceFileDescriptorSupplier() {}
  }

  private static final class DomainEventRelayServiceMethodDescriptorSupplier
      extends DomainEventRelayServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    DomainEventRelayServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (DomainEventRelayServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new DomainEventRelayServiceFileDescriptorSupplier())
              .addMethod(getPublishDomainEventMethod())
              .addMethod(getAckDomainEventMethod())
              .addMethod(getWatchDomainEventsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
