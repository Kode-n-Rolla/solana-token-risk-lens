import "./App.css";
import { useState } from "react";
import { analyzeToken } from "./lib/api";
import { RiskSummary } from "./components/RiskSummary";
import { ScoreBreakdown } from "./components/ScoreBreakdown";
import { HolderMetrics } from "./components/HolderMetrics";
import { RedFlags } from "./components/RedFlags";
import { ManualChecks } from "./components/ManualChecks";
import { DataSources } from "./components/DataSources";
import { Disclaimer } from "./components/Disclaimer";
import type { AnalyzeTokenResponse } from "./types/risk";

function App() {
  const [apiKey, setApiKey] = useState("");
  const [tokenAddress, setTokenAddress] = useState("");
  const [report, setReport] = useState<AnalyzeTokenResponse | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [activeRequest, setActiveRequest] = useState<
    "analyze" | "price" | "overview" | null
  >(null);

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setActiveRequest("analyze");
    setError(null);
    setReport(null);

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
          </div>
        </form>
      </section>

      {error ? (
        <section className="panel error-panel">
          <h2>Request error</h2>
          <p>{error}</p>
        </section>
      ) : null}

      {report ? (
        <div className="report-stack">
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

          <ScoreBreakdown breakdown={report.breakdown} />            

          {report.holderMetrics ? (
            <HolderMetrics metrics={report.holderMetrics} />
          ) : null}

          <RedFlags flags={report.redFlags}/>

          <ManualChecks checks={report.manualChecks} />

          <DataSources sources={report.dataSources} />

          <Disclaimer />
        </div>
      ) : null}
    </main>
  );
}

export default App;
