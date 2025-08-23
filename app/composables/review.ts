export const useReview = createSharedComposable(async () => {
  // return [];

  const { d } = useI18n();

  const spaces = await useSpaces();
  const spaceIds = Object.keys(spaces.value);
  const notes = await Promise.all(
    spaceIds.map(async (spaceId) => {
      const notes = await useSpaceNotes(spaceId);

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

  return notes
    .flat()
    .slice(0, 3)
    .map(({ spaceId, note, createdAt }) => {
      const date = d(createdAt, {
        weekday: "long",
        month: "long",
        day: "numeric",
      });

      return { spaceId, note, date, stage: 1, lastReviewed: Date.now() };
    });
});
