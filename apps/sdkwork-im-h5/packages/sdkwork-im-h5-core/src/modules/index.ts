export const IM_H5_SDK_MODULES = ['drive', 'notary'] as const;

export type ImH5SdkModule = (typeof IM_H5_SDK_MODULES)[number];
