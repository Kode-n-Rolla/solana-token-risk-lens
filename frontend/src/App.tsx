import { useState } from "react";
import "./App.css";
import { analyzeToken, probeOverview, probePrice } from "./lib/api";
import { RiskSummary } from "./components/RiskSummary";
import { ScoreBreakdown } from "./components/ScoreBreakdown";
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
        <h1>Unsure about a token? Review the signals.</h1>
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
              placeholder="DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263"
            />
          </label>

          <p className="helper-text">
            Your API key is used only for this analysis request and is not
            stored by the app.
          </p>

          <div className="button-row">
            <button type="submit" disabled={activeRequest !== null}>
              {activeRequest === "analyze" ? (
                <span className="button-content">
                  <span className="button-spinner" aria-hidden="true" />
                  <span>Analyzing...</span>
                </span>
              ) : (
                "Analyze token"
              )}
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
        <>
          <section className="panel report-panel">
            <RiskSummary
              name={report.name}
              symbol={report.symbol}
              logoUri={report.logoUri}
              riskIndex={report.riskIndex}
              riskLevel={report.riskLevel}
              price={report.price}
              liquidity={report.liquidity}
              summary={report.summary}
            />
          </section>

          <section className="panel">
            <p className="eyebrow">Score breakdown</p>
              <ScoreBreakdown breakdown={report.breakdown} />            
          </section>

          {report.holderMetrics ? (
            <section className="panel">
              <p className="eyebrow">Holder concentration</p>
              <div className="stats-grid">
                <article className="stat-card">
                  <span>Top 1</span>
                  <strong>{report.holderMetrics.top1Percent.toFixed(2)}%</strong>
                </article>

                <article className="stat-card">
                  <span>Top 5</span>
                  <strong>{report.holderMetrics.top5Percent.toFixed(2)}%</strong>
                </article>

                <article className="stat-card">
                  <span>Top 10</span>
                  <strong>{report.holderMetrics.top10Percent.toFixed(2)}%</strong>
                </article>
              </div>
            </section>
          ) : null}

          <section className="panel">
            <p className="eyebrow">Red flags</p>
            {report.redFlags.length > 0 ? (
              <ul className="content-list">
                {report.redFlags.map((flag) => (
                  <li key={flag}>{flag}</li>
                ))}
              </ul>
            ) : (
              <p className="helper-text">
                No major red flags were triggered by the current input data.
              </p>
            )}
          </section>

          <section className="panel">
            <p className="eyebrow">Manual checks</p>
            <ul className="content-list">
              {report.manualChecks.map((check) => (
                <li key={check}>{check}</li>
              ))}
            </ul>
          </section>

          <section className="panel">
            <p className="eyebrow">Data sources</p>
            <ul className="content-list">
              {report.dataSources.map((source) => (
                <li key={source.source}>
                  <strong>{source.source}</strong>: {source.status}
                  {source.detail ? ` - ${source.detail}` : ""}
                </li>
              ))}
            </ul>
          </section>
        </>
      ) : null}
    </main>
  );
}

export default App;
