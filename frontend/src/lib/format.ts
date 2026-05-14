export function formatPrice(value: number | null): string {
    if (value === null) {
        return "Unavailable";
    }

    if (value >= 1) {
        return `$${value.toFixed(0)}`;
    }

    if (value >= 0.01) {
        return `$${value.toFixed(4)}`;
    }

    return `$${value.toFixed(8)}`;
}

export function formatUsdCompact(value: number | null): string {
    if (value === null) {
        return "Unavailable";
    }

    return new Intl.NumberFormat("en-US", {
        style: "currency",
        currency: "USD",
        notation: "compact",
        maximumFractionDigits: 2,
    }).format(value);
}

export function formatPercent(value: number): string {
    return `${value.toFixed(2)}%`;
}