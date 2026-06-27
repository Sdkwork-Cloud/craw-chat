package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class AutomationServiceGrpc {

  private AutomationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.AutomationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest,
      com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> getCreateAutomationExecutionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAutomationExecution",
      requestType = com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest,
      com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> getCreateAutomationExecutionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest, com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> getCreateAutomationExecutionMethod;
    if ((getCreateAutomationExecutionMethod = AutomationServiceGrpc.getCreateAutomationExecutionMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getCreateAutomationExecutionMethod = AutomationServiceGrpc.getCreateAutomationExecutionMethod) == null) {
          AutomationServiceGrpc.getCreateAutomationExecutionMethod = getCreateAutomationExecutionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest, com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAutomationExecution"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("CreateAutomationExecution"))
              .build();
        }
      }
    }
    return getCreateAutomationExecutionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest,
      com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> getRetrieveAutomationExecutionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveAutomationExecution",
      requestType = com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest,
      com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> getRetrieveAutomationExecutionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest, com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> getRetrieveAutomationExecutionMethod;
    if ((getRetrieveAutomationExecutionMethod = AutomationServiceGrpc.getRetrieveAutomationExecutionMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getRetrieveAutomationExecutionMethod = AutomationServiceGrpc.getRetrieveAutomationExecutionMethod) == null) {
          AutomationServiceGrpc.getRetrieveAutomationExecutionMethod = getRetrieveAutomationExecutionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest, com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveAutomationExecution"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("RetrieveAutomationExecution"))
              .build();
        }
      }
    }
    return getRetrieveAutomationExecutionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseRequest,
      com.sdkwork.communication.app.v3.CreateAgentResponseResponse> getCreateAgentResponseMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAgentResponse",
      requestType = com.sdkwork.communication.app.v3.CreateAgentResponseRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateAgentResponseResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseRequest,
      com.sdkwork.communication.app.v3.CreateAgentResponseResponse> getCreateAgentResponseMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseRequest, com.sdkwork.communication.app.v3.CreateAgentResponseResponse> getCreateAgentResponseMethod;
    if ((getCreateAgentResponseMethod = AutomationServiceGrpc.getCreateAgentResponseMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getCreateAgentResponseMethod = AutomationServiceGrpc.getCreateAgentResponseMethod) == null) {
          AutomationServiceGrpc.getCreateAgentResponseMethod = getCreateAgentResponseMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateAgentResponseRequest, com.sdkwork.communication.app.v3.CreateAgentResponseResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAgentResponse"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentResponseRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentResponseResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("CreateAgentResponse"))
              .build();
        }
      }
    }
    return getCreateAgentResponseMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentResponseRequest,
      com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> getCompleteAgentResponseMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CompleteAgentResponse",
      requestType = com.sdkwork.communication.app.v3.CompleteAgentResponseRequest.class,
      responseType = com.sdkwork.communication.app.v3.CompleteAgentResponseResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentResponseRequest,
      com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> getCompleteAgentResponseMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentResponseRequest, com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> getCompleteAgentResponseMethod;
    if ((getCompleteAgentResponseMethod = AutomationServiceGrpc.getCompleteAgentResponseMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getCompleteAgentResponseMethod = AutomationServiceGrpc.getCompleteAgentResponseMethod) == null) {
          AutomationServiceGrpc.getCompleteAgentResponseMethod = getCompleteAgentResponseMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CompleteAgentResponseRequest, com.sdkwork.communication.app.v3.CompleteAgentResponseResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CompleteAgentResponse"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteAgentResponseRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteAgentResponseResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("CompleteAgentResponse"))
              .build();
        }
      }
    }
    return getCompleteAgentResponseMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest,
      com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> getCreateAgentResponseFrameMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAgentResponseFrame",
      requestType = com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest,
      com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> getCreateAgentResponseFrameMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest, com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> getCreateAgentResponseFrameMethod;
    if ((getCreateAgentResponseFrameMethod = AutomationServiceGrpc.getCreateAgentResponseFrameMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getCreateAgentResponseFrameMethod = AutomationServiceGrpc.getCreateAgentResponseFrameMethod) == null) {
          AutomationServiceGrpc.getCreateAgentResponseFrameMethod = getCreateAgentResponseFrameMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest, com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAgentResponseFrame"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("CreateAgentResponseFrame"))
              .build();
        }
      }
    }
    return getCreateAgentResponseFrameMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RequestAgentToolCallRequest,
      com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> getRequestAgentToolCallMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RequestAgentToolCall",
      requestType = com.sdkwork.communication.app.v3.RequestAgentToolCallRequest.class,
      responseType = com.sdkwork.communication.app.v3.RequestAgentToolCallResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RequestAgentToolCallRequest,
      com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> getRequestAgentToolCallMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RequestAgentToolCallRequest, com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> getRequestAgentToolCallMethod;
    if ((getRequestAgentToolCallMethod = AutomationServiceGrpc.getRequestAgentToolCallMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getRequestAgentToolCallMethod = AutomationServiceGrpc.getRequestAgentToolCallMethod) == null) {
          AutomationServiceGrpc.getRequestAgentToolCallMethod = getRequestAgentToolCallMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RequestAgentToolCallRequest, com.sdkwork.communication.app.v3.RequestAgentToolCallResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RequestAgentToolCall"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RequestAgentToolCallRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RequestAgentToolCallResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("RequestAgentToolCall"))
              .build();
        }
      }
    }
    return getRequestAgentToolCallMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest,
      com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> getCompleteAgentToolCallMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CompleteAgentToolCall",
      requestType = com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest.class,
      responseType = com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest,
      com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> getCompleteAgentToolCallMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest, com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> getCompleteAgentToolCallMethod;
    if ((getCompleteAgentToolCallMethod = AutomationServiceGrpc.getCompleteAgentToolCallMethod) == null) {
      synchronized (AutomationServiceGrpc.class) {
        if ((getCompleteAgentToolCallMethod = AutomationServiceGrpc.getCompleteAgentToolCallMethod) == null) {
          AutomationServiceGrpc.getCompleteAgentToolCallMethod = getCompleteAgentToolCallMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest, com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CompleteAgentToolCall"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AutomationServiceMethodDescriptorSupplier("CompleteAgentToolCall"))
              .build();
        }
      }
    }
    return getCompleteAgentToolCallMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static AutomationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AutomationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AutomationServiceStub>() {
        @java.lang.Override
        public AutomationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AutomationServiceStub(channel, callOptions);
        }
      };
    return AutomationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static AutomationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AutomationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AutomationServiceBlockingV2Stub>() {
        @java.lang.Override
        public AutomationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AutomationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return AutomationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static AutomationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AutomationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AutomationServiceBlockingStub>() {
        @java.lang.Override
        public AutomationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AutomationServiceBlockingStub(channel, callOptions);
        }
      };
    return AutomationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static AutomationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AutomationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AutomationServiceFutureStub>() {
        @java.lang.Override
        public AutomationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AutomationServiceFutureStub(channel, callOptions);
        }
      };
    return AutomationServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createAutomationExecution(com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAutomationExecutionMethod(), responseObserver);
    }

    /**
     */
    default void retrieveAutomationExecution(com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveAutomationExecutionMethod(), responseObserver);
    }

    /**
     */
    default void createAgentResponse(com.sdkwork.communication.app.v3.CreateAgentResponseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAgentResponseMethod(), responseObserver);
    }

    /**
     */
    default void completeAgentResponse(com.sdkwork.communication.app.v3.CompleteAgentResponseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCompleteAgentResponseMethod(), responseObserver);
    }

    /**
     */
    default void createAgentResponseFrame(com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAgentResponseFrameMethod(), responseObserver);
    }

    /**
     */
    default void requestAgentToolCall(com.sdkwork.communication.app.v3.RequestAgentToolCallRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRequestAgentToolCallMethod(), responseObserver);
    }

    /**
     */
    default void completeAgentToolCall(com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCompleteAgentToolCallMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service AutomationService.
   */
  public static abstract class AutomationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return AutomationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service AutomationService.
   */
  public static final class AutomationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<AutomationServiceStub> {
    private AutomationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AutomationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AutomationServiceStub(channel, callOptions);
    }

    /**
     */
    public void createAutomationExecution(com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAutomationExecutionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveAutomationExecution(com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveAutomationExecutionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAgentResponse(com.sdkwork.communication.app.v3.CreateAgentResponseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAgentResponseMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void completeAgentResponse(com.sdkwork.communication.app.v3.CompleteAgentResponseRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCompleteAgentResponseMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAgentResponseFrame(com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAgentResponseFrameMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void requestAgentToolCall(com.sdkwork.communication.app.v3.RequestAgentToolCallRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRequestAgentToolCallMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void completeAgentToolCall(com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCompleteAgentToolCallMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service AutomationService.
   */
  public static final class AutomationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<AutomationServiceBlockingV2Stub> {
    private AutomationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AutomationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AutomationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse createAutomationExecution(com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAutomationExecutionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse retrieveAutomationExecution(com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveAutomationExecutionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentResponseResponse createAgentResponse(com.sdkwork.communication.app.v3.CreateAgentResponseRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAgentResponseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteAgentResponseResponse completeAgentResponse(com.sdkwork.communication.app.v3.CompleteAgentResponseRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCompleteAgentResponseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse createAgentResponseFrame(com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAgentResponseFrameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RequestAgentToolCallResponse requestAgentToolCall(com.sdkwork.communication.app.v3.RequestAgentToolCallRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRequestAgentToolCallMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse completeAgentToolCall(com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCompleteAgentToolCallMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service AutomationService.
   */
  public static final class AutomationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<AutomationServiceBlockingStub> {
    private AutomationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AutomationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AutomationServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse createAutomationExecution(com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAutomationExecutionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse retrieveAutomationExecution(com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveAutomationExecutionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentResponseResponse createAgentResponse(com.sdkwork.communication.app.v3.CreateAgentResponseRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAgentResponseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteAgentResponseResponse completeAgentResponse(com.sdkwork.communication.app.v3.CompleteAgentResponseRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCompleteAgentResponseMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse createAgentResponseFrame(com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAgentResponseFrameMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RequestAgentToolCallResponse requestAgentToolCall(com.sdkwork.communication.app.v3.RequestAgentToolCallRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRequestAgentToolCallMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse completeAgentToolCall(com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCompleteAgentToolCallMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service AutomationService.
   */
  public static final class AutomationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<AutomationServiceFutureStub> {
    private AutomationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AutomationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AutomationServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse> createAutomationExecution(
        com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAutomationExecutionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse> retrieveAutomationExecution(
        com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveAutomationExecutionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateAgentResponseResponse> createAgentResponse(
        com.sdkwork.communication.app.v3.CreateAgentResponseRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAgentResponseMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CompleteAgentResponseResponse> completeAgentResponse(
        com.sdkwork.communication.app.v3.CompleteAgentResponseRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCompleteAgentResponseMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse> createAgentResponseFrame(
        com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAgentResponseFrameMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RequestAgentToolCallResponse> requestAgentToolCall(
        com.sdkwork.communication.app.v3.RequestAgentToolCallRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRequestAgentToolCallMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse> completeAgentToolCall(
        com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCompleteAgentToolCallMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_AUTOMATION_EXECUTION = 0;
  private static final int METHODID_RETRIEVE_AUTOMATION_EXECUTION = 1;
  private static final int METHODID_CREATE_AGENT_RESPONSE = 2;
  private static final int METHODID_COMPLETE_AGENT_RESPONSE = 3;
  private static final int METHODID_CREATE_AGENT_RESPONSE_FRAME = 4;
  private static final int METHODID_REQUEST_AGENT_TOOL_CALL = 5;
  private static final int METHODID_COMPLETE_AGENT_TOOL_CALL = 6;

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
        case METHODID_CREATE_AUTOMATION_EXECUTION:
          serviceImpl.createAutomationExecution((com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_AUTOMATION_EXECUTION:
          serviceImpl.retrieveAutomationExecution((com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse>) responseObserver);
          break;
        case METHODID_CREATE_AGENT_RESPONSE:
          serviceImpl.createAgentResponse((com.sdkwork.communication.app.v3.CreateAgentResponseRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseResponse>) responseObserver);
          break;
        case METHODID_COMPLETE_AGENT_RESPONSE:
          serviceImpl.completeAgentResponse((com.sdkwork.communication.app.v3.CompleteAgentResponseRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentResponseResponse>) responseObserver);
          break;
        case METHODID_CREATE_AGENT_RESPONSE_FRAME:
          serviceImpl.createAgentResponseFrame((com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse>) responseObserver);
          break;
        case METHODID_REQUEST_AGENT_TOOL_CALL:
          serviceImpl.requestAgentToolCall((com.sdkwork.communication.app.v3.RequestAgentToolCallRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RequestAgentToolCallResponse>) responseObserver);
          break;
        case METHODID_COMPLETE_AGENT_TOOL_CALL:
          serviceImpl.completeAgentToolCall((com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse>) responseObserver);
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
          getCreateAutomationExecutionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateAutomationExecutionRequest,
              com.sdkwork.communication.app.v3.CreateAutomationExecutionResponse>(
                service, METHODID_CREATE_AUTOMATION_EXECUTION)))
        .addMethod(
          getRetrieveAutomationExecutionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveAutomationExecutionRequest,
              com.sdkwork.communication.app.v3.RetrieveAutomationExecutionResponse>(
                service, METHODID_RETRIEVE_AUTOMATION_EXECUTION)))
        .addMethod(
          getCreateAgentResponseMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateAgentResponseRequest,
              com.sdkwork.communication.app.v3.CreateAgentResponseResponse>(
                service, METHODID_CREATE_AGENT_RESPONSE)))
        .addMethod(
          getCompleteAgentResponseMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CompleteAgentResponseRequest,
              com.sdkwork.communication.app.v3.CompleteAgentResponseResponse>(
                service, METHODID_COMPLETE_AGENT_RESPONSE)))
        .addMethod(
          getCreateAgentResponseFrameMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateAgentResponseFrameRequest,
              com.sdkwork.communication.app.v3.CreateAgentResponseFrameResponse>(
                service, METHODID_CREATE_AGENT_RESPONSE_FRAME)))
        .addMethod(
          getRequestAgentToolCallMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RequestAgentToolCallRequest,
              com.sdkwork.communication.app.v3.RequestAgentToolCallResponse>(
                service, METHODID_REQUEST_AGENT_TOOL_CALL)))
        .addMethod(
          getCompleteAgentToolCallMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CompleteAgentToolCallRequest,
              com.sdkwork.communication.app.v3.CompleteAgentToolCallResponse>(
                service, METHODID_COMPLETE_AGENT_TOOL_CALL)))
        .build();
  }

  private static abstract class AutomationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    AutomationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.AutomationServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("AutomationService");
    }
  }

  private static final class AutomationServiceFileDescriptorSupplier
      extends AutomationServiceBaseDescriptorSupplier {
    AutomationServiceFileDescriptorSupplier() {}
  }

  private static final class AutomationServiceMethodDescriptorSupplier
      extends AutomationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    AutomationServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (AutomationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new AutomationServiceFileDescriptorSupplier())
              .addMethod(getCreateAutomationExecutionMethod())
              .addMethod(getRetrieveAutomationExecutionMethod())
              .addMethod(getCreateAgentResponseMethod())
              .addMethod(getCompleteAgentResponseMethod())
              .addMethod(getCreateAgentResponseFrameMethod())
              .addMethod(getRequestAgentToolCallMethod())
              .addMethod(getCompleteAgentToolCallMethod())
              .build();
        }
      }
    }
    return result;
  }
}
