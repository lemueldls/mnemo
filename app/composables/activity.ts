import {
  fromAbsolute,
  getLocalTimeZone,
  toCalendarDate,
  today,
} from "@internationalized/date";

interface ActivityNode {
  date: string;
  activity: number;
}

export const useActivity = createSharedComposable(async () => {
  const activity = await useStorageSet<ActivityNode[]>("activity.json", "date");

  setTimeout(async () => {
    const spaces = await useSpaces();

    const spaceIds = computed(() => Object.keys(spaces.value));
    const notes = await eagerComputedAsync(() =>
      Promise.all(
        spaceIds.value.map(async (spaceId) => {
          const notes = await useDailyNotes(spaceId);

          return notes.value.map((note) => {
            const {
              datetime: [year, month, day, hour, minute],
            } = note;

            const createdAt = Date.UTC(year, month, day, hour, minute);

            return { spaceId, note, createdAt };
          });
        }),
      ),
    );

    const resolvedNotes = computed(() =>
      notes.value.flat().sort((a, b) => b.createdAt - a.createdAt),
    );

    const timeZone = getLocalTimeZone();

    watchImmediate(resolvedNotes, (notes) => {
      const days = 259;
      let deltaDate = today(timeZone).subtract({ days });

      const recentActivity = Object.fromEntries(
        Array.from({ length: days }).map((_, days) => {
          const date = deltaDate.add({ days });

          return [date.toString(), 0];
        }),
      );

      for (const note of notes) {
        const date = toCalendarDate(fromAbsolute(note.createdAt, timeZone));
        const key = date.toString();
        if (recentActivity[key] != undefined) recentActivity[key]++;
      }

      activity.value = Object.entries(recentActivity).map(
        ([date, activity]) => ({ date, activity }),
      );
    });
  }, 2500);

  return activity;
});
