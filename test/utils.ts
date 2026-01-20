/** Generate a short random ID for test identification */
export function shortId(): string {
  return Math.random().toString(36).substring(2, 8);
}

/** Get current date as YYYY-MM-DD */
export function dateStamp(): string {
  return new Date().toISOString().split("T")[0];
}

/** Generate a test list name with runtime, platform, ID, and date */
export function testListName(runtime: string, target?: string): string {
  const platform = target || "unknown";
  return `CI ${runtime}-${platform} ${shortId()} ${dateStamp()}`;
}
