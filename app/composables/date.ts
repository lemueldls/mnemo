export function useShortDate(date: Date) {
  const { d } = useI18n();

  return d(date, { month: "short", weekday: "short", day: "numeric" });
}

export function useLongDate(date: Date) {
  const { d } = useI18n();

  return d(date, { weekday: "long", month: "long", day: "numeric" });
}
