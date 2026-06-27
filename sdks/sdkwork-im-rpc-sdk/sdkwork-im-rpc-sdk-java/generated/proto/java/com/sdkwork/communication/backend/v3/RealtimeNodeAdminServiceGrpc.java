package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RealtimeNodeAdminServiceGrpc {

  private RealtimeNodeAdminServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.RealtimeNodeAdminService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest,
      com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> getActivateRealtimeNodeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ActivateRealtimeNode",
      requestType = com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest,
      com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> getActivateRealtimeNodeMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest, com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> getActivateRealtimeNodeMethod;
    if ((getActivateRealtimeNodeMethod = RealtimeNodeAdminServiceGrpc.getActivateRealtimeNodeMethod) == null) {
      synchronized (RealtimeNodeAdminServiceGrpc.class) {
        if ((getActivateRealtimeNodeMethod = RealtimeNodeAdminServiceGrpc.getActivateRealtimeNodeMethod) == null) {
          RealtimeNodeAdminServiceGrpc.getActivateRealtimeNodeMethod = getActivateRealtimeNodeMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest, com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ActivateRealtimeNode"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeNodeAdminServiceMethodDescriptorSupplier("ActivateRealtimeNode"))
              .build();
        }
      }
    }
    return getActivateRealtimeNodeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest,
      com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> getDrainRealtimeNodeMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DrainRealtimeNode",
      requestType = com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest.class,
      responseType = com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest,
      com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> getDrainRealtimeNodeMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest, com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> getDrainRealtimeNodeMethod;
    if ((getDrainRealtimeNodeMethod = RealtimeNodeAdminServiceGrpc.getDrainRealtimeNodeMethod) == null) {
      synchronized (RealtimeNodeAdminServiceGrpc.class) {
        if ((getDrainRealtimeNodeMethod = RealtimeNodeAdminServiceGrpc.getDrainRealtimeNodeMethod) == null) {
          RealtimeNodeAdminServiceGrpc.getDrainRealtimeNodeMethod = getDrainRealtimeNodeMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest, com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DrainRealtimeNode"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeNodeAdminServiceMethodDescriptorSupplier("DrainRealtimeNode"))
              .build();
        }
      }
    }
    return getDrainRealtimeNodeMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest,
      com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> getMigrateRealtimeNodeRoutesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "MigrateRealtimeNodeRoutes",
      requestType = com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest.class,
      responseType = com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest,
      com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> getMigrateRealtimeNodeRoutesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest, com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> getMigrateRealtimeNodeRoutesMethod;
    if ((getMigrateRealtimeNodeRoutesMethod = RealtimeNodeAdminServiceGrpc.getMigrateRealtimeNodeRoutesMethod) == null) {
      synchronized (RealtimeNodeAdminServiceGrpc.class) {
        if ((getMigrateRealtimeNodeRoutesMethod = RealtimeNodeAdminServiceGrpc.getMigrateRealtimeNodeRoutesMethod) == null) {
          RealtimeNodeAdminServiceGrpc.getMigrateRealtimeNodeRoutesMethod = getMigrateRealtimeNodeRoutesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest, com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "MigrateRealtimeNodeRoutes"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RealtimeNodeAdminServiceMethodDescriptorSupplier("MigrateRealtimeNodeRoutes"))
              .build();
        }
      }
    }
    return getMigrateRealtimeNodeRoutesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RealtimeNodeAdminServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceStub>() {
        @java.lang.Override
        public RealtimeNodeAdminServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeNodeAdminServiceStub(channel, callOptions);
        }
      };
    return RealtimeNodeAdminServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RealtimeNodeAdminServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceBlockingV2Stub>() {
        @java.lang.Override
        public RealtimeNodeAdminServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeNodeAdminServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RealtimeNodeAdminServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RealtimeNodeAdminServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceBlockingStub>() {
        @java.lang.Override
        public RealtimeNodeAdminServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeNodeAdminServiceBlockingStub(channel, callOptions);
        }
      };
    return RealtimeNodeAdminServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RealtimeNodeAdminServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RealtimeNodeAdminServiceFutureStub>() {
        @java.lang.Override
        public RealtimeNodeAdminServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RealtimeNodeAdminServiceFutureStub(channel, callOptions);
        }
      };
    return RealtimeNodeAdminServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void activateRealtimeNode(com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getActivateRealtimeNodeMethod(), responseObserver);
    }

    /**
     */
    default void drainRealtimeNode(com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDrainRealtimeNodeMethod(), responseObserver);
    }

    /**
     */
    default void migrateRealtimeNodeRoutes(com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getMigrateRealtimeNodeRoutesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RealtimeNodeAdminService.
   */
  public static abstract class RealtimeNodeAdminServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RealtimeNodeAdminServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RealtimeNodeAdminService.
   */
  public static final class RealtimeNodeAdminServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RealtimeNodeAdminServiceStub> {
    private RealtimeNodeAdminServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeNodeAdminServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeNodeAdminServiceStub(channel, callOptions);
    }

    /**
     */
    public void activateRealtimeNode(com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getActivateRealtimeNodeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void drainRealtimeNode(com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDrainRealtimeNodeMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void migrateRealtimeNodeRoutes(com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getMigrateRealtimeNodeRoutesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RealtimeNodeAdminService.
   */
  public static final class RealtimeNodeAdminServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RealtimeNodeAdminServiceBlockingV2Stub> {
    private RealtimeNodeAdminServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeNodeAdminServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeNodeAdminServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse activateRealtimeNode(com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getActivateRealtimeNodeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse drainRealtimeNode(com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDrainRealtimeNodeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse migrateRealtimeNodeRoutes(com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getMigrateRealtimeNodeRoutesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RealtimeNodeAdminService.
   */
  public static final class RealtimeNodeAdminServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RealtimeNodeAdminServiceBlockingStub> {
    private RealtimeNodeAdminServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeNodeAdminServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeNodeAdminServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse activateRealtimeNode(com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getActivateRealtimeNodeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse drainRealtimeNode(com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDrainRealtimeNodeMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse migrateRealtimeNodeRoutes(com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getMigrateRealtimeNodeRoutesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RealtimeNodeAdminService.
   */
  public static final class RealtimeNodeAdminServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RealtimeNodeAdminServiceFutureStub> {
    private RealtimeNodeAdminServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RealtimeNodeAdminServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RealtimeNodeAdminServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse> activateRealtimeNode(
        com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getActivateRealtimeNodeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse> drainRealtimeNode(
        com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDrainRealtimeNodeMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse> migrateRealtimeNodeRoutes(
        com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getMigrateRealtimeNodeRoutesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_ACTIVATE_REALTIME_NODE = 0;
  private static final int METHODID_DRAIN_REALTIME_NODE = 1;
  private static final int METHODID_MIGRATE_REALTIME_NODE_ROUTES = 2;

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
        case METHODID_ACTIVATE_REALTIME_NODE:
          serviceImpl.activateRealtimeNode((com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse>) responseObserver);
          break;
        case METHODID_DRAIN_REALTIME_NODE:
          serviceImpl.drainRealtimeNode((com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse>) responseObserver);
          break;
        case METHODID_MIGRATE_REALTIME_NODE_ROUTES:
          serviceImpl.migrateRealtimeNodeRoutes((com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse>) responseObserver);
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
          getActivateRealtimeNodeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ActivateRealtimeNodeRequest,
              com.sdkwork.communication.backend.v3.ActivateRealtimeNodeResponse>(
                service, METHODID_ACTIVATE_REALTIME_NODE)))
        .addMethod(
          getDrainRealtimeNodeMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.DrainRealtimeNodeRequest,
              com.sdkwork.communication.backend.v3.DrainRealtimeNodeResponse>(
                service, METHODID_DRAIN_REALTIME_NODE)))
        .addMethod(
          getMigrateRealtimeNodeRoutesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesRequest,
              com.sdkwork.communication.backend.v3.MigrateRealtimeNodeRoutesResponse>(
                service, METHODID_MIGRATE_REALTIME_NODE_ROUTES)))
        .build();
  }

  private static abstract class RealtimeNodeAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RealtimeNodeAdminServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RealtimeNodeAdminService");
    }
  }

  private static final class RealtimeNodeAdminServiceFileDescriptorSupplier
      extends RealtimeNodeAdminServiceBaseDescriptorSupplier {
    RealtimeNodeAdminServiceFileDescriptorSupplier() {}
  }

  private static final class RealtimeNodeAdminServiceMethodDescriptorSupplier
      extends RealtimeNodeAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RealtimeNodeAdminServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RealtimeNodeAdminServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RealtimeNodeAdminServiceFileDescriptorSupplier())
              .addMethod(getActivateRealtimeNodeMethod())
              .addMethod(getDrainRealtimeNodeMethod())
              .addMethod(getMigrateRealtimeNodeRoutesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
