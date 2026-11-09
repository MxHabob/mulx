import type { HealthResponse, ReadinessResponse } from '@mulx/api-contracts';

export async function getHealth(baseUrl: string): Promise<HealthResponse> {
  const response = await fetch(`${baseUrl.replace(/\/$/, '')}/api/v1/health`);
  if (!response.ok) {
    throw new Error(`Health request failed: ${response.status}`);
  }
  return (await response.json()) as HealthResponse;
}

export async function getReadiness(baseUrl: string): Promise<ReadinessResponse> {
  const response = await fetch(`${baseUrl.replace(/\/$/, '')}/api/v1/ready`);
  if (!response.ok) {
    throw new Error(`Readiness request failed: ${response.status}`);
  }
  return (await response.json()) as ReadinessResponse;
}
