import { fromAbsolute, toCalendarDate, today } from "@internationalized/date";
import { decodeTime } from "ulid";

interface ActivityNode {
  date: string;
  activity: number;
}

export const useActivityGraph = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    const activityGraph = await useStorageItem<ActivityNode[]>(
      "activity.json",
      [],
    );

    setTimeout(async () => {
      if (toValue(amount) < 1) return;

      const spaces = await useSpaces();
      const spaceIds = computed(() => Object.keys(spaces.value));

      const timeZone = useTimeZone();

      const notes = await eagerComputedAsync(() =>
        Promise.all(
          spaceIds.value.map(async (spaceId) => {
            const notes = await useDailyNotes(spaceId);

            return notes.value.map((note) => {
              const createdAt = toCalendarDate(
                fromAbsolute(decodeTime(note.id), timeZone),
              );

              return { note, createdAt };
            });
          }),
        ),
      );

      const recentActivity = computed(() => {
        const days = toValue(amount);
        let deltaDate = today(timeZone).subtract({ days });

        return Object.fromEntries(
          Array.from({ length: days }).map((_, days) => {
            const date = deltaDate.add({ days });

            return [date.toString(), 0];
          }),
        );
      });

      watchImmediate(
        [() => notes.value.flat(), recentActivity],
        ([notes, recentActivity]) => {
          for (const { note, createdAt } of notes) {
            const date = createdAt.toString();
            if (recentActivity[date] !== undefined) recentActivity[date]++;

            if (note.datesReviewed?.length)
              for (const date of note.datesReviewed) {
                if (recentActivity[date] !== undefined) recentActivity[date]++;
              }
          }

          activityGraph.value = Object.entries(recentActivity).map(
            ([date, activity]) => ({ date, activity }),
          );
        },
      );
    }, 500);

    return activityGraph;
  },
);
