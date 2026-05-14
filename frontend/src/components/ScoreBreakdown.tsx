import type { RiskBreakdown } from "../types/risk";

type ScoreBreakdownProps = {
  breakdown: RiskBreakdown;
};

export function ScoreBreakdown({ breakdown }: ScoreBreakdownProps) {
  return (
    <section className="panel">
      <p className="eyebrow">Score breakdown</p>
      <div className="breakdown-grid">
        {breakdown.holders ? (
          <article className="breakdown-card">
            <h3>Holders</h3>
            <p>
              {breakdown.holders.score}/{breakdown.holders.maxScore} ·{" "}
              {breakdown.holders.level}
            </p>
            <p>{breakdown.holders.summary}</p>
          </article>
        ) : null}

        <article className="breakdown-card">
          <h3>Liquidity</h3>
          <p>
            {breakdown.liquidity.score}/{breakdown.liquidity.maxScore} ·{" "}
            {breakdown.liquidity.level}
          </p>
          <p>{breakdown.liquidity.summary}</p>
        </article>

        <article className="breakdown-card">
          <h3>Momentum</h3>
          <p>
            {breakdown.momentum.score}/{breakdown.momentum.maxScore} ·{" "}
            {breakdown.momentum.level}
          </p>
          <p>{breakdown.momentum.summary}</p>
        </article>

        <article className="breakdown-card">
          <h3>Context</h3>
          <p>
            {breakdown.context.score}/{breakdown.context.maxScore} ·{" "}
            {breakdown.context.level}
          </p>
          <p>{breakdown.context.summary}</p>
        </article>
      </div>
    </section>
  );
}
