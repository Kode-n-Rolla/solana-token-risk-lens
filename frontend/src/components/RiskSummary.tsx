import { formatPrice, formatUsdCompact } from "../lib/format";

type RiskSummaryProps = {
    name: string | null;
    symbol: string |null;
    logoUri: string |null;
    riskIndex: number;
    riskLevel: string;
    price: number | null;
    liquidity: number | null;
    summary: string;
};

export function RiskSummary({
    name,
    symbol,
    logoUri,
    riskIndex,
    riskLevel,
    price,
    liquidity,
    summary,
}: RiskSummaryProps) {
    return (
    <section className="panel report-panel">
      <div className="report-header">
        <div>
          <p className="eyebrow">Token report</p>
          <h2>{name ?? "Unknown token"}</h2>
          <p>{symbol ?? "Symbol unavailable"}</p>
        </div>

        {logoUri ? (
          <img
            className="token-logo"
            src={logoUri}
            alt={`${name ?? "Token"} logo`}
          />
        ) : null}
      </div>

      <div className="hero-risk-card">
        <div>
          <p className="eyebrow">Risk Index</p>
          <div className="hero-risk-score">{riskIndex}</div>
        </div>

        <div>
          <p className="eyebrow">Risk Level</p>
          <p className="hero-risk-level">{riskLevel}</p>
        </div>
      </div>

      <div className="stats-grid">
        <article className="stat-card">
          <span>Price</span>
          <strong>{formatPrice(price)}</strong>
        </article>

        <article className="stat-card">
          <span>Liquidity</span>
          <strong>{formatUsdCompact(liquidity)}</strong>
        </article>
      </div>

      <p className="report-summary">{summary}</p>
    </section>
    );
}