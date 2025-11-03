import { today } from "@internationalized/date";
import { decodeTime } from "ulid";

interface ActivityNode {
  date: string;
  activity: number;
}

export const useActivityGraph = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    const activityGraph = await useStorageSet<ActivityNode[]>(
      "activity.json",
      "date",
    );

    setTimeout(async () => {
      if (toValue(amount) < 1) return;

      const spaces = await useSpaces();

      const spaceIds = computed(() => Object.keys(spaces.value));
      const notes = await eagerComputedAsync(() =>
        Promise.all(
          spaceIds.value.map(async (spaceId) => {
            const notes = await useDailyNotes(spaceId);

            return notes.value.map((note) => {
              const createdAt = decodeTime(note.id);

              return { note, createdAt };
            });
          }),
        ),
      );

      const timeZone = useTimeZone();
      const recentActivity = computed(() => {
        const days = toValue(amount);
        let deltaDate = today(timeZone).subtract({ days });

        console.log({ days });

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
          console.log({ notes, recentActivity });

          for (const note of notes) {
            const key = note.createdAt.toString();
            if (recentActivity[key] != undefined) recentActivity[key]++;
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
