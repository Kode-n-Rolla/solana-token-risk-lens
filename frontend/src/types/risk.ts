export type DataSourceStatus = {
  source: string;
  status: string;
  detail: string | null;
};

export type RiskComponent = {
  score: number;
  maxScore: number;
  level: string;
  flags: string[];
  summary: string;
}

export type HolderMetrics = {
  top1Percent: number;
  top5Percent: number;
  top10Percent: number;
}

export type RiskBreakdown = {
  holders: RiskComponent | null;
  liquidity: RiskComponent;
  momentum: RiskComponent;
  context: RiskComponent;
}

export type AnalyzeTokenResponse = {
  tokenAddress: string;
  chain: string;
  name: string | null;
  symbol: string | null;
  logoUri: string | null;
  price: number | null;
  liquidity: number | null;
  riskIndex: number;
  riskLevel: string;
  summary: string;
  holderMetrics: HolderMetrics | null;
  breakdown: RiskBreakdown;
  redFlags: string[];
  manualChecks: string[];
  dataSources: DataSourceStatus[];
  message: string;
};

export type ApiErrorResponse = {
  error: string;
};

export type SourceProbeResponse = {
  source: string;
  status: string;
  detail: string | null;
  tokenAddress: string;
  chain: string;
  message: string;
};
