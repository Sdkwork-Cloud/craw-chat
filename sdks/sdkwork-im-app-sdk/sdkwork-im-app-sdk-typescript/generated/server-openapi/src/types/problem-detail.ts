export interface ProblemDetail {
  type: string;
  title: string;
  status: number;
  detail: string;
  code?: string;
  message?: string;
  traceId?: string;
  retryable?: boolean;
}
