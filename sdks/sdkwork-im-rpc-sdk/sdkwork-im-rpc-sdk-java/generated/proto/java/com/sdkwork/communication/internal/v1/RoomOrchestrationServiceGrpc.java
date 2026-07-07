package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RoomOrchestrationServiceGrpc {

  private RoomOrchestrationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.RoomOrchestrationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.CreateRoomRequest,
      com.sdkwork.communication.internal.v1.CreateRoomResponse> getCreateRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateRoom",
      requestType = com.sdkwork.communication.internal.v1.CreateRoomRequest.class,
      responseType = com.sdkwork.communication.internal.v1.CreateRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.CreateRoomRequest,
      com.sdkwork.communication.internal.v1.CreateRoomResponse> getCreateRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.CreateRoomRequest, com.sdkwork.communication.internal.v1.CreateRoomResponse> getCreateRoomMethod;
    if ((getCreateRoomMethod = RoomOrchestrationServiceGrpc.getCreateRoomMethod) == null) {
      synchronized (RoomOrchestrationServiceGrpc.class) {
        if ((getCreateRoomMethod = RoomOrchestrationServiceGrpc.getCreateRoomMethod) == null) {
          RoomOrchestrationServiceGrpc.getCreateRoomMethod = getCreateRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.CreateRoomRequest, com.sdkwork.communication.internal.v1.CreateRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.CreateRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.CreateRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomOrchestrationServiceMethodDescriptorSupplier("CreateRoom"))
              .build();
        }
      }
    }
    return getCreateRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRoomRequest,
      com.sdkwork.communication.internal.v1.RetrieveRoomResponse> getRetrieveRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveRoom",
      requestType = com.sdkwork.communication.internal.v1.RetrieveRoomRequest.class,
      responseType = com.sdkwork.communication.internal.v1.RetrieveRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRoomRequest,
      com.sdkwork.communication.internal.v1.RetrieveRoomResponse> getRetrieveRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.RetrieveRoomRequest, com.sdkwork.communication.internal.v1.RetrieveRoomResponse> getRetrieveRoomMethod;
    if ((getRetrieveRoomMethod = RoomOrchestrationServiceGrpc.getRetrieveRoomMethod) == null) {
      synchronized (RoomOrchestrationServiceGrpc.class) {
        if ((getRetrieveRoomMethod = RoomOrchestrationServiceGrpc.getRetrieveRoomMethod) == null) {
          RoomOrchestrationServiceGrpc.getRetrieveRoomMethod = getRetrieveRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.RetrieveRoomRequest, com.sdkwork.communication.internal.v1.RetrieveRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RetrieveRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.RetrieveRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomOrchestrationServiceMethodDescriptorSupplier("RetrieveRoom"))
              .build();
        }
      }
    }
    return getRetrieveRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.EnterRoomRequest,
      com.sdkwork.communication.internal.v1.EnterRoomResponse> getEnterRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "EnterRoom",
      requestType = com.sdkwork.communication.internal.v1.EnterRoomRequest.class,
      responseType = com.sdkwork.communication.internal.v1.EnterRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.EnterRoomRequest,
      com.sdkwork.communication.internal.v1.EnterRoomResponse> getEnterRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.EnterRoomRequest, com.sdkwork.communication.internal.v1.EnterRoomResponse> getEnterRoomMethod;
    if ((getEnterRoomMethod = RoomOrchestrationServiceGrpc.getEnterRoomMethod) == null) {
      synchronized (RoomOrchestrationServiceGrpc.class) {
        if ((getEnterRoomMethod = RoomOrchestrationServiceGrpc.getEnterRoomMethod) == null) {
          RoomOrchestrationServiceGrpc.getEnterRoomMethod = getEnterRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.EnterRoomRequest, com.sdkwork.communication.internal.v1.EnterRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "EnterRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.EnterRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.EnterRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomOrchestrationServiceMethodDescriptorSupplier("EnterRoom"))
              .build();
        }
      }
    }
    return getEnterRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.LeaveRoomRequest,
      com.sdkwork.communication.internal.v1.LeaveRoomResponse> getLeaveRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LeaveRoom",
      requestType = com.sdkwork.communication.internal.v1.LeaveRoomRequest.class,
      responseType = com.sdkwork.communication.internal.v1.LeaveRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.LeaveRoomRequest,
      com.sdkwork.communication.internal.v1.LeaveRoomResponse> getLeaveRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.LeaveRoomRequest, com.sdkwork.communication.internal.v1.LeaveRoomResponse> getLeaveRoomMethod;
    if ((getLeaveRoomMethod = RoomOrchestrationServiceGrpc.getLeaveRoomMethod) == null) {
      synchronized (RoomOrchestrationServiceGrpc.class) {
        if ((getLeaveRoomMethod = RoomOrchestrationServiceGrpc.getLeaveRoomMethod) == null) {
          RoomOrchestrationServiceGrpc.getLeaveRoomMethod = getLeaveRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.LeaveRoomRequest, com.sdkwork.communication.internal.v1.LeaveRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LeaveRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.LeaveRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.LeaveRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomOrchestrationServiceMethodDescriptorSupplier("LeaveRoom"))
              .build();
        }
      }
    }
    return getLeaveRoomMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RoomOrchestrationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceStub>() {
        @java.lang.Override
        public RoomOrchestrationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomOrchestrationServiceStub(channel, callOptions);
        }
      };
    return RoomOrchestrationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RoomOrchestrationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceBlockingV2Stub>() {
        @java.lang.Override
        public RoomOrchestrationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomOrchestrationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RoomOrchestrationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RoomOrchestrationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceBlockingStub>() {
        @java.lang.Override
        public RoomOrchestrationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomOrchestrationServiceBlockingStub(channel, callOptions);
        }
      };
    return RoomOrchestrationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RoomOrchestrationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomOrchestrationServiceFutureStub>() {
        @java.lang.Override
        public RoomOrchestrationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomOrchestrationServiceFutureStub(channel, callOptions);
        }
      };
    return RoomOrchestrationServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createRoom(com.sdkwork.communication.internal.v1.CreateRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.CreateRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateRoomMethod(), responseObserver);
    }

    /**
     */
    default void retrieveRoom(com.sdkwork.communication.internal.v1.RetrieveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveRoomMethod(), responseObserver);
    }

    /**
     */
    default void enterRoom(com.sdkwork.communication.internal.v1.EnterRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.EnterRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEnterRoomMethod(), responseObserver);
    }

    /**
     */
    default void leaveRoom(com.sdkwork.communication.internal.v1.LeaveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.LeaveRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLeaveRoomMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RoomOrchestrationService.
   */
  public static abstract class RoomOrchestrationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RoomOrchestrationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RoomOrchestrationService.
   */
  public static final class RoomOrchestrationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RoomOrchestrationServiceStub> {
    private RoomOrchestrationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomOrchestrationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomOrchestrationServiceStub(channel, callOptions);
    }

    /**
     */
    public void createRoom(com.sdkwork.communication.internal.v1.CreateRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.CreateRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveRoom(com.sdkwork.communication.internal.v1.RetrieveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void enterRoom(com.sdkwork.communication.internal.v1.EnterRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.EnterRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEnterRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void leaveRoom(com.sdkwork.communication.internal.v1.LeaveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.LeaveRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLeaveRoomMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RoomOrchestrationService.
   */
  public static final class RoomOrchestrationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RoomOrchestrationServiceBlockingV2Stub> {
    private RoomOrchestrationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomOrchestrationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomOrchestrationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.CreateRoomResponse createRoom(com.sdkwork.communication.internal.v1.CreateRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RetrieveRoomResponse retrieveRoom(com.sdkwork.communication.internal.v1.RetrieveRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.EnterRoomResponse enterRoom(com.sdkwork.communication.internal.v1.EnterRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEnterRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.LeaveRoomResponse leaveRoom(com.sdkwork.communication.internal.v1.LeaveRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLeaveRoomMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RoomOrchestrationService.
   */
  public static final class RoomOrchestrationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RoomOrchestrationServiceBlockingStub> {
    private RoomOrchestrationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomOrchestrationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomOrchestrationServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.CreateRoomResponse createRoom(com.sdkwork.communication.internal.v1.CreateRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.RetrieveRoomResponse retrieveRoom(com.sdkwork.communication.internal.v1.RetrieveRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.EnterRoomResponse enterRoom(com.sdkwork.communication.internal.v1.EnterRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnterRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.LeaveRoomResponse leaveRoom(com.sdkwork.communication.internal.v1.LeaveRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLeaveRoomMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RoomOrchestrationService.
   */
  public static final class RoomOrchestrationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RoomOrchestrationServiceFutureStub> {
    private RoomOrchestrationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomOrchestrationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomOrchestrationServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.CreateRoomResponse> createRoom(
        com.sdkwork.communication.internal.v1.CreateRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.RetrieveRoomResponse> retrieveRoom(
        com.sdkwork.communication.internal.v1.RetrieveRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.EnterRoomResponse> enterRoom(
        com.sdkwork.communication.internal.v1.EnterRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEnterRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.LeaveRoomResponse> leaveRoom(
        com.sdkwork.communication.internal.v1.LeaveRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getLeaveRoomMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_ROOM = 0;
  private static final int METHODID_RETRIEVE_ROOM = 1;
  private static final int METHODID_ENTER_ROOM = 2;
  private static final int METHODID_LEAVE_ROOM = 3;

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
        case METHODID_CREATE_ROOM:
          serviceImpl.createRoom((com.sdkwork.communication.internal.v1.CreateRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.CreateRoomResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_ROOM:
          serviceImpl.retrieveRoom((com.sdkwork.communication.internal.v1.RetrieveRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.RetrieveRoomResponse>) responseObserver);
          break;
        case METHODID_ENTER_ROOM:
          serviceImpl.enterRoom((com.sdkwork.communication.internal.v1.EnterRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.EnterRoomResponse>) responseObserver);
          break;
        case METHODID_LEAVE_ROOM:
          serviceImpl.leaveRoom((com.sdkwork.communication.internal.v1.LeaveRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.LeaveRoomResponse>) responseObserver);
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
          getCreateRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.CreateRoomRequest,
              com.sdkwork.communication.internal.v1.CreateRoomResponse>(
                service, METHODID_CREATE_ROOM)))
        .addMethod(
          getRetrieveRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.RetrieveRoomRequest,
              com.sdkwork.communication.internal.v1.RetrieveRoomResponse>(
                service, METHODID_RETRIEVE_ROOM)))
        .addMethod(
          getEnterRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.EnterRoomRequest,
              com.sdkwork.communication.internal.v1.EnterRoomResponse>(
                service, METHODID_ENTER_ROOM)))
        .addMethod(
          getLeaveRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.LeaveRoomRequest,
              com.sdkwork.communication.internal.v1.LeaveRoomResponse>(
                service, METHODID_LEAVE_ROOM)))
        .build();
  }

  private static abstract class RoomOrchestrationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RoomOrchestrationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.RoomOrchestrationServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RoomOrchestrationService");
    }
  }

  private static final class RoomOrchestrationServiceFileDescriptorSupplier
      extends RoomOrchestrationServiceBaseDescriptorSupplier {
    RoomOrchestrationServiceFileDescriptorSupplier() {}
  }

  private static final class RoomOrchestrationServiceMethodDescriptorSupplier
      extends RoomOrchestrationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RoomOrchestrationServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RoomOrchestrationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RoomOrchestrationServiceFileDescriptorSupplier())
              .addMethod(getCreateRoomMethod())
              .addMethod(getRetrieveRoomMethod())
              .addMethod(getEnterRoomMethod())
              .addMethod(getLeaveRoomMethod())
              .build();
        }
      }
    }
    return result;
  }
}
