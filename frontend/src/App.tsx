import { useState } from "react";
import "./App.css";
import { analyzeToken, probeOverview, probePrice } from "./lib/api";
import type { AnalyzeTokenResponse, SourceProbeResponse } from "./types/risk";

function App() {
  const [apiKey, setApiKey] = useState("");
  const [tokenAddress, setTokenAddress] = useState("");
  const [report, setReport] = useState<AnalyzeTokenResponse | null>(null);
  const [probeResult, setProbeResult] = useState<SourceProbeResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [activeRequest, setActiveRequest] = useState<
    "analyze" | "price" | "overview" | null
  >(null);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setActiveRequest("analyze");
    setError(null);
    setReport(null);
    setProbeResult(null);

    try {
      const result = await analyzeToken({ apiKey, tokenAddress });
      setReport(result);
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError("Unexpected error while analyzing token");
      }
    } finally {
      setActiveRequest(null);
    }
  }

  async function handleProbe(source: "price" | "overview") {
    setActiveRequest(source);
    setError(null);
    setProbeResult(null);

    try {
      const result =
        source === "price"
          ? await probePrice({ apiKey, tokenAddress })
          : await probeOverview({ apiKey, tokenAddress });

      setProbeResult(result);
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else {
        setError(`Unexpected error while probing ${source}`);
      }
    } finally {
      setActiveRequest(null);
    }
  }

  return (
    <main className="app-shell">
      <section className="hero">
        <p className="eyebrow">Solana Token Risk Lens</p>
        <h1>Unsure about a token? Review the signals before you trade or interact.</h1>
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

          <div className="button-row">
            <button type="submit" disabled={activeRequest !== null}>
              {activeRequest === "analyze" ? "Analyzing..." : "Analyze token"}
            </button>

            <button
              type="button"
              className="secondary-button"
              disabled={activeRequest !== null}
              onClick={() => void handleProbe("price")}
            >
              {activeRequest === "price" ? "Testing price..." : "Test price"}
            </button>

            <button
              type="button"
              className="secondary-button"
              disabled={activeRequest !== null}
              onClick={() => void handleProbe("overview")}
            >
              {activeRequest === "overview"
                ? "Testing overview..."
                : "Test overview"}
            </button>
          </div>
        </form>
      </section>

      {error ? (
        <section className="panel error-panel">
          <h2>Request error</h2>
          <p>{error}</p>
        </section>
      ) : null}

      {probeResult ? (
        <section className="panel">
          <p className="eyebrow">Source probe</p>
          <h2 className="probe-title">{probeResult.source}</h2>
          <p className="helper-text">{probeResult.message}</p>

          <div className="stats-grid">
            <article className="stat-card">
              <span>Status</span>
              <strong>{probeResult.status}</strong>
            </article>

            <article className="stat-card">
              <span>Chain</span>
              <strong>{probeResult.chain}</strong>
            </article>
          </div>

          {probeResult.detail ? (
            <p className="helper-text">{probeResult.detail}</p>
          ) : (
            <p className="helper-text">
              The standalone endpoint request completed without an upstream
              error.
            </p>
          )}
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
                  {source.detail ? ` - ${source.detail}` : ""}
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
