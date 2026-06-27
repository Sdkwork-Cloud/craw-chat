package com.sdkwork.im.sdk.generated.model;


public class SocialFriendRequestAcceptanceResponse {
    private FriendRequest friendRequest;
    private Friendship friendship;
    private DirectChat directChat;
    private CreateConversationResult conversation;

    public FriendRequest getFriendRequest() {
        return this.friendRequest;
    }

    public void setFriendRequest(FriendRequest friendRequest) {
        this.friendRequest = friendRequest;
    }

    public Friendship getFriendship() {
        return this.friendship;
    }

    public void setFriendship(Friendship friendship) {
        this.friendship = friendship;
    }

    public DirectChat getDirectChat() {
        return this.directChat;
    }

    public void setDirectChat(DirectChat directChat) {
        this.directChat = directChat;
    }

    public CreateConversationResult getConversation() {
        return this.conversation;
    }

    public void setConversation(CreateConversationResult conversation) {
        this.conversation = conversation;
    }
}
