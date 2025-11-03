import { decodeTime } from "ulid";

interface Review {
  spaceId: string;
  noteId: string;
  date: string;
  stage: number;
}

export const useReview = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    const { d } = useI18n();

    const review = await useStorageSet<Review[]>("review.json", "noteId");

    setTimeout(async () => {
      if (toValue(amount) < 1) return;

      const spaces = await useSpaces();

      // const timeZone = useTimeZone();
      const yesterday = Date.now() - 1000 * 60 * 60 * 24;

      const spaceIds = computed(() => Object.keys(spaces.value));
      const notes = await eagerComputedAsync(() => {
        const max = toValue(amount);

        return Promise.all(
          spaceIds.value.map(async (spaceId) => {
            const dailyNotes = await useDailyNotes(spaceId);
            const notes = dailyNotes.value;

            const notesToReview = [];

            const end = notes.length - 1;
            for (let i = end; i >= 0 && notesToReview.length < max; i--) {
              const note = notes[i]!;

              const createdAt = decodeTime(note.id);

              if (createdAt < yesterday) {
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

                  notesToReview.push({
                    spaceId,
                    noteId,
                    date,
                    stage: 1,
                  });
                }
              }
            }

            return notesToReview;
          }),
        );
      });

      watchImmediate(notes, (notes) => {
        review.value = notes.flat();
      });
    }, 250);

    return review;
  },
);
