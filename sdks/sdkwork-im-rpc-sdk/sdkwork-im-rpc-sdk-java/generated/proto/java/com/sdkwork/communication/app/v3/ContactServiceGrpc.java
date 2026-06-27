package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class ContactServiceGrpc {

  private ContactServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.ContactService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactsRequest,
      com.sdkwork.communication.app.v3.ListContactsResponse> getListContactsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListContacts",
      requestType = com.sdkwork.communication.app.v3.ListContactsRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListContactsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactsRequest,
      com.sdkwork.communication.app.v3.ListContactsResponse> getListContactsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactsRequest, com.sdkwork.communication.app.v3.ListContactsResponse> getListContactsMethod;
    if ((getListContactsMethod = ContactServiceGrpc.getListContactsMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getListContactsMethod = ContactServiceGrpc.getListContactsMethod) == null) {
          ContactServiceGrpc.getListContactsMethod = getListContactsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListContactsRequest, com.sdkwork.communication.app.v3.ListContactsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListContacts"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListContactsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListContactsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("ListContacts"))
              .build();
        }
      }
    }
    return getListContactsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactTagsRequest,
      com.sdkwork.communication.app.v3.ListContactTagsResponse> getListContactTagsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListContactTags",
      requestType = com.sdkwork.communication.app.v3.ListContactTagsRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListContactTagsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactTagsRequest,
      com.sdkwork.communication.app.v3.ListContactTagsResponse> getListContactTagsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListContactTagsRequest, com.sdkwork.communication.app.v3.ListContactTagsResponse> getListContactTagsMethod;
    if ((getListContactTagsMethod = ContactServiceGrpc.getListContactTagsMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getListContactTagsMethod = ContactServiceGrpc.getListContactTagsMethod) == null) {
          ContactServiceGrpc.getListContactTagsMethod = getListContactTagsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListContactTagsRequest, com.sdkwork.communication.app.v3.ListContactTagsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListContactTags"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListContactTagsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListContactTagsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("ListContactTags"))
              .build();
        }
      }
    }
    return getListContactTagsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactTagRequest,
      com.sdkwork.communication.app.v3.CreateContactTagResponse> getCreateContactTagMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateContactTag",
      requestType = com.sdkwork.communication.app.v3.CreateContactTagRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateContactTagResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactTagRequest,
      com.sdkwork.communication.app.v3.CreateContactTagResponse> getCreateContactTagMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactTagRequest, com.sdkwork.communication.app.v3.CreateContactTagResponse> getCreateContactTagMethod;
    if ((getCreateContactTagMethod = ContactServiceGrpc.getCreateContactTagMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getCreateContactTagMethod = ContactServiceGrpc.getCreateContactTagMethod) == null) {
          ContactServiceGrpc.getCreateContactTagMethod = getCreateContactTagMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateContactTagRequest, com.sdkwork.communication.app.v3.CreateContactTagResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateContactTag"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateContactTagRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateContactTagResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("CreateContactTag"))
              .build();
        }
      }
    }
    return getCreateContactTagMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactTagRequest,
      com.sdkwork.communication.app.v3.UpdateContactTagResponse> getUpdateContactTagMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UpdateContactTag",
      requestType = com.sdkwork.communication.app.v3.UpdateContactTagRequest.class,
      responseType = com.sdkwork.communication.app.v3.UpdateContactTagResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactTagRequest,
      com.sdkwork.communication.app.v3.UpdateContactTagResponse> getUpdateContactTagMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactTagRequest, com.sdkwork.communication.app.v3.UpdateContactTagResponse> getUpdateContactTagMethod;
    if ((getUpdateContactTagMethod = ContactServiceGrpc.getUpdateContactTagMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getUpdateContactTagMethod = ContactServiceGrpc.getUpdateContactTagMethod) == null) {
          ContactServiceGrpc.getUpdateContactTagMethod = getUpdateContactTagMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UpdateContactTagRequest, com.sdkwork.communication.app.v3.UpdateContactTagResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UpdateContactTag"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateContactTagRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateContactTagResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("UpdateContactTag"))
              .build();
        }
      }
    }
    return getUpdateContactTagMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteContactTagRequest,
      com.sdkwork.communication.app.v3.DeleteContactTagResponse> getDeleteContactTagMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteContactTag",
      requestType = com.sdkwork.communication.app.v3.DeleteContactTagRequest.class,
      responseType = com.sdkwork.communication.app.v3.DeleteContactTagResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteContactTagRequest,
      com.sdkwork.communication.app.v3.DeleteContactTagResponse> getDeleteContactTagMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteContactTagRequest, com.sdkwork.communication.app.v3.DeleteContactTagResponse> getDeleteContactTagMethod;
    if ((getDeleteContactTagMethod = ContactServiceGrpc.getDeleteContactTagMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getDeleteContactTagMethod = ContactServiceGrpc.getDeleteContactTagMethod) == null) {
          ContactServiceGrpc.getDeleteContactTagMethod = getDeleteContactTagMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.DeleteContactTagRequest, com.sdkwork.communication.app.v3.DeleteContactTagResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteContactTag"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteContactTagRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteContactTagResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("DeleteContactTag"))
              .build();
        }
      }
    }
    return getDeleteContactTagMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactRecommendationRequest,
      com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> getCreateContactRecommendationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateContactRecommendation",
      requestType = com.sdkwork.communication.app.v3.CreateContactRecommendationRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateContactRecommendationResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactRecommendationRequest,
      com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> getCreateContactRecommendationMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateContactRecommendationRequest, com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> getCreateContactRecommendationMethod;
    if ((getCreateContactRecommendationMethod = ContactServiceGrpc.getCreateContactRecommendationMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getCreateContactRecommendationMethod = ContactServiceGrpc.getCreateContactRecommendationMethod) == null) {
          ContactServiceGrpc.getCreateContactRecommendationMethod = getCreateContactRecommendationMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateContactRecommendationRequest, com.sdkwork.communication.app.v3.CreateContactRecommendationResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateContactRecommendation"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateContactRecommendationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateContactRecommendationResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("CreateContactRecommendation"))
              .build();
        }
      }
    }
    return getCreateContactRecommendationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest,
      com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> getRetrieveContactPreferencesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveContactPreferences",
      requestType = com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest,
      com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> getRetrieveContactPreferencesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest, com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> getRetrieveContactPreferencesMethod;
    if ((getRetrieveContactPreferencesMethod = ContactServiceGrpc.getRetrieveContactPreferencesMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getRetrieveContactPreferencesMethod = ContactServiceGrpc.getRetrieveContactPreferencesMethod) == null) {
          ContactServiceGrpc.getRetrieveContactPreferencesMethod = getRetrieveContactPreferencesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest, com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveContactPreferences"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("RetrieveContactPreferences"))
              .build();
        }
      }
    }
    return getRetrieveContactPreferencesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest,
      com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> getUpdateContactPreferencesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UpdateContactPreferences",
      requestType = com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest.class,
      responseType = com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest,
      com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> getUpdateContactPreferencesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest, com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> getUpdateContactPreferencesMethod;
    if ((getUpdateContactPreferencesMethod = ContactServiceGrpc.getUpdateContactPreferencesMethod) == null) {
      synchronized (ContactServiceGrpc.class) {
        if ((getUpdateContactPreferencesMethod = ContactServiceGrpc.getUpdateContactPreferencesMethod) == null) {
          ContactServiceGrpc.getUpdateContactPreferencesMethod = getUpdateContactPreferencesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest, com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UpdateContactPreferences"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new ContactServiceMethodDescriptorSupplier("UpdateContactPreferences"))
              .build();
        }
      }
    }
    return getUpdateContactPreferencesMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static ContactServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ContactServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ContactServiceStub>() {
        @java.lang.Override
        public ContactServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ContactServiceStub(channel, callOptions);
        }
      };
    return ContactServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static ContactServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ContactServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ContactServiceBlockingV2Stub>() {
        @java.lang.Override
        public ContactServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ContactServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return ContactServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static ContactServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ContactServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ContactServiceBlockingStub>() {
        @java.lang.Override
        public ContactServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ContactServiceBlockingStub(channel, callOptions);
        }
      };
    return ContactServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static ContactServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<ContactServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<ContactServiceFutureStub>() {
        @java.lang.Override
        public ContactServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new ContactServiceFutureStub(channel, callOptions);
        }
      };
    return ContactServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listContacts(com.sdkwork.communication.app.v3.ListContactsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListContactsMethod(), responseObserver);
    }

    /**
     */
    default void listContactTags(com.sdkwork.communication.app.v3.ListContactTagsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactTagsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListContactTagsMethod(), responseObserver);
    }

    /**
     */
    default void createContactTag(com.sdkwork.communication.app.v3.CreateContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactTagResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateContactTagMethod(), responseObserver);
    }

    /**
     */
    default void updateContactTag(com.sdkwork.communication.app.v3.UpdateContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactTagResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateContactTagMethod(), responseObserver);
    }

    /**
     */
    default void deleteContactTag(com.sdkwork.communication.app.v3.DeleteContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteContactTagResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteContactTagMethod(), responseObserver);
    }

    /**
     */
    default void createContactRecommendation(com.sdkwork.communication.app.v3.CreateContactRecommendationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateContactRecommendationMethod(), responseObserver);
    }

    /**
     */
    default void retrieveContactPreferences(com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveContactPreferencesMethod(), responseObserver);
    }

    /**
     */
    default void updateContactPreferences(com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUpdateContactPreferencesMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service ContactService.
   */
  public static abstract class ContactServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return ContactServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service ContactService.
   */
  public static final class ContactServiceStub
      extends io.grpc.stub.AbstractAsyncStub<ContactServiceStub> {
    private ContactServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ContactServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ContactServiceStub(channel, callOptions);
    }

    /**
     */
    public void listContacts(com.sdkwork.communication.app.v3.ListContactsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListContactsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listContactTags(com.sdkwork.communication.app.v3.ListContactTagsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactTagsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListContactTagsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createContactTag(com.sdkwork.communication.app.v3.CreateContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactTagResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateContactTagMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void updateContactTag(com.sdkwork.communication.app.v3.UpdateContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactTagResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateContactTagMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteContactTag(com.sdkwork.communication.app.v3.DeleteContactTagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteContactTagResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteContactTagMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createContactRecommendation(com.sdkwork.communication.app.v3.CreateContactRecommendationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateContactRecommendationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveContactPreferences(com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveContactPreferencesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void updateContactPreferences(com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUpdateContactPreferencesMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service ContactService.
   */
  public static final class ContactServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<ContactServiceBlockingV2Stub> {
    private ContactServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ContactServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ContactServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListContactsResponse listContacts(com.sdkwork.communication.app.v3.ListContactsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListContactsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListContactTagsResponse listContactTags(com.sdkwork.communication.app.v3.ListContactTagsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListContactTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateContactTagResponse createContactTag(com.sdkwork.communication.app.v3.CreateContactTagRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateContactTagResponse updateContactTag(com.sdkwork.communication.app.v3.UpdateContactTagRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteContactTagResponse deleteContactTag(com.sdkwork.communication.app.v3.DeleteContactTagRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateContactRecommendationResponse createContactRecommendation(com.sdkwork.communication.app.v3.CreateContactRecommendationRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateContactRecommendationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse retrieveContactPreferences(com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveContactPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse updateContactPreferences(com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUpdateContactPreferencesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service ContactService.
   */
  public static final class ContactServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<ContactServiceBlockingStub> {
    private ContactServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ContactServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ContactServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListContactsResponse listContacts(com.sdkwork.communication.app.v3.ListContactsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListContactsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListContactTagsResponse listContactTags(com.sdkwork.communication.app.v3.ListContactTagsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListContactTagsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateContactTagResponse createContactTag(com.sdkwork.communication.app.v3.CreateContactTagRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateContactTagResponse updateContactTag(com.sdkwork.communication.app.v3.UpdateContactTagRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteContactTagResponse deleteContactTag(com.sdkwork.communication.app.v3.DeleteContactTagRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteContactTagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateContactRecommendationResponse createContactRecommendation(com.sdkwork.communication.app.v3.CreateContactRecommendationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateContactRecommendationMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse retrieveContactPreferences(com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveContactPreferencesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse updateContactPreferences(com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUpdateContactPreferencesMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service ContactService.
   */
  public static final class ContactServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<ContactServiceFutureStub> {
    private ContactServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected ContactServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new ContactServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListContactsResponse> listContacts(
        com.sdkwork.communication.app.v3.ListContactsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListContactsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListContactTagsResponse> listContactTags(
        com.sdkwork.communication.app.v3.ListContactTagsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListContactTagsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateContactTagResponse> createContactTag(
        com.sdkwork.communication.app.v3.CreateContactTagRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateContactTagMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UpdateContactTagResponse> updateContactTag(
        com.sdkwork.communication.app.v3.UpdateContactTagRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateContactTagMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.DeleteContactTagResponse> deleteContactTag(
        com.sdkwork.communication.app.v3.DeleteContactTagRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteContactTagMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateContactRecommendationResponse> createContactRecommendation(
        com.sdkwork.communication.app.v3.CreateContactRecommendationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateContactRecommendationMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse> retrieveContactPreferences(
        com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveContactPreferencesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse> updateContactPreferences(
        com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUpdateContactPreferencesMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_CONTACTS = 0;
  private static final int METHODID_LIST_CONTACT_TAGS = 1;
  private static final int METHODID_CREATE_CONTACT_TAG = 2;
  private static final int METHODID_UPDATE_CONTACT_TAG = 3;
  private static final int METHODID_DELETE_CONTACT_TAG = 4;
  private static final int METHODID_CREATE_CONTACT_RECOMMENDATION = 5;
  private static final int METHODID_RETRIEVE_CONTACT_PREFERENCES = 6;
  private static final int METHODID_UPDATE_CONTACT_PREFERENCES = 7;

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
        case METHODID_LIST_CONTACTS:
          serviceImpl.listContacts((com.sdkwork.communication.app.v3.ListContactsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactsResponse>) responseObserver);
          break;
        case METHODID_LIST_CONTACT_TAGS:
          serviceImpl.listContactTags((com.sdkwork.communication.app.v3.ListContactTagsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListContactTagsResponse>) responseObserver);
          break;
        case METHODID_CREATE_CONTACT_TAG:
          serviceImpl.createContactTag((com.sdkwork.communication.app.v3.CreateContactTagRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactTagResponse>) responseObserver);
          break;
        case METHODID_UPDATE_CONTACT_TAG:
          serviceImpl.updateContactTag((com.sdkwork.communication.app.v3.UpdateContactTagRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactTagResponse>) responseObserver);
          break;
        case METHODID_DELETE_CONTACT_TAG:
          serviceImpl.deleteContactTag((com.sdkwork.communication.app.v3.DeleteContactTagRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteContactTagResponse>) responseObserver);
          break;
        case METHODID_CREATE_CONTACT_RECOMMENDATION:
          serviceImpl.createContactRecommendation((com.sdkwork.communication.app.v3.CreateContactRecommendationRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateContactRecommendationResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CONTACT_PREFERENCES:
          serviceImpl.retrieveContactPreferences((com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse>) responseObserver);
          break;
        case METHODID_UPDATE_CONTACT_PREFERENCES:
          serviceImpl.updateContactPreferences((com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse>) responseObserver);
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
          getListContactsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListContactsRequest,
              com.sdkwork.communication.app.v3.ListContactsResponse>(
                service, METHODID_LIST_CONTACTS)))
        .addMethod(
          getListContactTagsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListContactTagsRequest,
              com.sdkwork.communication.app.v3.ListContactTagsResponse>(
                service, METHODID_LIST_CONTACT_TAGS)))
        .addMethod(
          getCreateContactTagMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateContactTagRequest,
              com.sdkwork.communication.app.v3.CreateContactTagResponse>(
                service, METHODID_CREATE_CONTACT_TAG)))
        .addMethod(
          getUpdateContactTagMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UpdateContactTagRequest,
              com.sdkwork.communication.app.v3.UpdateContactTagResponse>(
                service, METHODID_UPDATE_CONTACT_TAG)))
        .addMethod(
          getDeleteContactTagMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.DeleteContactTagRequest,
              com.sdkwork.communication.app.v3.DeleteContactTagResponse>(
                service, METHODID_DELETE_CONTACT_TAG)))
        .addMethod(
          getCreateContactRecommendationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateContactRecommendationRequest,
              com.sdkwork.communication.app.v3.CreateContactRecommendationResponse>(
                service, METHODID_CREATE_CONTACT_RECOMMENDATION)))
        .addMethod(
          getRetrieveContactPreferencesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveContactPreferencesRequest,
              com.sdkwork.communication.app.v3.RetrieveContactPreferencesResponse>(
                service, METHODID_RETRIEVE_CONTACT_PREFERENCES)))
        .addMethod(
          getUpdateContactPreferencesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UpdateContactPreferencesRequest,
              com.sdkwork.communication.app.v3.UpdateContactPreferencesResponse>(
                service, METHODID_UPDATE_CONTACT_PREFERENCES)))
        .build();
  }

  private static abstract class ContactServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    ContactServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.ConversationServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("ContactService");
    }
  }

  private static final class ContactServiceFileDescriptorSupplier
      extends ContactServiceBaseDescriptorSupplier {
    ContactServiceFileDescriptorSupplier() {}
  }

  private static final class ContactServiceMethodDescriptorSupplier
      extends ContactServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    ContactServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (ContactServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new ContactServiceFileDescriptorSupplier())
              .addMethod(getListContactsMethod())
              .addMethod(getListContactTagsMethod())
              .addMethod(getCreateContactTagMethod())
              .addMethod(getUpdateContactTagMethod())
              .addMethod(getDeleteContactTagMethod())
              .addMethod(getCreateContactRecommendationMethod())
              .addMethod(getRetrieveContactPreferencesMethod())
              .addMethod(getUpdateContactPreferencesMethod())
              .build();
        }
      }
    }
    return result;
  }
}
