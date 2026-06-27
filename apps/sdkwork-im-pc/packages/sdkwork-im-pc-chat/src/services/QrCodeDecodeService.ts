import { isSdkworkChatDesktopRuntime } from '@sdkwork/im-pc-core/runtime/desktopEnvironment';

export interface QrCameraScanner {
  stop: () => void;
}

export interface QrCameraScannerOptions {
  deviceId?: string;
  onDecode: (content: string) => void;
  onError?: (error: Error) => void;
  videoElement: HTMLVideoElement;
}

interface BrowserQrCodeReader {
  decodeFromImageUrl(url: string): Promise<{ getText(): string }>;
  decodeFromVideoDevice(
    deviceId: string | undefined,
    videoElement: HTMLVideoElement,
    callback: (result?: { getText(): string }, error?: Error) => void,
  ): Promise<{ stop(): void }>;
}

interface BrowserQrCodeReaderConstructor {
  new (): BrowserQrCodeReader;
}

type TauriInvoke = <T>(command: string, args?: Record<string, unknown>) => Promise<T>;

type TauriBridge = {
  __TAURI__?: {
    core?: {
      invoke?: TauriInvoke;
    };
  };
};

const DESKTOP_CAMERA_SCAN_INTERVAL_MS = 180;
const DESKTOP_CAMERA_MAX_FRAME_WIDTH = 960;
export const MAX_QR_IMAGE_FILE_SIZE_BYTES = 8 * 1024 * 1024;
export const SUPPORTED_QR_IMAGE_FILE_EXTENSION_PATTERN = /\.(?:jpe?g|png|webp)$/iu;
const SUPPORTED_QR_IMAGE_MIME_TYPES = new Set(['image/jpeg', 'image/png', 'image/webp']);

function normalizeDecodedText(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function createDecodeError(message: string): Error {
  return new Error(message);
}

function getTauriInvoke(): TauriInvoke | undefined {
  const bridge = globalThis as TauriBridge;
  return bridge.__TAURI__?.core?.invoke;
}

function stripDataUrlPrefix(value: string): string {
  return value.includes(',') ? value.slice(value.indexOf(',') + 1) : value;
}

async function fileToDataUrl(file: File): Promise<string> {
  return await new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onerror = () => reject(createDecodeError('QR code image cannot be read'));
    reader.onload = () => {
      if (typeof reader.result === 'string') {
        resolve(reader.result);
        return;
      }
      reject(createDecodeError('QR code image cannot be read'));
    };
    reader.readAsDataURL(file);
  });
}

function bytesToBase64(bytes: Uint8ClampedArray): string {
  let binary = '';
  const chunkSize = 0x8000;
  for (let offset = 0; offset < bytes.length; offset += chunkSize) {
    const chunk = bytes.subarray(offset, offset + chunkSize);
    binary += String.fromCharCode(...chunk);
  }
  return btoa(binary);
}

async function loadBrowserQrCodeReader(): Promise<BrowserQrCodeReader> {
  const module = await import('@zxing/browser') as {
    BrowserQRCodeReader: BrowserQrCodeReaderConstructor;
  };
  return new module.BrowserQRCodeReader();
}

export function isSupportedQrImageFile(file: File): boolean {
  if (file.size > MAX_QR_IMAGE_FILE_SIZE_BYTES) {
    return false;
  }

  const mimeType = file.type.trim().toLowerCase();
  if (mimeType) {
    return SUPPORTED_QR_IMAGE_MIME_TYPES.has(mimeType);
  }

  return SUPPORTED_QR_IMAGE_FILE_EXTENSION_PATTERN.test(file.name);
}

export async function decodeQrCodeWithDesktopNative(imageBase64: string): Promise<string | null> {
  const invoke = getTauriInvoke();
  if (!invoke) {
    throw createDecodeError('Native QR decoder is unavailable');
  }
  const decodedText = await invoke<string | null>('sdkwork_chat_pc_decode_qr_code_image', {
    request: {
      imageBase64: stripDataUrlPrefix(imageBase64),
    },
  });
  return normalizeDecodedText(decodedText) ?? null;
}

async function decodeQrCodeRgbaWithDesktopNative(
  dataBase64: string,
  width: number,
  height: number,
): Promise<string | null> {
  const invoke = getTauriInvoke();
  if (!invoke) {
    throw createDecodeError('Native QR decoder is unavailable');
  }
  const decodedText = await invoke<string | null>('sdkwork_chat_pc_decode_qr_code_rgba', {
    request: {
      dataBase64,
      height,
      width,
    },
  });
  return normalizeDecodedText(decodedText) ?? null;
}

export async function decodeQrCodeFromImageFile(file: File): Promise<string> {
  if (!isSupportedQrImageFile(file)) {
    throw createDecodeError('QR code image file is required');
  }

  if (isSdkworkChatDesktopRuntime()) {
    const decodedText = await decodeQrCodeWithDesktopNative(await fileToDataUrl(file));
    if (decodedText) {
      return decodedText;
    }
    throw createDecodeError('QR code image cannot be decoded');
  }

  const imageUrl = URL.createObjectURL(file);
  const reader = await loadBrowserQrCodeReader();
  try {
    const result = await reader.decodeFromImageUrl(imageUrl);
    const decodedText = normalizeDecodedText(result.getText());
    if (!decodedText) {
      throw createDecodeError('QR code content is empty');
    }
    return decodedText;
  } catch (error) {
    throw error instanceof Error
      ? error
      : createDecodeError('QR code image cannot be decoded');
  } finally {
    URL.revokeObjectURL(imageUrl);
  }
}

function createDesktopCameraCanvas(videoElement: HTMLVideoElement): HTMLCanvasElement {
  const canvas = document.createElement('canvas');
  const sourceWidth = videoElement.videoWidth || videoElement.clientWidth || 640;
  const sourceHeight = videoElement.videoHeight || videoElement.clientHeight || 480;
  const scale = Math.min(1, DESKTOP_CAMERA_MAX_FRAME_WIDTH / sourceWidth);
  canvas.width = Math.max(1, Math.round(sourceWidth * scale));
  canvas.height = Math.max(1, Math.round(sourceHeight * scale));
  return canvas;
}

async function createDesktopQrCameraScanner(options: QrCameraScannerOptions): Promise<QrCameraScanner> {
  let isStopped = false;
  let isDecoding = false;
  let animationFrameId: number | undefined;
  let stream: MediaStream | undefined;
  let lastScanAt = 0;
  let canvas: HTMLCanvasElement | undefined;

  stream = await navigator.mediaDevices.getUserMedia({
    audio: false,
    video: options.deviceId ? { deviceId: { exact: options.deviceId } } : true,
  });
  options.videoElement.srcObject = stream;
  await options.videoElement.play();

  const scanFrame = () => {
    if (isStopped) {
      return;
    }
    animationFrameId = requestAnimationFrame(scanFrame);
    const now = Date.now();
    if (isDecoding || now - lastScanAt < DESKTOP_CAMERA_SCAN_INTERVAL_MS) {
      return;
    }
    if (!options.videoElement.videoWidth || !options.videoElement.videoHeight) {
      return;
    }

    lastScanAt = now;
    isDecoding = true;
    canvas ??= createDesktopCameraCanvas(options.videoElement);
    const context = canvas.getContext('2d', { willReadFrequently: true });
    if (!context) {
      isDecoding = false;
      options.onError?.(createDecodeError('Camera frame cannot be captured'));
      return;
    }

    context.drawImage(options.videoElement, 0, 0, canvas.width, canvas.height);
    const frame = context.getImageData(0, 0, canvas.width, canvas.height);
    void decodeQrCodeRgbaWithDesktopNative(bytesToBase64(frame.data), canvas.width, canvas.height)
      .then((decodedText) => {
        if (decodedText && !isStopped) {
          options.onDecode(decodedText);
        }
      })
      .catch((error) => {
        if (!isStopped) {
          options.onError?.(error instanceof Error ? error : createDecodeError('Camera QR frame cannot be decoded'));
        }
      })
      .finally(() => {
        isDecoding = false;
      });
  };

  animationFrameId = requestAnimationFrame(scanFrame);

  return {
    stop: () => {
      isStopped = true;
      if (animationFrameId !== undefined) {
        cancelAnimationFrame(animationFrameId);
      }
      for (const track of stream?.getTracks() ?? []) {
        track.stop();
      }
      options.videoElement.pause();
      options.videoElement.srcObject = null;
    },
  };
}

async function createBrowserQrCameraScanner(options: QrCameraScannerOptions): Promise<QrCameraScanner> {
  const reader = await loadBrowserQrCodeReader();
  let isStopped = false;
  const controls = await reader.decodeFromVideoDevice(
    options.deviceId,
    options.videoElement,
    (result, error) => {
      if (isStopped) {
        return;
      }
      const decodedText = normalizeDecodedText(result?.getText());
      if (decodedText) {
        options.onDecode(decodedText);
        return;
      }
      if (error && options.onError && error.name !== 'NotFoundException') {
        options.onError(error);
      }
    },
  );

  return {
    stop: () => {
      isStopped = true;
      controls.stop();
      options.videoElement.srcObject = null;
    },
  };
}

export async function createQrCameraScanner(options: QrCameraScannerOptions): Promise<QrCameraScanner> {
  if (isSdkworkChatDesktopRuntime()) {
    return createDesktopQrCameraScanner(options);
  }

  return createBrowserQrCameraScanner(options);
}
