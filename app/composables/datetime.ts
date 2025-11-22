import { getLocalTimeZone } from "@internationalized/date";

export const useTimeZone = createSharedComposable(() => getLocalTimeZone());

export function useShortDate(date: Date) {
  const { d } = useSharedI18n();

  return d(date, { month: "short", weekday: "short", day: "numeric" });
}

export function useLongDate(date: Date) {
  const { d } = useSharedI18n();

  return d(date, { weekday: "long", month: "long", day: "numeric" });
}

export function useRelativeTime(time: number) {
  const now = useNow({ interval: 60000 });

  const { locale } = useSharedI18n();
  const relativeTimeFormat = new Intl.RelativeTimeFormat(locale.value, {
    numeric: "auto",
  });

  return computed(() => {
    const milliseconds = now.value.getTime() - time;
    const seconds = Math.floor(milliseconds / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);
    const days = Math.floor(hours / 24);

    return relativeTimeFormat.format(-days, "days");

    // if (days > 0) return relativeTimeFormat.format(-days, "days");
    // else if (hours > 0) return relativeTimeFormat.format(-hours, "hours");
    // else if (minutes > 0) return relativeTimeFormat.format(-minutes, "minutes");
    // else return relativeTimeFormat.format(-seconds, "seconds");
  });
}

export const useWeekdays = createSharedComposable(() => {
  const showWeekends = useCookie("show-weekends", { default: () => false });
  const startWeekday = computed(() => (showWeekends.value ? 0 : 1));
  const endWeekday = computed(() => (showWeekends.value ? 7 : 6));
  const totalWeekdays = computed(() => endWeekday.value - startWeekday.value);

  return { showWeekends, startWeekday, endWeekday, totalWeekdays };
});
