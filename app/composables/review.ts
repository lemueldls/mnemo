import { CalendarDate, parseDate } from "@internationalized/date";
import { decodeTime } from "ulid";

export interface Review {
  spaceId: string;
  noteId: string;
  date: string;
  stage: number;
  lastReviewed?: number;
}

const REVIEW_STAGES = [
  1 * 24 * 60 * 60 * 1000, // 1 day
  2 * 24 * 60 * 60 * 1000, // 2 days
  4 * 24 * 60 * 60 * 1000, // 4 days
  6 * 24 * 60 * 60 * 1000, // 6 days
];

export const useReviewStages = () => REVIEW_STAGES;

export const useReview = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    const { d } = useSharedI18n();
    const spaces = await useSpaces();

    const review = await useStorageItem<Review[]>("review.json", []);

    const today = Date.now();
    const yesterday = today - 1000 * 60 * 60 * 24;

    const spaceIds = computed(() => Object.keys(spaces.value));
    const spaceNotes = await eagerComputedAsync(() =>
      Promise.all(
        spaceIds.value.map(async (spaceId) => {
          const dailyNotes = await useDailyNotes(spaceId);
          const notes = dailyNotes.value;

          return { spaceId, notes };
        }),
      ),
    );

    watchDebounced(
      [amount, spaceNotes],
      async (results) => {
        const [max, spaceNotes] = results as [
          number,
          { spaceId: string; notes: DailyNote[] }[],
        ];

        if (max < 1) return [];

        const notesToReview: Review[] = [];

        await Promise.all(
          spaceNotes.map(async ({ spaceId, notes }) => {
            const end = notes.length - 1;
            for (let i = end; i >= 0 && notesToReview.length < max; i--) {
              const note = notes[i]!;

              const datesReviewed = note.datesReviewed || [];

              const createdAt = decodeTime(note.id);
              if (createdAt > yesterday) continue;

              const lastDateReviewed = datesReviewed.at(-1);
              const timeZone = useTimeZone();
              const lastReviewed = lastDateReviewed
                ? parseDate(lastDateReviewed).toDate(timeZone).getTime()
                : undefined;

              const stage = getReviewStage(datesReviewed);

              // if (lastReviewed && lastReviewed > today - REVIEW_STAGES[stage]!)
              //   continue;

              const noteId = note.id;

              const content = await getStorageItem(
                `spaces/${spaceId}/daily/${noteId}.typ`,
                "",
              );

              if (content) {
                const date = d(createdAt, {
                  weekday: "long",
                  month: "long",
                  day: "numeric",
                });

                if (notesToReview.length < max)
                  notesToReview.push({
                    spaceId,
                    noteId,
                    date,
                    stage,
                    lastReviewed,
                  });
              }
            }
          }),
        );

        review.value = notesToReview.flat();
      },
      { debounce: 500 },
    );

    return review;
  },
);

export function getReviewStage(dates: string[]) {
  const timeZone = useTimeZone();

  const { stage } = dates.reduce(
    (state, date) => {
      const calendarDate = parseDate(date);
      const time = calendarDate.toDate(timeZone).getTime();

      if (state.lastReviewed) {
        if (state.stage < REVIEW_STAGES.length) {
          const lastTime = state.lastReviewed.toDate(timeZone).getTime();

          if (time - lastTime >= REVIEW_STAGES[state.stage]!)
            return { stage: state.stage + 1, lastReviewed: calendarDate };
        }

        return state;
      }

      return { stage: 0, lastReviewed: calendarDate };
    },
    { stage: 0, lastReviewed: undefined as CalendarDate | undefined },
  );

  return stage;
}
