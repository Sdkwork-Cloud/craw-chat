package com.sdkwork.im.backend.api.generated.model;


public class ProblemDetail {
    private String type;
    private String title;
    private Integer status;
    private String detail;
    private String code;
    private String message;
    private String traceId;
    private Boolean retryable;

    public String getType() {
        return this.type;
    }
    
    public void setType(String type) {
        this.type = type;
    }

    public String getTitle() {
        return this.title;
    }
    
    public void setTitle(String title) {
        this.title = title;
    }

    public Integer getStatus() {
        return this.status;
    }
    
    public void setStatus(Integer status) {
        this.status = status;
    }

    public String getDetail() {
        return this.detail;
    }
    
    public void setDetail(String detail) {
        this.detail = detail;
    }

    public String getCode() {
        return this.code;
    }
    
    public void setCode(String code) {
        this.code = code;
    }

    public String getMessage() {
        return this.message;
    }
    
    public void setMessage(String message) {
        this.message = message;
    }

    public String getTraceId() {
        return this.traceId;
    }
    
    public void setTraceId(String traceId) {
        this.traceId = traceId;
    }

    public Boolean getRetryable() {
        return this.retryable;
    }
    
    public void setRetryable(Boolean retryable) {
        this.retryable = retryable;
    }
}
