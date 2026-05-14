type ManualChecksProps = {
    checks: string[];
};

export function ManualChecks({ checks }: ManualChecksProps) {
  return (
    <section className="panel">
      <p className="eyebrow">Manual checks</p>
      <ul className="content-list">
        {checks.map((check) => (
          <li key={check}>{check}</li>
        ))}
      </ul>
    </section>
  );    
}