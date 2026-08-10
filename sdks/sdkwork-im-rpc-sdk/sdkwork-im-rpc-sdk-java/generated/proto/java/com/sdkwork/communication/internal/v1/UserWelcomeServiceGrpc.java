package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * System-agent onboarding: ensures the target user receives the system
 * Welcome message exactly once. Trusted callers only (service-mTLS).
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class UserWelcomeServiceGrpc {

  private UserWelcomeServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.UserWelcomeService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest,
      com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> getSendWelcomeMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "SendWelcomeMessage",
      requestType = com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest.class,
      responseType = com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest,
      com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> getSendWelcomeMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest, com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> getSendWelcomeMessageMethod;
    if ((getSendWelcomeMessageMethod = UserWelcomeServiceGrpc.getSendWelcomeMessageMethod) == null) {
      synchronized (UserWelcomeServiceGrpc.class) {
        if ((getSendWelcomeMessageMethod = UserWelcomeServiceGrpc.getSendWelcomeMessageMethod) == null) {
          UserWelcomeServiceGrpc.getSendWelcomeMessageMethod = getSendWelcomeMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest, com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "SendWelcomeMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new UserWelcomeServiceMethodDescriptorSupplier("SendWelcomeMessage"))
              .build();
        }
      }
    }
    return getSendWelcomeMessageMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static UserWelcomeServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceStub>() {
        @java.lang.Override
        public UserWelcomeServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserWelcomeServiceStub(channel, callOptions);
        }
      };
    return UserWelcomeServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static UserWelcomeServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceBlockingV2Stub>() {
        @java.lang.Override
        public UserWelcomeServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserWelcomeServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return UserWelcomeServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static UserWelcomeServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceBlockingStub>() {
        @java.lang.Override
        public UserWelcomeServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserWelcomeServiceBlockingStub(channel, callOptions);
        }
      };
    return UserWelcomeServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static UserWelcomeServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<UserWelcomeServiceFutureStub>() {
        @java.lang.Override
        public UserWelcomeServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new UserWelcomeServiceFutureStub(channel, callOptions);
        }
      };
    return UserWelcomeServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void sendWelcomeMessage(com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getSendWelcomeMessageMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service UserWelcomeService.
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public static abstract class UserWelcomeServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return UserWelcomeServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service UserWelcomeService.
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public static final class UserWelcomeServiceStub
      extends io.grpc.stub.AbstractAsyncStub<UserWelcomeServiceStub> {
    private UserWelcomeServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserWelcomeServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserWelcomeServiceStub(channel, callOptions);
    }

    /**
     */
    public void sendWelcomeMessage(com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getSendWelcomeMessageMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service UserWelcomeService.
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public static final class UserWelcomeServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<UserWelcomeServiceBlockingV2Stub> {
    private UserWelcomeServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserWelcomeServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserWelcomeServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse sendWelcomeMessage(com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getSendWelcomeMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service UserWelcomeService.
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public static final class UserWelcomeServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<UserWelcomeServiceBlockingStub> {
    private UserWelcomeServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserWelcomeServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserWelcomeServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse sendWelcomeMessage(com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getSendWelcomeMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service UserWelcomeService.
   * <pre>
   * System-agent onboarding: ensures the target user receives the system
   * Welcome message exactly once. Trusted callers only (service-mTLS).
   * </pre>
   */
  public static final class UserWelcomeServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<UserWelcomeServiceFutureStub> {
    private UserWelcomeServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected UserWelcomeServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new UserWelcomeServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse> sendWelcomeMessage(
        com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getSendWelcomeMessageMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_SEND_WELCOME_MESSAGE = 0;

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
        case METHODID_SEND_WELCOME_MESSAGE:
          serviceImpl.sendWelcomeMessage((com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse>) responseObserver);
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
          getSendWelcomeMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.SendWelcomeMessageRequest,
              com.sdkwork.communication.internal.v1.SendWelcomeMessageResponse>(
                service, METHODID_SEND_WELCOME_MESSAGE)))
        .build();
  }

  private static abstract class UserWelcomeServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    UserWelcomeServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.MessageDispatchServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("UserWelcomeService");
    }
  }

  private static final class UserWelcomeServiceFileDescriptorSupplier
      extends UserWelcomeServiceBaseDescriptorSupplier {
    UserWelcomeServiceFileDescriptorSupplier() {}
  }

  private static final class UserWelcomeServiceMethodDescriptorSupplier
      extends UserWelcomeServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    UserWelcomeServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (UserWelcomeServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new UserWelcomeServiceFileDescriptorSupplier())
              .addMethod(getSendWelcomeMessageMethod())
              .build();
        }
      }
    }
    return result;
  }
}
