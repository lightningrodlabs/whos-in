export function getDateTime(date: Date): string {
  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()} ${date.getHours()}:${date.getMinutes()}`;
}
export function getDate(date: Date): string {
  return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;
}
export function getTime(date: Date): string {
  return `${date.getHours()}:${date.getMinutes()}`;
}
export function secondsToDateInput(seconds: number): string {
  return new Date(seconds / 1000).toISOString().slice(0, 16);
}
export function dateInputToSeconds(dateInput: string): number {
    return Math.floor(new Date(dateInput).getTime() / 1000);
}