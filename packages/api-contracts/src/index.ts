export type ApiEnvelope<T> = {
  ok: boolean;
  data: T;
};

export type HealthPayload = {
  status: 'ok';
  service: string;
  version: string;
};

export type ReadinessPayload = {
  ready: boolean;
  database: boolean;
  storage: boolean;
};

export type HealthResponse = ApiEnvelope<HealthPayload>;
export type ReadinessResponse = ApiEnvelope<ReadinessPayload>;
