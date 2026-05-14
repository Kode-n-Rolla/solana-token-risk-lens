type RedFlagsProps = {
  flags: string[];
};

export function RedFlags({ flags }: RedFlagsProps) {
  return (
    <section className="panel">
      <p className="eyebrow">Red flags</p>
      {flags.length > 0 ? (
        <ul className="content-list">
          {flags.map((flag) => (
            <li key={flag}>{flag}</li>
          ))}
        </ul>
      ) : (
        <p className="helper-text">
          No major red flags were triggered by the current input data.
        </p>
      )}
    </section>
  );
}
