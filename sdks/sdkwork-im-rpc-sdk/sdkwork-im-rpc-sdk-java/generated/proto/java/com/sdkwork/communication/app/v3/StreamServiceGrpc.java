package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class StreamServiceGrpc {

  private StreamServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.StreamService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamRequest,
      com.sdkwork.communication.app.v3.CreateStreamResponse> getCreateStreamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateStream",
      requestType = com.sdkwork.communication.app.v3.CreateStreamRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateStreamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamRequest,
      com.sdkwork.communication.app.v3.CreateStreamResponse> getCreateStreamMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamRequest, com.sdkwork.communication.app.v3.CreateStreamResponse> getCreateStreamMethod;
    if ((getCreateStreamMethod = StreamServiceGrpc.getCreateStreamMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getCreateStreamMethod = StreamServiceGrpc.getCreateStreamMethod) == null) {
          StreamServiceGrpc.getCreateStreamMethod = getCreateStreamMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateStreamRequest, com.sdkwork.communication.app.v3.CreateStreamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateStream"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateStreamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateStreamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("CreateStream"))
              .build();
        }
      }
    }
    return getCreateStreamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListStreamFramesRequest,
      com.sdkwork.communication.app.v3.ListStreamFramesResponse> getListStreamFramesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListStreamFrames",
      requestType = com.sdkwork.communication.app.v3.ListStreamFramesRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListStreamFramesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListStreamFramesRequest,
      com.sdkwork.communication.app.v3.ListStreamFramesResponse> getListStreamFramesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListStreamFramesRequest, com.sdkwork.communication.app.v3.ListStreamFramesResponse> getListStreamFramesMethod;
    if ((getListStreamFramesMethod = StreamServiceGrpc.getListStreamFramesMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getListStreamFramesMethod = StreamServiceGrpc.getListStreamFramesMethod) == null) {
          StreamServiceGrpc.getListStreamFramesMethod = getListStreamFramesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListStreamFramesRequest, com.sdkwork.communication.app.v3.ListStreamFramesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListStreamFrames"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListStreamFramesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListStreamFramesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("ListStreamFrames"))
              .build();
        }
      }
    }
    return getListStreamFramesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AppendStreamFrameRequest,
      com.sdkwork.communication.app.v3.AppendStreamFrameResponse> getAppendStreamFrameMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AppendStreamFrame",
      requestType = com.sdkwork.communication.app.v3.AppendStreamFrameRequest.class,
      responseType = com.sdkwork.communication.app.v3.AppendStreamFrameResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AppendStreamFrameRequest,
      com.sdkwork.communication.app.v3.AppendStreamFrameResponse> getAppendStreamFrameMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AppendStreamFrameRequest, com.sdkwork.communication.app.v3.AppendStreamFrameResponse> getAppendStreamFrameMethod;
    if ((getAppendStreamFrameMethod = StreamServiceGrpc.getAppendStreamFrameMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getAppendStreamFrameMethod = StreamServiceGrpc.getAppendStreamFrameMethod) == null) {
          StreamServiceGrpc.getAppendStreamFrameMethod = getAppendStreamFrameMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AppendStreamFrameRequest, com.sdkwork.communication.app.v3.AppendStreamFrameResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AppendStreamFrame"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AppendStreamFrameRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AppendStreamFrameResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("AppendStreamFrame"))
              .build();
        }
      }
    }
    return getAppendStreamFrameMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest,
      com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> getCreateStreamCheckpointMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateStreamCheckpoint",
      requestType = com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest,
      com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> getCreateStreamCheckpointMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest, com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> getCreateStreamCheckpointMethod;
    if ((getCreateStreamCheckpointMethod = StreamServiceGrpc.getCreateStreamCheckpointMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getCreateStreamCheckpointMethod = StreamServiceGrpc.getCreateStreamCheckpointMethod) == null) {
          StreamServiceGrpc.getCreateStreamCheckpointMethod = getCreateStreamCheckpointMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest, com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateStreamCheckpoint"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("CreateStreamCheckpoint"))
              .build();
        }
      }
    }
    return getCreateStreamCheckpointMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteStreamRequest,
      com.sdkwork.communication.app.v3.CompleteStreamResponse> getCompleteStreamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CompleteStream",
      requestType = com.sdkwork.communication.app.v3.CompleteStreamRequest.class,
      responseType = com.sdkwork.communication.app.v3.CompleteStreamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteStreamRequest,
      com.sdkwork.communication.app.v3.CompleteStreamResponse> getCompleteStreamMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteStreamRequest, com.sdkwork.communication.app.v3.CompleteStreamResponse> getCompleteStreamMethod;
    if ((getCompleteStreamMethod = StreamServiceGrpc.getCompleteStreamMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getCompleteStreamMethod = StreamServiceGrpc.getCompleteStreamMethod) == null) {
          StreamServiceGrpc.getCompleteStreamMethod = getCompleteStreamMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CompleteStreamRequest, com.sdkwork.communication.app.v3.CompleteStreamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CompleteStream"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteStreamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteStreamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("CompleteStream"))
              .build();
        }
      }
    }
    return getCompleteStreamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AbortStreamRequest,
      com.sdkwork.communication.app.v3.AbortStreamResponse> getAbortStreamMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AbortStream",
      requestType = com.sdkwork.communication.app.v3.AbortStreamRequest.class,
      responseType = com.sdkwork.communication.app.v3.AbortStreamResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AbortStreamRequest,
      com.sdkwork.communication.app.v3.AbortStreamResponse> getAbortStreamMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AbortStreamRequest, com.sdkwork.communication.app.v3.AbortStreamResponse> getAbortStreamMethod;
    if ((getAbortStreamMethod = StreamServiceGrpc.getAbortStreamMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getAbortStreamMethod = StreamServiceGrpc.getAbortStreamMethod) == null) {
          StreamServiceGrpc.getAbortStreamMethod = getAbortStreamMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AbortStreamRequest, com.sdkwork.communication.app.v3.AbortStreamResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AbortStream"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AbortStreamRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AbortStreamResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("AbortStream"))
              .build();
        }
      }
    }
    return getAbortStreamMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchStreamFramesRequest,
      com.sdkwork.communication.app.v3.WatchStreamFramesResponse> getWatchStreamFramesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchStreamFrames",
      requestType = com.sdkwork.communication.app.v3.WatchStreamFramesRequest.class,
      responseType = com.sdkwork.communication.app.v3.WatchStreamFramesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchStreamFramesRequest,
      com.sdkwork.communication.app.v3.WatchStreamFramesResponse> getWatchStreamFramesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchStreamFramesRequest, com.sdkwork.communication.app.v3.WatchStreamFramesResponse> getWatchStreamFramesMethod;
    if ((getWatchStreamFramesMethod = StreamServiceGrpc.getWatchStreamFramesMethod) == null) {
      synchronized (StreamServiceGrpc.class) {
        if ((getWatchStreamFramesMethod = StreamServiceGrpc.getWatchStreamFramesMethod) == null) {
          StreamServiceGrpc.getWatchStreamFramesMethod = getWatchStreamFramesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.WatchStreamFramesRequest, com.sdkwork.communication.app.v3.WatchStreamFramesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchStreamFrames"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchStreamFramesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchStreamFramesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new StreamServiceMethodDescriptorSupplier("WatchStreamFrames"))
              .build();
        }
      }
    }
    return getWatchStreamFramesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static StreamServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<StreamServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<StreamServiceStub>() {
        @java.lang.Override
        public StreamServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new StreamServiceStub(channel, callOptions);
        }
      };
    return StreamServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static StreamServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<StreamServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<StreamServiceBlockingV2Stub>() {
        @java.lang.Override
        public StreamServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new StreamServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return StreamServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static StreamServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<StreamServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<StreamServiceBlockingStub>() {
        @java.lang.Override
        public StreamServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new StreamServiceBlockingStub(channel, callOptions);
        }
      };
    return StreamServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static StreamServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<StreamServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<StreamServiceFutureStub>() {
        @java.lang.Override
        public StreamServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new StreamServiceFutureStub(channel, callOptions);
        }
      };
    return StreamServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createStream(com.sdkwork.communication.app.v3.CreateStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateStreamMethod(), responseObserver);
    }

    /**
     */
    default void listStreamFrames(com.sdkwork.communication.app.v3.ListStreamFramesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListStreamFramesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListStreamFramesMethod(), responseObserver);
    }

    /**
     */
    default void appendStreamFrame(com.sdkwork.communication.app.v3.AppendStreamFrameRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AppendStreamFrameResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAppendStreamFrameMethod(), responseObserver);
    }

    /**
     */
    default void createStreamCheckpoint(com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateStreamCheckpointMethod(), responseObserver);
    }

    /**
     */
    default void completeStream(com.sdkwork.communication.app.v3.CompleteStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteStreamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCompleteStreamMethod(), responseObserver);
    }

    /**
     */
    default void abortStream(com.sdkwork.communication.app.v3.AbortStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AbortStreamResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAbortStreamMethod(), responseObserver);
    }

    /**
     */
    default void watchStreamFrames(com.sdkwork.communication.app.v3.WatchStreamFramesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchStreamFramesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchStreamFramesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service StreamService.
   */
  public static abstract class StreamServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return StreamServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service StreamService.
   */
  public static final class StreamServiceStub
      extends io.grpc.stub.AbstractAsyncStub<StreamServiceStub> {
    private StreamServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected StreamServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new StreamServiceStub(channel, callOptions);
    }

    /**
     */
    public void createStream(com.sdkwork.communication.app.v3.CreateStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateStreamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listStreamFrames(com.sdkwork.communication.app.v3.ListStreamFramesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListStreamFramesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListStreamFramesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void appendStreamFrame(com.sdkwork.communication.app.v3.AppendStreamFrameRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AppendStreamFrameResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAppendStreamFrameMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createStreamCheckpoint(com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateStreamCheckpointMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void completeStream(com.sdkwork.communication.app.v3.CompleteStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteStreamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCompleteStreamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void abortStream(com.sdkwork.communication.app.v3.AbortStreamRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AbortStreamResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAbortStreamMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchStreamFrames(com.sdkwork.communication.app.v3.WatchStreamFramesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchStreamFramesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchStreamFramesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service StreamService.
   */
  public static final class StreamServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<StreamServiceBlockingV2Stub> {
    private StreamServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected StreamServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new StreamServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateStreamResponse createStream(com.sdkwork.communication.app.v3.CreateStreamRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListStreamFramesResponse listStreamFrames(com.sdkwork.communication.app.v3.ListStreamFramesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListStreamFramesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AppendStreamFrameResponse appendStreamFrame(com.sdkwork.communication.app.v3.AppendStreamFrameRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAppendStreamFrameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse createStreamCheckpoint(com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateStreamCheckpointMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteStreamResponse completeStream(com.sdkwork.communication.app.v3.CompleteStreamRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCompleteStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AbortStreamResponse abortStream(com.sdkwork.communication.app.v3.AbortStreamRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAbortStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.app.v3.WatchStreamFramesResponse>
        watchStreamFrames(com.sdkwork.communication.app.v3.WatchStreamFramesRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchStreamFramesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service StreamService.
   */
  public static final class StreamServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<StreamServiceBlockingStub> {
    private StreamServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected StreamServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new StreamServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateStreamResponse createStream(com.sdkwork.communication.app.v3.CreateStreamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListStreamFramesResponse listStreamFrames(com.sdkwork.communication.app.v3.ListStreamFramesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListStreamFramesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AppendStreamFrameResponse appendStreamFrame(com.sdkwork.communication.app.v3.AppendStreamFrameRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAppendStreamFrameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse createStreamCheckpoint(com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateStreamCheckpointMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteStreamResponse completeStream(com.sdkwork.communication.app.v3.CompleteStreamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCompleteStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AbortStreamResponse abortStream(com.sdkwork.communication.app.v3.AbortStreamRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAbortStreamMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.app.v3.WatchStreamFramesResponse> watchStreamFrames(
        com.sdkwork.communication.app.v3.WatchStreamFramesRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchStreamFramesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service StreamService.
   */
  public static final class StreamServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<StreamServiceFutureStub> {
    private StreamServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected StreamServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new StreamServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateStreamResponse> createStream(
        com.sdkwork.communication.app.v3.CreateStreamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateStreamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListStreamFramesResponse> listStreamFrames(
        com.sdkwork.communication.app.v3.ListStreamFramesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListStreamFramesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AppendStreamFrameResponse> appendStreamFrame(
        com.sdkwork.communication.app.v3.AppendStreamFrameRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAppendStreamFrameMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse> createStreamCheckpoint(
        com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateStreamCheckpointMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CompleteStreamResponse> completeStream(
        com.sdkwork.communication.app.v3.CompleteStreamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCompleteStreamMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AbortStreamResponse> abortStream(
        com.sdkwork.communication.app.v3.AbortStreamRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAbortStreamMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_STREAM = 0;
  private static final int METHODID_LIST_STREAM_FRAMES = 1;
  private static final int METHODID_APPEND_STREAM_FRAME = 2;
  private static final int METHODID_CREATE_STREAM_CHECKPOINT = 3;
  private static final int METHODID_COMPLETE_STREAM = 4;
  private static final int METHODID_ABORT_STREAM = 5;
  private static final int METHODID_WATCH_STREAM_FRAMES = 6;

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
        case METHODID_CREATE_STREAM:
          serviceImpl.createStream((com.sdkwork.communication.app.v3.CreateStreamRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamResponse>) responseObserver);
          break;
        case METHODID_LIST_STREAM_FRAMES:
          serviceImpl.listStreamFrames((com.sdkwork.communication.app.v3.ListStreamFramesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListStreamFramesResponse>) responseObserver);
          break;
        case METHODID_APPEND_STREAM_FRAME:
          serviceImpl.appendStreamFrame((com.sdkwork.communication.app.v3.AppendStreamFrameRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AppendStreamFrameResponse>) responseObserver);
          break;
        case METHODID_CREATE_STREAM_CHECKPOINT:
          serviceImpl.createStreamCheckpoint((com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse>) responseObserver);
          break;
        case METHODID_COMPLETE_STREAM:
          serviceImpl.completeStream((com.sdkwork.communication.app.v3.CompleteStreamRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteStreamResponse>) responseObserver);
          break;
        case METHODID_ABORT_STREAM:
          serviceImpl.abortStream((com.sdkwork.communication.app.v3.AbortStreamRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AbortStreamResponse>) responseObserver);
          break;
        case METHODID_WATCH_STREAM_FRAMES:
          serviceImpl.watchStreamFrames((com.sdkwork.communication.app.v3.WatchStreamFramesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchStreamFramesResponse>) responseObserver);
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
          getCreateStreamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateStreamRequest,
              com.sdkwork.communication.app.v3.CreateStreamResponse>(
                service, METHODID_CREATE_STREAM)))
        .addMethod(
          getListStreamFramesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListStreamFramesRequest,
              com.sdkwork.communication.app.v3.ListStreamFramesResponse>(
                service, METHODID_LIST_STREAM_FRAMES)))
        .addMethod(
          getAppendStreamFrameMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AppendStreamFrameRequest,
              com.sdkwork.communication.app.v3.AppendStreamFrameResponse>(
                service, METHODID_APPEND_STREAM_FRAME)))
        .addMethod(
          getCreateStreamCheckpointMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateStreamCheckpointRequest,
              com.sdkwork.communication.app.v3.CreateStreamCheckpointResponse>(
                service, METHODID_CREATE_STREAM_CHECKPOINT)))
        .addMethod(
          getCompleteStreamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CompleteStreamRequest,
              com.sdkwork.communication.app.v3.CompleteStreamResponse>(
                service, METHODID_COMPLETE_STREAM)))
        .addMethod(
          getAbortStreamMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AbortStreamRequest,
              com.sdkwork.communication.app.v3.AbortStreamResponse>(
                service, METHODID_ABORT_STREAM)))
        .addMethod(
          getWatchStreamFramesMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.WatchStreamFramesRequest,
              com.sdkwork.communication.app.v3.WatchStreamFramesResponse>(
                service, METHODID_WATCH_STREAM_FRAMES)))
        .build();
  }

  private static abstract class StreamServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    StreamServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.StreamServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("StreamService");
    }
  }

  private static final class StreamServiceFileDescriptorSupplier
      extends StreamServiceBaseDescriptorSupplier {
    StreamServiceFileDescriptorSupplier() {}
  }

  private static final class StreamServiceMethodDescriptorSupplier
      extends StreamServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    StreamServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (StreamServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new StreamServiceFileDescriptorSupplier())
              .addMethod(getCreateStreamMethod())
              .addMethod(getListStreamFramesMethod())
              .addMethod(getAppendStreamFrameMethod())
              .addMethod(getCreateStreamCheckpointMethod())
              .addMethod(getCompleteStreamMethod())
              .addMethod(getAbortStreamMethod())
              .addMethod(getWatchStreamFramesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
