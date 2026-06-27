package com.sdkwork.im.sdk.generated.model;


public class MediaContentPart extends ContentPart {
    private String kind;
    private DriveReference drive;
    private MediaResource resource;
    private String mediaRole;

    public String getKind() {
        return this.kind;
    }

    public void setKind(String kind) {
        this.kind = kind;
    }

    public DriveReference getDrive() {
        return this.drive;
    }

    public void setDrive(DriveReference drive) {
        this.drive = drive;
    }

    public MediaResource getResource() {
        return this.resource;
    }

    public void setResource(MediaResource resource) {
        this.resource = resource;
    }

    public String getMediaRole() {
        return this.mediaRole;
    }

    public void setMediaRole(String mediaRole) {
        this.mediaRole = mediaRole;
    }
}
