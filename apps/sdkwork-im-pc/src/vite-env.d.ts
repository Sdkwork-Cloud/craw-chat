/// <reference types="vite/client" />

declare module '*.css';

declare module 'react-qr-code';
declare module 'tiptap-markdown';

declare module 'express' {
  export interface Request {
    body: unknown;
  }

  export interface Response {
    json(body: unknown): void;
    sendFile(path: string): void;
    status(code: number): Response;
  }

  export interface ExpressApp {
    get(path: string, handler: (req: Request, res: Response) => unknown): void;
    listen(port: number, host: string, callback: () => void): void;
    post(path: string, handler: (req: Request, res: Response) => unknown): void;
    use(...args: unknown[]): void;
  }

  export interface ExpressFactory {
    (): ExpressApp;
    json(): unknown;
    static(root: string): unknown;
  }

  const express: ExpressFactory;
  export default express;
}

declare module 'signature_pad' {
  export interface SignaturePadOptions {
    backgroundColor?: string;
    maxWidth?: number;
    minWidth?: number;
    penColor?: string;
    velocityFilterWeight?: number;
  }

  export default class SignaturePad {
    penColor: string;

    constructor(canvas: HTMLCanvasElement, options?: SignaturePadOptions);

    addEventListener(type: string, listener: () => void): void;
    clear(): void;
    fromData(data: unknown[]): void;
    isEmpty(): boolean;
    toData(): unknown[];
    toDataURL(type?: string): string;
  }
}
