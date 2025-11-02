interface Review {
  spaceId: string;
  noteId: string;
  date: string;
  stage: number;
  lastReviewed: number;
}

export const useReview = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    const { d } = useI18n();

    const review = await useStorageSet<Review[]>("review.json", "noteId");

    setTimeout(async () => {
      const spaces = await useSpaces();

      const spaceIds = computed(() => Object.keys(spaces.value));
      const notes = await eagerComputedAsync(() =>
        Promise.all(
          spaceIds.value.map(async (spaceId) => {
            const notes = await useDailyNotes(spaceId);

            return notes.value
              .map((note) => {
                const {
                  datetime: [year, month, day, hour, minute],
                } = note;

                const createdAt = Date.UTC(year, month, day, hour, minute);

                return { spaceId, note, createdAt };
              })
              .filter(
                ({ createdAt }) => createdAt < Date.now() - 1000 * 60 * 60 * 24,
              );
          }),
        ),
      );

      const resolvedNotes = computed(() =>
        notes.value.flat().sort((a, b) => b.createdAt - a.createdAt),
      );

      return await eagerComputedAsync(async () => {
        const notesToReview = [];
        const notes = resolvedNotes.value;

        for (
          let i = 0;
          i < notes.length && notesToReview.length < toValue(amount);
          i++
        ) {
          const { spaceId, note, createdAt } = notes[i]!;

          const noteId = note.id;
          const date = d(createdAt, {
            weekday: "long",
            month: "long",
            day: "numeric",
          });

          const content = await getStorageItem(
            `spaces/${spaceId}/daily/${noteId}.typ`,
            "",
          );

          if (content)
            notesToReview.push({
              spaceId,
              noteId,
              date,
              stage: 1,
              lastReviewed: Date.now(),
            });
        }

        console.log({ notesToReview });
        review.value = notesToReview;
      });
    }, 2500);

    return review;
  },
);
