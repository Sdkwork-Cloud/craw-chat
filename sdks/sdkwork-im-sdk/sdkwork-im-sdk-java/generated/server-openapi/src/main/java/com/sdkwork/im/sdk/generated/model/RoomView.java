package com.sdkwork.im.sdk.generated.model;


public class RoomView {
    private String roomId;
    private String roomKind;
    private String conversationId;
    private Integer activeMemberCount;
    private Integer maxMembers;

    public String getRoomId() {
        return this.roomId;
    }

    public void setRoomId(String roomId) {
        this.roomId = roomId;
    }

    public String getRoomKind() {
        return this.roomKind;
    }

    public void setRoomKind(String roomKind) {
        this.roomKind = roomKind;
    }

    public String getConversationId() {
        return this.conversationId;
    }

    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public Integer getActiveMemberCount() {
        return this.activeMemberCount;
    }

    public void setActiveMemberCount(Integer activeMemberCount) {
        this.activeMemberCount = activeMemberCount;
    }

    public Integer getMaxMembers() {
        return this.maxMembers;
    }

    public void setMaxMembers(Integer maxMembers) {
        this.maxMembers = maxMembers;
    }
}
