package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RuntimeTopologyServiceGrpc {

  private RuntimeTopologyServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.RuntimeTopologyService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest,
      com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> getRetrieveRuntimeTopologyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveRuntimeTopology",
      requestType = com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest.class,
      responseType = com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest,
      com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> getRetrieveRuntimeTopologyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest, com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> getRetrieveRuntimeTopologyMethod;
    if ((getRetrieveRuntimeTopologyMethod = RuntimeTopologyServiceGrpc.getRetrieveRuntimeTopologyMethod) == null) {
      synchronized (RuntimeTopologyServiceGrpc.class) {
        if ((getRetrieveRuntimeTopologyMethod = RuntimeTopologyServiceGrpc.getRetrieveRuntimeTopologyMethod) == null) {
          RuntimeTopologyServiceGrpc.getRetrieveRuntimeTopologyMethod = getRetrieveRuntimeTopologyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest, com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveRuntimeTopology"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RuntimeTopologyServiceMethodDescriptorSupplier("RetrieveRuntimeTopology"))
              .build();
        }
      }
    }
    return getRetrieveRuntimeTopologyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest,
      com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> getListRuntimeCapabilitiesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListRuntimeCapabilities",
      requestType = com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest.class,
      responseType = com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest,
      com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> getListRuntimeCapabilitiesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest, com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> getListRuntimeCapabilitiesMethod;
    if ((getListRuntimeCapabilitiesMethod = RuntimeTopologyServiceGrpc.getListRuntimeCapabilitiesMethod) == null) {
      synchronized (RuntimeTopologyServiceGrpc.class) {
        if ((getListRuntimeCapabilitiesMethod = RuntimeTopologyServiceGrpc.getListRuntimeCapabilitiesMethod) == null) {
          RuntimeTopologyServiceGrpc.getListRuntimeCapabilitiesMethod = getListRuntimeCapabilitiesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest, com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListRuntimeCapabilities"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RuntimeTopologyServiceMethodDescriptorSupplier("ListRuntimeCapabilities"))
              .build();
        }
      }
    }
    return getListRuntimeCapabilitiesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest,
      com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> getWatchRuntimeTopologyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchRuntimeTopology",
      requestType = com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest.class,
      responseType = com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest,
      com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> getWatchRuntimeTopologyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest, com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> getWatchRuntimeTopologyMethod;
    if ((getWatchRuntimeTopologyMethod = RuntimeTopologyServiceGrpc.getWatchRuntimeTopologyMethod) == null) {
      synchronized (RuntimeTopologyServiceGrpc.class) {
        if ((getWatchRuntimeTopologyMethod = RuntimeTopologyServiceGrpc.getWatchRuntimeTopologyMethod) == null) {
          RuntimeTopologyServiceGrpc.getWatchRuntimeTopologyMethod = getWatchRuntimeTopologyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest, com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchRuntimeTopology"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RuntimeTopologyServiceMethodDescriptorSupplier("WatchRuntimeTopology"))
              .build();
        }
      }
    }
    return getWatchRuntimeTopologyMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RuntimeTopologyServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceStub>() {
        @java.lang.Override
        public RuntimeTopologyServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RuntimeTopologyServiceStub(channel, callOptions);
        }
      };
    return RuntimeTopologyServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RuntimeTopologyServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceBlockingV2Stub>() {
        @java.lang.Override
        public RuntimeTopologyServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RuntimeTopologyServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RuntimeTopologyServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RuntimeTopologyServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceBlockingStub>() {
        @java.lang.Override
        public RuntimeTopologyServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RuntimeTopologyServiceBlockingStub(channel, callOptions);
        }
      };
    return RuntimeTopologyServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RuntimeTopologyServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RuntimeTopologyServiceFutureStub>() {
        @java.lang.Override
        public RuntimeTopologyServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RuntimeTopologyServiceFutureStub(channel, callOptions);
        }
      };
    return RuntimeTopologyServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void retrieveRuntimeTopology(com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveRuntimeTopologyMethod(), responseObserver);
    }

    /**
     */
    default void listRuntimeCapabilities(com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListRuntimeCapabilitiesMethod(), responseObserver);
    }

    /**
     */
    default void watchRuntimeTopology(com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchRuntimeTopologyMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RuntimeTopologyService.
   */
  public static abstract class RuntimeTopologyServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RuntimeTopologyServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RuntimeTopologyService.
   */
  public static final class RuntimeTopologyServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RuntimeTopologyServiceStub> {
    private RuntimeTopologyServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RuntimeTopologyServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RuntimeTopologyServiceStub(channel, callOptions);
    }

    /**
     */
    public void retrieveRuntimeTopology(com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveRuntimeTopologyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listRuntimeCapabilities(com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListRuntimeCapabilitiesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchRuntimeTopology(com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchRuntimeTopologyMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RuntimeTopologyService.
   */
  public static final class RuntimeTopologyServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RuntimeTopologyServiceBlockingV2Stub> {
    private RuntimeTopologyServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RuntimeTopologyServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RuntimeTopologyServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse retrieveRuntimeTopology(com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveRuntimeTopologyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse listRuntimeCapabilities(com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListRuntimeCapabilitiesMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse>
        watchRuntimeTopology(com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchRuntimeTopologyMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RuntimeTopologyService.
   */
  public static final class RuntimeTopologyServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RuntimeTopologyServiceBlockingStub> {
    private RuntimeTopologyServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RuntimeTopologyServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RuntimeTopologyServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse retrieveRuntimeTopology(com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveRuntimeTopologyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse listRuntimeCapabilities(com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListRuntimeCapabilitiesMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse> watchRuntimeTopology(
        com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchRuntimeTopologyMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RuntimeTopologyService.
   */
  public static final class RuntimeTopologyServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RuntimeTopologyServiceFutureStub> {
    private RuntimeTopologyServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RuntimeTopologyServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RuntimeTopologyServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse> retrieveRuntimeTopology(
        com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveRuntimeTopologyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse> listRuntimeCapabilities(
        com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListRuntimeCapabilitiesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_RETRIEVE_RUNTIME_TOPOLOGY = 0;
  private static final int METHODID_LIST_RUNTIME_CAPABILITIES = 1;
  private static final int METHODID_WATCH_RUNTIME_TOPOLOGY = 2;

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
        case METHODID_RETRIEVE_RUNTIME_TOPOLOGY:
          serviceImpl.retrieveRuntimeTopology((com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse>) responseObserver);
          break;
        case METHODID_LIST_RUNTIME_CAPABILITIES:
          serviceImpl.listRuntimeCapabilities((com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse>) responseObserver);
          break;
        case METHODID_WATCH_RUNTIME_TOPOLOGY:
          serviceImpl.watchRuntimeTopology((com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse>) responseObserver);
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
          getRetrieveRuntimeTopologyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyRequest,
              com.sdkwork.communication.internal.v1.RetrieveRuntimeTopologyResponse>(
                service, METHODID_RETRIEVE_RUNTIME_TOPOLOGY)))
        .addMethod(
          getListRuntimeCapabilitiesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesRequest,
              com.sdkwork.communication.internal.v1.ListRuntimeCapabilitiesResponse>(
                service, METHODID_LIST_RUNTIME_CAPABILITIES)))
        .addMethod(
          getWatchRuntimeTopologyMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.WatchRuntimeTopologyRequest,
              com.sdkwork.communication.internal.v1.WatchRuntimeTopologyResponse>(
                service, METHODID_WATCH_RUNTIME_TOPOLOGY)))
        .build();
  }

  private static abstract class RuntimeTopologyServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RuntimeTopologyServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.DistributedRuntimeService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RuntimeTopologyService");
    }
  }

  private static final class RuntimeTopologyServiceFileDescriptorSupplier
      extends RuntimeTopologyServiceBaseDescriptorSupplier {
    RuntimeTopologyServiceFileDescriptorSupplier() {}
  }

  private static final class RuntimeTopologyServiceMethodDescriptorSupplier
      extends RuntimeTopologyServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RuntimeTopologyServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RuntimeTopologyServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RuntimeTopologyServiceFileDescriptorSupplier())
              .addMethod(getRetrieveRuntimeTopologyMethod())
              .addMethod(getListRuntimeCapabilitiesMethod())
              .addMethod(getWatchRuntimeTopologyMethod())
              .build();
        }
      }
    }
    return result;
  }
}
