import type {
  AnalyzeTokenResponse,
  ApiErrorResponse,
  SourceProbeResponse,
} from "../types/risk";

type AnalyzeTokenInput = {
    apiKey: string;
    tokenAddress: string;
};

export async function analyzeToken({
  apiKey,
  tokenAddress,
}: AnalyzeTokenInput): Promise<AnalyzeTokenResponse> {
  return postJson<AnalyzeTokenResponse>("/api/analyze-token", {
    apiKey,
    tokenAddress,
    chain: "solana",
    options: {
      includeHolders: true,
      holderLimit: 100,
    },
  });
}

export async function probePrice({
  apiKey,
  tokenAddress,
}: AnalyzeTokenInput): Promise<SourceProbeResponse> {
  return postJson<SourceProbeResponse>("/api/probe-price", {
    apiKey,
    tokenAddress,
    chain: "solana",
    options: {
      includeHolders: true,
      holderLimit: 100,
    },
  });
}

export async function probeOverview({
  apiKey,
  tokenAddress,
}: AnalyzeTokenInput): Promise<SourceProbeResponse> {
  return postJson<SourceProbeResponse>("/api/probe-overview", {
    apiKey,
    tokenAddress,
    chain: "solana",
    options: {
      includeHolders: true,
      holderLimit: 100,
    },
  });
}

async function postJson<T>(
  path: string,
  body: AnalyzeTokenInput & {
    chain: string;
    options: {
      includeHolders: boolean;
      holderLimit: number;
    };
  },
): Promise<T> {
  const response = await fetch(`http://localhost:3001${path}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    const errorBody = (await response.json()) as ApiErrorResponse;
    throw new Error(errorBody.error || "Request failed");
  }

  return (await response.json()) as T;
}
