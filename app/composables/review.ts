export const useReview = createSharedComposable(
  async (amount: MaybeRefOrGetter<number>) => {
    // return [];

    const { d } = useI18n();

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

        const date = d(createdAt, {
          weekday: "long",
          month: "long",
          day: "numeric",
        });

        const content = await getStorageItem(
          `spaces/${spaceId}/daily/${note.id}.typ`,
          "",
        );

        if (content)
          notesToReview.push({
            spaceId,
            note,
            date,
            stage: 1,
            lastReviewed: Date.now(),
          });
      }

      return notesToReview;
    });
  },
);
