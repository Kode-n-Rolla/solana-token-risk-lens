import type { HolderMetrics as HolderMetricsData } from "../types/risk";
import { formatPercent } from "../lib/format";

type HolderMetricsProps = {
  metrics: HolderMetricsData;
};

export function HolderMetrics({ metrics }: HolderMetricsProps) {
  return (
    <section className="panel">
      <p className="eyebrow">Holder concentration</p>
      <div className="holder-metrics-grid">
        <article className="stat-card">
          <span>Top 1</span>
          <strong>{formatPercent(metrics.top1Percent)}</strong>
        </article>

        <article className="stat-card">
          <span>Top 5</span>
          <strong>{formatPercent(metrics.top5Percent)}</strong>
        </article>

        <article className="stat-card">
          <span>Top 10</span>
          <strong>{formatPercent(metrics.top10Percent)}</strong>
        </article>
      </div>
    </section>
  );
}
