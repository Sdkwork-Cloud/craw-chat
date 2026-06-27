package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class MessageDispatchServiceGrpc {

  private MessageDispatchServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.MessageDispatchService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest,
      com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> getDispatchConversationMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DispatchConversationMessage",
      requestType = com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest.class,
      responseType = com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest,
      com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> getDispatchConversationMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest, com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> getDispatchConversationMessageMethod;
    if ((getDispatchConversationMessageMethod = MessageDispatchServiceGrpc.getDispatchConversationMessageMethod) == null) {
      synchronized (MessageDispatchServiceGrpc.class) {
        if ((getDispatchConversationMessageMethod = MessageDispatchServiceGrpc.getDispatchConversationMessageMethod) == null) {
          MessageDispatchServiceGrpc.getDispatchConversationMessageMethod = getDispatchConversationMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest, com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DispatchConversationMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageDispatchServiceMethodDescriptorSupplier("DispatchConversationMessage"))
              .build();
        }
      }
    }
    return getDispatchConversationMessageMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static MessageDispatchServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceStub>() {
        @java.lang.Override
        public MessageDispatchServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageDispatchServiceStub(channel, callOptions);
        }
      };
    return MessageDispatchServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static MessageDispatchServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceBlockingV2Stub>() {
        @java.lang.Override
        public MessageDispatchServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageDispatchServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return MessageDispatchServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static MessageDispatchServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceBlockingStub>() {
        @java.lang.Override
        public MessageDispatchServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageDispatchServiceBlockingStub(channel, callOptions);
        }
      };
    return MessageDispatchServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static MessageDispatchServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageDispatchServiceFutureStub>() {
        @java.lang.Override
        public MessageDispatchServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageDispatchServiceFutureStub(channel, callOptions);
        }
      };
    return MessageDispatchServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void dispatchConversationMessage(com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDispatchConversationMessageMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service MessageDispatchService.
   */
  public static abstract class MessageDispatchServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return MessageDispatchServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service MessageDispatchService.
   */
  public static final class MessageDispatchServiceStub
      extends io.grpc.stub.AbstractAsyncStub<MessageDispatchServiceStub> {
    private MessageDispatchServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageDispatchServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageDispatchServiceStub(channel, callOptions);
    }

    /**
     */
    public void dispatchConversationMessage(com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDispatchConversationMessageMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service MessageDispatchService.
   */
  public static final class MessageDispatchServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<MessageDispatchServiceBlockingV2Stub> {
    private MessageDispatchServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageDispatchServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageDispatchServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse dispatchConversationMessage(com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDispatchConversationMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service MessageDispatchService.
   */
  public static final class MessageDispatchServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<MessageDispatchServiceBlockingStub> {
    private MessageDispatchServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageDispatchServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageDispatchServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse dispatchConversationMessage(com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDispatchConversationMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service MessageDispatchService.
   */
  public static final class MessageDispatchServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<MessageDispatchServiceFutureStub> {
    private MessageDispatchServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageDispatchServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageDispatchServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse> dispatchConversationMessage(
        com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDispatchConversationMessageMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_DISPATCH_CONVERSATION_MESSAGE = 0;

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
        case METHODID_DISPATCH_CONVERSATION_MESSAGE:
          serviceImpl.dispatchConversationMessage((com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse>) responseObserver);
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
          getDispatchConversationMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.DispatchConversationMessageRequest,
              com.sdkwork.communication.internal.v1.DispatchConversationMessageResponse>(
                service, METHODID_DISPATCH_CONVERSATION_MESSAGE)))
        .build();
  }

  private static abstract class MessageDispatchServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    MessageDispatchServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.MessageDispatchServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("MessageDispatchService");
    }
  }

  private static final class MessageDispatchServiceFileDescriptorSupplier
      extends MessageDispatchServiceBaseDescriptorSupplier {
    MessageDispatchServiceFileDescriptorSupplier() {}
  }

  private static final class MessageDispatchServiceMethodDescriptorSupplier
      extends MessageDispatchServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    MessageDispatchServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (MessageDispatchServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new MessageDispatchServiceFileDescriptorSupplier())
              .addMethod(getDispatchConversationMessageMethod())
              .build();
        }
      }
    }
    return result;
  }
}
