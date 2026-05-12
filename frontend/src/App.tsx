import { useState } from "react";
import "./App.css";

type DataSourceStatus = {
  source: string;
  status: string;
  detail: string | null;
};

type AnalyzeTokenResponse = {
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

type ApiErrorResponse = {
  error: string;
};

function App() {
  const [apiKey, setApiKey] = useState("");
  const [tokenAddress, setTokenAddress] = useState("");
  const [report, setReport] = useState<AnalyzeTokenResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setIsLoading(true);
    setError(null);
    setReport(null);

    try {
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

      const result = (await response.json()) as AnalyzeTokenResponse;
      setReport(result);
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("Unexpected error while analyzing token");
      }
    } finally {
      setIsLoading(false);
    }
  }

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Solana Token Risk Lens</p>
        <h1>Rust backend thinks. React UI shows.</h1>
        <p className="hero-copy">
          A decision-support dashboard for reviewing observable token risk
          signals from Birdeye data.
        </p>
      </section>

      <section className="panel">
        <form className="analyze-form" onSubmit={handleSubmit}>
          <label>
            <span>Birdeye API key</span>
            <input
              type="password"
              value={apiKey}
              onChange={(event) => setApiKey(event.target.value)}
              placeholder="Enter your Birdeye API key"
            />
          </label>

          <label>
            <span>Solana token mint</span>
            <input
              type="text"
              value={tokenAddress}
              onChange={(event) => setTokenAddress(event.target.value)}
              placeholder="So11111111111111111111111111111111111111112"
            />
          </label>

          <p className="helper-text">
            Your API key is used only for this analysis request and is not
            stored by the app.
          </p>

          <button type="submit" disabled={isLoading}>
            {isLoading ? "Analyzing..." : "Analyze token"}
          </button>
        </form>
      </section>

      {error ? (
        <section className="panel error-panel">
          <h2>Request error</h2>
          <p>{error}</p>
        </section>
      ) : null}

      {report ? (
        <section className="panel report-panel">
          <div className="report-header">
            <div>
              <p className="eyebrow">Basic token data</p>
              <h2>{report.name ?? "Unknown token"}</h2>
              <p>{report.symbol ?? "Symbol unavailable"}</p>
            </div>

            {report.logoUri ? (
              <img
                className="token-logo"
                src={report.logoUri}
                alt={`${report.name ?? "Token"} logo`}
              />
            ) : null}
          </div>

          <div className="stats-grid">
            <article className="stat-card">
              <span>Token address</span>
              <strong>{report.tokenAddress}</strong>
            </article>

            <article className="stat-card">
              <span>Price</span>
              <strong>
                {report.price !== null ? report.price.toString() : "Unavailable"}
              </strong>
            </article>

            <article className="stat-card">
              <span>Liquidity</span>
              <strong>
                {report.liquidity !== null
                  ? report.liquidity.toString()
                  : "Unavailable"}
              </strong>
            </article>

            <article className="stat-card">
              <span>Chain</span>
              <strong>{report.chain}</strong>
            </article>
          </div>

          <div className="data-source-block">
            <h3>Data sources</h3>
            <ul>
              {report.dataSources.map((source) => (
                <li key={source.source}>
                  <strong>{source.source}</strong>: {source.status}
                  {source.detail ? ` — ${source.detail}` : ""}
                </li>
              ))}
            </ul>
          </div>

          <p className="helper-text">{report.message}</p>
        </section>
      ) : null}
    </main>
  );
}

export default App;
