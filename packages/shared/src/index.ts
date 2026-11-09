export const APP_NAME = 'MULX';
export const API_BASE_URL = '/api/v1';

export type RuntimeConfig = {
  apiBaseUrl: string;
  appName: string;
};

export const defaultRuntimeConfig: RuntimeConfig = {
  apiBaseUrl: API_BASE_URL,
  appName: APP_NAME
};
