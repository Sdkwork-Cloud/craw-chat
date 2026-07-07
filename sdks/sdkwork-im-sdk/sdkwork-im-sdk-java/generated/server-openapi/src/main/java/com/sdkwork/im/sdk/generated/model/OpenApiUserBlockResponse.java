package com.sdkwork.im.sdk.generated.model;


public class OpenApiUserBlockResponse {
    private UserBlock userBlock;
    private CommitEnvelopeResponse latestCommit;
    private SocialWritePersistence persistence;

    public UserBlock getUserBlock() {
        return this.userBlock;
    }

    public void setUserBlock(UserBlock userBlock) {
        this.userBlock = userBlock;
    }

    public CommitEnvelopeResponse getLatestCommit() {
        return this.latestCommit;
    }

    public void setLatestCommit(CommitEnvelopeResponse latestCommit) {
        this.latestCommit = latestCommit;
    }

    public SocialWritePersistence getPersistence() {
        return this.persistence;
    }

    public void setPersistence(SocialWritePersistence persistence) {
        this.persistence = persistence;
    }
}
