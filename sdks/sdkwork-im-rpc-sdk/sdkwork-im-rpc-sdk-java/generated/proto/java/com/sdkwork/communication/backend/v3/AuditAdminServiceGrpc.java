package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class AuditAdminServiceGrpc {

  private AuditAdminServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.AuditAdminService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListAuditRecordsRequest,
      com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> getListAuditRecordsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListAuditRecords",
      requestType = com.sdkwork.communication.backend.v3.ListAuditRecordsRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListAuditRecordsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListAuditRecordsRequest,
      com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> getListAuditRecordsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListAuditRecordsRequest, com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> getListAuditRecordsMethod;
    if ((getListAuditRecordsMethod = AuditAdminServiceGrpc.getListAuditRecordsMethod) == null) {
      synchronized (AuditAdminServiceGrpc.class) {
        if ((getListAuditRecordsMethod = AuditAdminServiceGrpc.getListAuditRecordsMethod) == null) {
          AuditAdminServiceGrpc.getListAuditRecordsMethod = getListAuditRecordsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListAuditRecordsRequest, com.sdkwork.communication.backend.v3.ListAuditRecordsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListAuditRecords"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListAuditRecordsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListAuditRecordsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AuditAdminServiceMethodDescriptorSupplier("ListAuditRecords"))
              .build();
        }
      }
    }
    return getListAuditRecordsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateAuditRecordRequest,
      com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> getCreateAuditRecordMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateAuditRecord",
      requestType = com.sdkwork.communication.backend.v3.CreateAuditRecordRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateAuditRecordResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateAuditRecordRequest,
      com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> getCreateAuditRecordMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateAuditRecordRequest, com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> getCreateAuditRecordMethod;
    if ((getCreateAuditRecordMethod = AuditAdminServiceGrpc.getCreateAuditRecordMethod) == null) {
      synchronized (AuditAdminServiceGrpc.class) {
        if ((getCreateAuditRecordMethod = AuditAdminServiceGrpc.getCreateAuditRecordMethod) == null) {
          AuditAdminServiceGrpc.getCreateAuditRecordMethod = getCreateAuditRecordMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateAuditRecordRequest, com.sdkwork.communication.backend.v3.CreateAuditRecordResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateAuditRecord"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateAuditRecordRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateAuditRecordResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AuditAdminServiceMethodDescriptorSupplier("CreateAuditRecord"))
              .build();
        }
      }
    }
    return getCreateAuditRecordMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest,
      com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> getRetrieveAuditExportMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveAuditExport",
      requestType = com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest,
      com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> getRetrieveAuditExportMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest, com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> getRetrieveAuditExportMethod;
    if ((getRetrieveAuditExportMethod = AuditAdminServiceGrpc.getRetrieveAuditExportMethod) == null) {
      synchronized (AuditAdminServiceGrpc.class) {
        if ((getRetrieveAuditExportMethod = AuditAdminServiceGrpc.getRetrieveAuditExportMethod) == null) {
          AuditAdminServiceGrpc.getRetrieveAuditExportMethod = getRetrieveAuditExportMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest, com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveAuditExport"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse.getDefaultInstance()))
              .setSchemaDescriptor(new AuditAdminServiceMethodDescriptorSupplier("RetrieveAuditExport"))
              .build();
        }
      }
    }
    return getRetrieveAuditExportMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static AuditAdminServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceStub>() {
        @java.lang.Override
        public AuditAdminServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AuditAdminServiceStub(channel, callOptions);
        }
      };
    return AuditAdminServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static AuditAdminServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceBlockingV2Stub>() {
        @java.lang.Override
        public AuditAdminServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AuditAdminServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return AuditAdminServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static AuditAdminServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceBlockingStub>() {
        @java.lang.Override
        public AuditAdminServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AuditAdminServiceBlockingStub(channel, callOptions);
        }
      };
    return AuditAdminServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static AuditAdminServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<AuditAdminServiceFutureStub>() {
        @java.lang.Override
        public AuditAdminServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new AuditAdminServiceFutureStub(channel, callOptions);
        }
      };
    return AuditAdminServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listAuditRecords(com.sdkwork.communication.backend.v3.ListAuditRecordsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListAuditRecordsMethod(), responseObserver);
    }

    /**
     */
    default void createAuditRecord(com.sdkwork.communication.backend.v3.CreateAuditRecordRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateAuditRecordMethod(), responseObserver);
    }

    /**
     */
    default void retrieveAuditExport(com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveAuditExportMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service AuditAdminService.
   */
  public static abstract class AuditAdminServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return AuditAdminServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service AuditAdminService.
   */
  public static final class AuditAdminServiceStub
      extends io.grpc.stub.AbstractAsyncStub<AuditAdminServiceStub> {
    private AuditAdminServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AuditAdminServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AuditAdminServiceStub(channel, callOptions);
    }

    /**
     */
    public void listAuditRecords(com.sdkwork.communication.backend.v3.ListAuditRecordsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListAuditRecordsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createAuditRecord(com.sdkwork.communication.backend.v3.CreateAuditRecordRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateAuditRecordMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveAuditExport(com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveAuditExportMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service AuditAdminService.
   */
  public static final class AuditAdminServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<AuditAdminServiceBlockingV2Stub> {
    private AuditAdminServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AuditAdminServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AuditAdminServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListAuditRecordsResponse listAuditRecords(com.sdkwork.communication.backend.v3.ListAuditRecordsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListAuditRecordsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateAuditRecordResponse createAuditRecord(com.sdkwork.communication.backend.v3.CreateAuditRecordRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateAuditRecordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse retrieveAuditExport(com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveAuditExportMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service AuditAdminService.
   */
  public static final class AuditAdminServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<AuditAdminServiceBlockingStub> {
    private AuditAdminServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AuditAdminServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AuditAdminServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListAuditRecordsResponse listAuditRecords(com.sdkwork.communication.backend.v3.ListAuditRecordsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListAuditRecordsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateAuditRecordResponse createAuditRecord(com.sdkwork.communication.backend.v3.CreateAuditRecordRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateAuditRecordMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse retrieveAuditExport(com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveAuditExportMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service AuditAdminService.
   */
  public static final class AuditAdminServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<AuditAdminServiceFutureStub> {
    private AuditAdminServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected AuditAdminServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new AuditAdminServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListAuditRecordsResponse> listAuditRecords(
        com.sdkwork.communication.backend.v3.ListAuditRecordsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListAuditRecordsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateAuditRecordResponse> createAuditRecord(
        com.sdkwork.communication.backend.v3.CreateAuditRecordRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateAuditRecordMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse> retrieveAuditExport(
        com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveAuditExportMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_AUDIT_RECORDS = 0;
  private static final int METHODID_CREATE_AUDIT_RECORD = 1;
  private static final int METHODID_RETRIEVE_AUDIT_EXPORT = 2;

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
        case METHODID_LIST_AUDIT_RECORDS:
          serviceImpl.listAuditRecords((com.sdkwork.communication.backend.v3.ListAuditRecordsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListAuditRecordsResponse>) responseObserver);
          break;
        case METHODID_CREATE_AUDIT_RECORD:
          serviceImpl.createAuditRecord((com.sdkwork.communication.backend.v3.CreateAuditRecordRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateAuditRecordResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_AUDIT_EXPORT:
          serviceImpl.retrieveAuditExport((com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse>) responseObserver);
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
          getListAuditRecordsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListAuditRecordsRequest,
              com.sdkwork.communication.backend.v3.ListAuditRecordsResponse>(
                service, METHODID_LIST_AUDIT_RECORDS)))
        .addMethod(
          getCreateAuditRecordMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateAuditRecordRequest,
              com.sdkwork.communication.backend.v3.CreateAuditRecordResponse>(
                service, METHODID_CREATE_AUDIT_RECORD)))
        .addMethod(
          getRetrieveAuditExportMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveAuditExportRequest,
              com.sdkwork.communication.backend.v3.RetrieveAuditExportResponse>(
                service, METHODID_RETRIEVE_AUDIT_EXPORT)))
        .build();
  }

  private static abstract class AuditAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    AuditAdminServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("AuditAdminService");
    }
  }

  private static final class AuditAdminServiceFileDescriptorSupplier
      extends AuditAdminServiceBaseDescriptorSupplier {
    AuditAdminServiceFileDescriptorSupplier() {}
  }

  private static final class AuditAdminServiceMethodDescriptorSupplier
      extends AuditAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    AuditAdminServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (AuditAdminServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new AuditAdminServiceFileDescriptorSupplier())
              .addMethod(getListAuditRecordsMethod())
              .addMethod(getCreateAuditRecordMethod())
              .addMethod(getRetrieveAuditExportMethod())
              .build();
        }
      }
    }
    return result;
  }
}
