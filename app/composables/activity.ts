import { fromAbsolute, getDayOfWeek, toCalendarDate, today } from "@internationalized/date";
import { decodeTime } from "ulid";

interface ActivityNode {
  date: string;
  activity: number;
}

export const useActivityGraph = createSharedComposable(async (amount: MaybeRefOrGetter<number>) => {
  const { locale } = useSharedI18n();

  const activityGraph = await useStorageItem<ActivityNode[]>("activity.json", []);

  setTimeout(async () => {
    const spaces = await useSpaces();
    const spaceIds = computed(() => Object.keys(spaces.value));

    const timeZone = useTimeZone();

    const notes = await eagerComputedAsync(() =>
      Promise.all(
        spaceIds.value.map(async (spaceId) => {
          const notes = await useDailyNotes(spaceId);

          return notes.value.map((note) => {
            const createdAt = toCalendarDate(fromAbsolute(decodeTime(note.id), timeZone));

            return { note, createdAt };
          });
        }),
      ),
    );

    const { startWeekday, endWeekday } = useWeekdays();
    const recentActivity = computed<Record<string, number>>(() => {
      const days = toValue(amount);

      let deltaDate = today(timeZone).add({ days: 1 });

      const start = startWeekday.value;
      const end = endWeekday.value;

      return Object.fromEntries(
        Array.from({ length: days })
          .map(() => {
            deltaDate = deltaDate.subtract({ days: 1 });

            const dayOfWeek = getDayOfWeek(deltaDate, locale.value);

            const startDelta = start - dayOfWeek;
            if (startDelta > 0) deltaDate = deltaDate.subtract({ days: 2 });

            const endDelta = dayOfWeek - end;
            if (endDelta > 0) deltaDate = deltaDate.subtract({ days: 1 });

            return [deltaDate.toString(), 0];
          })
          .reverse(),
      );
    });

    watchImmediate([() => notes.value.flat(), recentActivity], ([notes, recentActivity]) => {
      for (const { note, createdAt } of notes) {
        const date = createdAt.toString();
        if (recentActivity[date] !== undefined) recentActivity[date]++;

        if (note.datesReviewed?.length) {
          for (const date of note.datesReviewed)
            if (recentActivity[date] !== undefined) recentActivity[date]++;
        }
      }

      activityGraph.value = Object.entries(recentActivity).map(([date, activity]) => ({
        date,
        activity,
      }));
    });
  }, 1000);

  return activityGraph;
});
