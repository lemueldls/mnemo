export const useReview = createSharedComposable(async () => {
  // return [];

  const { d } = useI18n();

  const spaces = await useSpaces();
  const spaceIds = Object.keys(spaces.value);
  const notes = await Promise.all(
    spaceIds.map(async (spaceId) => {
      const notes = await useDailyNotes(spaceId);

      return notes.value
        .map((note) => {
          const {
            datetime: [year, month, day, hour, minute],
          } = note;

          const createdAt = Date.UTC(year, month, day, hour, minute);

          return { spaceId, note, createdAt };
        })
        .filter(({ createdAt }) => createdAt < Date.now() - 1000 * 60 * 60 * 24)
        .sort((a, b) => b.createdAt - a.createdAt);
    }),
  );

  const allNotes = notes.flat();
  const notesToReview = [];

  for (let i = 0; i < allNotes.length && notesToReview.length < 6; i++) {
    const { spaceId, note, createdAt } = allNotes[i]!;

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
