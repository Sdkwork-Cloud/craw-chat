package com.sdkwork.communication.internal.v1;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 * <pre>
 * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
 * service resolves tenant, organization, delegated user/session, conversation,
 * and space from framework-verified mTLS identity, a signed caller context,
 * and the ticket ledger; callers must never provide those authority selectors
 * in request metadata or the protobuf payload.
 * </pre>
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class GroupKnowledgebaseLaunchTicketServiceGrpc {

  private GroupKnowledgebaseLaunchTicketServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.internal.v1.GroupKnowledgebaseLaunchTicketService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest,
      com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> getConsumeGroupKnowledgebaseLaunchTicketMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ConsumeGroupKnowledgebaseLaunchTicket",
      requestType = com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest.class,
      responseType = com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest,
      com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> getConsumeGroupKnowledgebaseLaunchTicketMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest, com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> getConsumeGroupKnowledgebaseLaunchTicketMethod;
    if ((getConsumeGroupKnowledgebaseLaunchTicketMethod = GroupKnowledgebaseLaunchTicketServiceGrpc.getConsumeGroupKnowledgebaseLaunchTicketMethod) == null) {
      synchronized (GroupKnowledgebaseLaunchTicketServiceGrpc.class) {
        if ((getConsumeGroupKnowledgebaseLaunchTicketMethod = GroupKnowledgebaseLaunchTicketServiceGrpc.getConsumeGroupKnowledgebaseLaunchTicketMethod) == null) {
          GroupKnowledgebaseLaunchTicketServiceGrpc.getConsumeGroupKnowledgebaseLaunchTicketMethod = getConsumeGroupKnowledgebaseLaunchTicketMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest, com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ConsumeGroupKnowledgebaseLaunchTicket"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse.getDefaultInstance()))
              .setSchemaDescriptor(new GroupKnowledgebaseLaunchTicketServiceMethodDescriptorSupplier("ConsumeGroupKnowledgebaseLaunchTicket"))
              .build();
        }
      }
    }
    return getConsumeGroupKnowledgebaseLaunchTicketMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static GroupKnowledgebaseLaunchTicketServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceStub>() {
        @java.lang.Override
        public GroupKnowledgebaseLaunchTicketServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new GroupKnowledgebaseLaunchTicketServiceStub(channel, callOptions);
        }
      };
    return GroupKnowledgebaseLaunchTicketServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub>() {
        @java.lang.Override
        public GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static GroupKnowledgebaseLaunchTicketServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceBlockingStub>() {
        @java.lang.Override
        public GroupKnowledgebaseLaunchTicketServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new GroupKnowledgebaseLaunchTicketServiceBlockingStub(channel, callOptions);
        }
      };
    return GroupKnowledgebaseLaunchTicketServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static GroupKnowledgebaseLaunchTicketServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<GroupKnowledgebaseLaunchTicketServiceFutureStub>() {
        @java.lang.Override
        public GroupKnowledgebaseLaunchTicketServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new GroupKnowledgebaseLaunchTicketServiceFutureStub(channel, callOptions);
        }
      };
    return GroupKnowledgebaseLaunchTicketServiceFutureStub.newStub(factory, channel);
  }

  /**
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public interface AsyncService {

    /**
     */
    default void consumeGroupKnowledgebaseLaunchTicket(com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getConsumeGroupKnowledgebaseLaunchTicketMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service GroupKnowledgebaseLaunchTicketService.
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public static abstract class GroupKnowledgebaseLaunchTicketServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return GroupKnowledgebaseLaunchTicketServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service GroupKnowledgebaseLaunchTicketService.
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public static final class GroupKnowledgebaseLaunchTicketServiceStub
      extends io.grpc.stub.AbstractAsyncStub<GroupKnowledgebaseLaunchTicketServiceStub> {
    private GroupKnowledgebaseLaunchTicketServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected GroupKnowledgebaseLaunchTicketServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new GroupKnowledgebaseLaunchTicketServiceStub(channel, callOptions);
    }

    /**
     */
    public void consumeGroupKnowledgebaseLaunchTicket(com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getConsumeGroupKnowledgebaseLaunchTicketMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service GroupKnowledgebaseLaunchTicketService.
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public static final class GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub> {
    private GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new GroupKnowledgebaseLaunchTicketServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse consumeGroupKnowledgebaseLaunchTicket(com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getConsumeGroupKnowledgebaseLaunchTicketMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service GroupKnowledgebaseLaunchTicketService.
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public static final class GroupKnowledgebaseLaunchTicketServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<GroupKnowledgebaseLaunchTicketServiceBlockingStub> {
    private GroupKnowledgebaseLaunchTicketServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected GroupKnowledgebaseLaunchTicketServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new GroupKnowledgebaseLaunchTicketServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse consumeGroupKnowledgebaseLaunchTicket(com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getConsumeGroupKnowledgebaseLaunchTicketMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service GroupKnowledgebaseLaunchTicketService.
   * <pre>
   * Internal capability-ticket exchange for sdkwork-knowledgebase. The IM
   * service resolves tenant, organization, delegated user/session, conversation,
   * and space from framework-verified mTLS identity, a signed caller context,
   * and the ticket ledger; callers must never provide those authority selectors
   * in request metadata or the protobuf payload.
   * </pre>
   */
  public static final class GroupKnowledgebaseLaunchTicketServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<GroupKnowledgebaseLaunchTicketServiceFutureStub> {
    private GroupKnowledgebaseLaunchTicketServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected GroupKnowledgebaseLaunchTicketServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new GroupKnowledgebaseLaunchTicketServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse> consumeGroupKnowledgebaseLaunchTicket(
        com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getConsumeGroupKnowledgebaseLaunchTicketMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CONSUME_GROUP_KNOWLEDGEBASE_LAUNCH_TICKET = 0;

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
        case METHODID_CONSUME_GROUP_KNOWLEDGEBASE_LAUNCH_TICKET:
          serviceImpl.consumeGroupKnowledgebaseLaunchTicket((com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse>) responseObserver);
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
          getConsumeGroupKnowledgebaseLaunchTicketMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketRequest,
              com.sdkwork.communication.internal.v1.ConsumeGroupKnowledgebaseLaunchTicketResponse>(
                service, METHODID_CONSUME_GROUP_KNOWLEDGEBASE_LAUNCH_TICKET)))
        .build();
  }

  private static abstract class GroupKnowledgebaseLaunchTicketServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    GroupKnowledgebaseLaunchTicketServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.internal.v1.GroupKnowledgebaseLaunchTicketServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("GroupKnowledgebaseLaunchTicketService");
    }
  }

  private static final class GroupKnowledgebaseLaunchTicketServiceFileDescriptorSupplier
      extends GroupKnowledgebaseLaunchTicketServiceBaseDescriptorSupplier {
    GroupKnowledgebaseLaunchTicketServiceFileDescriptorSupplier() {}
  }

  private static final class GroupKnowledgebaseLaunchTicketServiceMethodDescriptorSupplier
      extends GroupKnowledgebaseLaunchTicketServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    GroupKnowledgebaseLaunchTicketServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (GroupKnowledgebaseLaunchTicketServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new GroupKnowledgebaseLaunchTicketServiceFileDescriptorSupplier())
              .addMethod(getConsumeGroupKnowledgebaseLaunchTicketMethod())
              .build();
        }
      }
    }
    return result;
  }
}
