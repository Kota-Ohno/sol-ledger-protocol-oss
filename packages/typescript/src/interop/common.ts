export type LossReason =
  | "invalid"
  | "not_mapped"
  | "privacy"
  | "required_for_round_trip";

export interface LossItem {
  path: string;
  reason: LossReason;
  detail: string;
}

export interface AdapterResult<T> {
  value?: T;
  losses: LossItem[];
  errors: string[];
}

export function isTraceId(value: string): boolean {
  return /^[0-9a-f]{32}$/.test(value) && value !== "00000000000000000000000000000000";
}

export function isSpanId(value: string): boolean {
  return /^[0-9a-f]{16}$/.test(value) && value !== "0000000000000000";
}

export interface ParsedTimestamp {
  unixNano: string;
  canonical: string;
}

export const OTEL_UINT64_MAX = 18_446_744_073_709_551_615n;

export function parseTimestamp(value: string, path: string, errors: string[]): ParsedTimestamp | undefined {
  const match = /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})(?:\.(\d{1,9}))?(Z|([+-])(\d{2}):(\d{2}))$/.exec(value);
  if (!match) {
    errors.push(`${path} must be a complete RFC 3339 timestamp with at most 9 fractional digits`);
    return undefined;
  }
  const [, yearText, monthText, dayText, hourText, minuteText, secondText, fraction = "", zone, sign, offsetHourText = "0", offsetMinuteText = "0"] = match;
  const [year, month, day, hour, minute, second, offsetHour, offsetMinute] = [yearText, monthText, dayText, hourText, minuteText, secondText, offsetHourText, offsetMinuteText].map(Number);
  const daysInMonth = new Date(Date.UTC(year, month, 0)).getUTCDate();
  if (year === 0 || month < 1 || month > 12 || day < 1 || day > daysInMonth || hour > 23 || minute > 59 || second > 59 || offsetHour > 23 || offsetMinute > 59) {
    errors.push(`${path} contains an invalid calendar date, time, or UTC offset`);
    return undefined;
  }
  const offsetMinutes = zone === "Z" ? 0 : (sign === "+" ? 1 : -1) * (offsetHour * 60 + offsetMinute);
  const local = new Date(0);
  local.setUTCFullYear(year, month - 1, day);
  local.setUTCHours(hour, minute, second, 0);
  const utcMillis = local.getTime() - offsetMinutes * 60_000;
  if (!Number.isFinite(utcMillis)) {
    errors.push(`${path} is outside the supported timestamp range`);
    return undefined;
  }
  const unixNano = BigInt(utcMillis) * 1_000_000n + BigInt(fraction.padEnd(9, "0"));
  if (unixNano < 0n || unixNano > OTEL_UINT64_MAX) {
    errors.push(`${path} is outside the OpenTelemetry uint64 nanosecond range`);
    return undefined;
  }
  return { unixNano: unixNano.toString(), canonical: value };
}

export function fromUnixNano(value: string, path: string, errors: string[]): string | undefined {
  if (!/^\d+$/.test(value)) {
    errors.push(`${path} must be an unsigned decimal nanosecond timestamp`);
    return undefined;
  }
  const nanos = BigInt(value);
  if (nanos > OTEL_UINT64_MAX) {
    errors.push(`${path} is outside the OpenTelemetry uint64 nanosecond range`);
    return undefined;
  }
  const millis = nanos / 1_000_000n;
  if (millis > BigInt(8_640_000_000_000_000)) {
    errors.push(`${path} is outside the JavaScript timestamp range`);
    return undefined;
  }
  const base = new Date(Number(millis)).toISOString().slice(0, 19);
  const fraction = (nanos % 1_000_000_000n).toString().padStart(9, "0").replace(/0+$/, "");
  return `${base}${fraction ? `.${fraction}` : ""}Z`;
}
