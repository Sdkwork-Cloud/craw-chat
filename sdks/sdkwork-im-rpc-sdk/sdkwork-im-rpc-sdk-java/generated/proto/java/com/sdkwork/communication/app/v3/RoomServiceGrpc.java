package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class RoomServiceGrpc {

  private RoomServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.RoomService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateRoomRequest,
      com.sdkwork.communication.app.v3.CreateRoomResponse> getCreateRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateRoom",
      requestType = com.sdkwork.communication.app.v3.CreateRoomRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateRoomRequest,
      com.sdkwork.communication.app.v3.CreateRoomResponse> getCreateRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateRoomRequest, com.sdkwork.communication.app.v3.CreateRoomResponse> getCreateRoomMethod;
    if ((getCreateRoomMethod = RoomServiceGrpc.getCreateRoomMethod) == null) {
      synchronized (RoomServiceGrpc.class) {
        if ((getCreateRoomMethod = RoomServiceGrpc.getCreateRoomMethod) == null) {
          RoomServiceGrpc.getCreateRoomMethod = getCreateRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateRoomRequest, com.sdkwork.communication.app.v3.CreateRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomServiceMethodDescriptorSupplier("CreateRoom"))
              .build();
        }
      }
    }
    return getCreateRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveRoomRequest,
      com.sdkwork.communication.app.v3.RetrieveRoomResponse> getRetrieveRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveRoom",
      requestType = com.sdkwork.communication.app.v3.RetrieveRoomRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveRoomRequest,
      com.sdkwork.communication.app.v3.RetrieveRoomResponse> getRetrieveRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveRoomRequest, com.sdkwork.communication.app.v3.RetrieveRoomResponse> getRetrieveRoomMethod;
    if ((getRetrieveRoomMethod = RoomServiceGrpc.getRetrieveRoomMethod) == null) {
      synchronized (RoomServiceGrpc.class) {
        if ((getRetrieveRoomMethod = RoomServiceGrpc.getRetrieveRoomMethod) == null) {
          RoomServiceGrpc.getRetrieveRoomMethod = getRetrieveRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveRoomRequest, com.sdkwork.communication.app.v3.RetrieveRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomServiceMethodDescriptorSupplier("RetrieveRoom"))
              .build();
        }
      }
    }
    return getRetrieveRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EnterRoomRequest,
      com.sdkwork.communication.app.v3.EnterRoomResponse> getEnterRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "EnterRoom",
      requestType = com.sdkwork.communication.app.v3.EnterRoomRequest.class,
      responseType = com.sdkwork.communication.app.v3.EnterRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EnterRoomRequest,
      com.sdkwork.communication.app.v3.EnterRoomResponse> getEnterRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EnterRoomRequest, com.sdkwork.communication.app.v3.EnterRoomResponse> getEnterRoomMethod;
    if ((getEnterRoomMethod = RoomServiceGrpc.getEnterRoomMethod) == null) {
      synchronized (RoomServiceGrpc.class) {
        if ((getEnterRoomMethod = RoomServiceGrpc.getEnterRoomMethod) == null) {
          RoomServiceGrpc.getEnterRoomMethod = getEnterRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.EnterRoomRequest, com.sdkwork.communication.app.v3.EnterRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "EnterRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EnterRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EnterRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomServiceMethodDescriptorSupplier("EnterRoom"))
              .build();
        }
      }
    }
    return getEnterRoomMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveRoomRequest,
      com.sdkwork.communication.app.v3.LeaveRoomResponse> getLeaveRoomMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "LeaveRoom",
      requestType = com.sdkwork.communication.app.v3.LeaveRoomRequest.class,
      responseType = com.sdkwork.communication.app.v3.LeaveRoomResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveRoomRequest,
      com.sdkwork.communication.app.v3.LeaveRoomResponse> getLeaveRoomMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.LeaveRoomRequest, com.sdkwork.communication.app.v3.LeaveRoomResponse> getLeaveRoomMethod;
    if ((getLeaveRoomMethod = RoomServiceGrpc.getLeaveRoomMethod) == null) {
      synchronized (RoomServiceGrpc.class) {
        if ((getLeaveRoomMethod = RoomServiceGrpc.getLeaveRoomMethod) == null) {
          RoomServiceGrpc.getLeaveRoomMethod = getLeaveRoomMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.LeaveRoomRequest, com.sdkwork.communication.app.v3.LeaveRoomResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "LeaveRoom"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.LeaveRoomRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.LeaveRoomResponse.getDefaultInstance()))
              .setSchemaDescriptor(new RoomServiceMethodDescriptorSupplier("LeaveRoom"))
              .build();
        }
      }
    }
    return getLeaveRoomMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static RoomServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomServiceStub>() {
        @java.lang.Override
        public RoomServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomServiceStub(channel, callOptions);
        }
      };
    return RoomServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static RoomServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomServiceBlockingV2Stub>() {
        @java.lang.Override
        public RoomServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return RoomServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static RoomServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomServiceBlockingStub>() {
        @java.lang.Override
        public RoomServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomServiceBlockingStub(channel, callOptions);
        }
      };
    return RoomServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static RoomServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<RoomServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<RoomServiceFutureStub>() {
        @java.lang.Override
        public RoomServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new RoomServiceFutureStub(channel, callOptions);
        }
      };
    return RoomServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createRoom(com.sdkwork.communication.app.v3.CreateRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateRoomMethod(), responseObserver);
    }

    /**
     */
    default void retrieveRoom(com.sdkwork.communication.app.v3.RetrieveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveRoomMethod(), responseObserver);
    }

    /**
     */
    default void enterRoom(com.sdkwork.communication.app.v3.EnterRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EnterRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEnterRoomMethod(), responseObserver);
    }

    /**
     */
    default void leaveRoom(com.sdkwork.communication.app.v3.LeaveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveRoomResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getLeaveRoomMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service RoomService.
   */
  public static abstract class RoomServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return RoomServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service RoomService.
   */
  public static final class RoomServiceStub
      extends io.grpc.stub.AbstractAsyncStub<RoomServiceStub> {
    private RoomServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomServiceStub(channel, callOptions);
    }

    /**
     */
    public void createRoom(com.sdkwork.communication.app.v3.CreateRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveRoom(com.sdkwork.communication.app.v3.RetrieveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void enterRoom(com.sdkwork.communication.app.v3.EnterRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EnterRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEnterRoomMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void leaveRoom(com.sdkwork.communication.app.v3.LeaveRoomRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveRoomResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getLeaveRoomMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service RoomService.
   */
  public static final class RoomServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<RoomServiceBlockingV2Stub> {
    private RoomServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateRoomResponse createRoom(com.sdkwork.communication.app.v3.CreateRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveRoomResponse retrieveRoom(com.sdkwork.communication.app.v3.RetrieveRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EnterRoomResponse enterRoom(com.sdkwork.communication.app.v3.EnterRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEnterRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.LeaveRoomResponse leaveRoom(com.sdkwork.communication.app.v3.LeaveRoomRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getLeaveRoomMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service RoomService.
   */
  public static final class RoomServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<RoomServiceBlockingStub> {
    private RoomServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateRoomResponse createRoom(com.sdkwork.communication.app.v3.CreateRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveRoomResponse retrieveRoom(com.sdkwork.communication.app.v3.RetrieveRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EnterRoomResponse enterRoom(com.sdkwork.communication.app.v3.EnterRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEnterRoomMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.LeaveRoomResponse leaveRoom(com.sdkwork.communication.app.v3.LeaveRoomRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getLeaveRoomMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service RoomService.
   */
  public static final class RoomServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<RoomServiceFutureStub> {
    private RoomServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected RoomServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new RoomServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateRoomResponse> createRoom(
        com.sdkwork.communication.app.v3.CreateRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveRoomResponse> retrieveRoom(
        com.sdkwork.communication.app.v3.RetrieveRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.EnterRoomResponse> enterRoom(
        com.sdkwork.communication.app.v3.EnterRoomRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEnterRoomMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.LeaveRoomResponse> leaveRoom(
        com.sdkwork.communication.app.v3.LeaveRoomRequest request) {
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
          serviceImpl.createRoom((com.sdkwork.communication.app.v3.CreateRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateRoomResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_ROOM:
          serviceImpl.retrieveRoom((com.sdkwork.communication.app.v3.RetrieveRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveRoomResponse>) responseObserver);
          break;
        case METHODID_ENTER_ROOM:
          serviceImpl.enterRoom((com.sdkwork.communication.app.v3.EnterRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EnterRoomResponse>) responseObserver);
          break;
        case METHODID_LEAVE_ROOM:
          serviceImpl.leaveRoom((com.sdkwork.communication.app.v3.LeaveRoomRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.LeaveRoomResponse>) responseObserver);
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
              com.sdkwork.communication.app.v3.CreateRoomRequest,
              com.sdkwork.communication.app.v3.CreateRoomResponse>(
                service, METHODID_CREATE_ROOM)))
        .addMethod(
          getRetrieveRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveRoomRequest,
              com.sdkwork.communication.app.v3.RetrieveRoomResponse>(
                service, METHODID_RETRIEVE_ROOM)))
        .addMethod(
          getEnterRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.EnterRoomRequest,
              com.sdkwork.communication.app.v3.EnterRoomResponse>(
                service, METHODID_ENTER_ROOM)))
        .addMethod(
          getLeaveRoomMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.LeaveRoomRequest,
              com.sdkwork.communication.app.v3.LeaveRoomResponse>(
                service, METHODID_LEAVE_ROOM)))
        .build();
  }

  private static abstract class RoomServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    RoomServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.RoomServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("RoomService");
    }
  }

  private static final class RoomServiceFileDescriptorSupplier
      extends RoomServiceBaseDescriptorSupplier {
    RoomServiceFileDescriptorSupplier() {}
  }

  private static final class RoomServiceMethodDescriptorSupplier
      extends RoomServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    RoomServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (RoomServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new RoomServiceFileDescriptorSupplier())
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
