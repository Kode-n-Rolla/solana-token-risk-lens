export type DataSourceStatus = {
    source: string;
    status: string;
    detail: string | null;
};

export type AnalyzeTokenResponse = {
    tokenAddress: string;
    chain: string;
    name: string | null;
    symbol: string | null;
    logoUri: string | null;
    price: number | null;
    liquidity: number | null;
    dataSources: DataSourceStatus[];
    message: string;
};

export type ApiErrorResponse = {
    error: string;
};