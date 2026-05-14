import type { DataSourceStatus } from "../types/risk";

type DataSourcesProps = {
  sources: DataSourceStatus[];
};

export function DataSources({ sources }: DataSourcesProps) {
  return (
    <section className="panel">
      <p className="eyebrow">Data sources</p>
      <ul className="content-list">
        {sources.map((source) => (
          <li key={source.source}>
            <strong>{source.source}</strong>: {source.status}
            {source.detail ? ` - ${source.detail}` : ""}
          </li>
        ))}
      </ul>
    </section>
  );
}
