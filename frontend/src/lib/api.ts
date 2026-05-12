import type { AnalyzeTokenResponse, ApiErrorResponse } from "../types/risk";

type AnalyzeTokenInput = {
    apiKey: string;
    tokenAddress: string;
};

export async function analyzeToken({
  apiKey,
  tokenAddress,
}: AnalyzeTokenInput): Promise<AnalyzeTokenResponse> {
  const response = await fetch("http://localhost:3001/api/analyze-token", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      apiKey,
      tokenAddress,
      chain: "solana",
      options: {
        includeHolders: true,
        holderLimit: 100,
      },
    }),
  });

if (!response.ok) {
    const errorBody = (await response.json()) as ApiErrorResponse;
    throw new Error(errorBody.error || "Request failed");
}

return (await response.json()) as AnalyzeTokenResponse;
}