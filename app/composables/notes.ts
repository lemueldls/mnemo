import { decodeTime, ulid } from "ulid";

export interface DailyNote {
  id: string;
  // name?: string;
  datetime: [number, number, number, number, number];
  lastViewed?: [number, number, number, number, number];
}

export type NoteKind = "daily" | "sticky" | "prelude" | "task";

export async function useDailyNotes(spaceId: MaybeRefOrGetter<string>) {
  const notes = await useStorageSet<DailyNote[]>(
    () => `spaces/${toValue(spaceId)}/daily/notes.json`,
    "id",
  );

  return notes;
}

export async function loadDailyNotes(
  spaceId: string,
  notes: DailyNote[],
  archived?: boolean,
) {
  const today = new Date();
  const year = today.getFullYear();
  const month = today.getMonth();
  const date = today.getDate();

  let addToday = true;

  const maybeNotes = await Promise.all(
    notes.map(async (note) => {
      if (
        note.datetime[2] === date &&
        note.datetime[1] === month &&
        note.datetime[0] === year &&
        !archived
      )
        addToday = false;
      else {
        const item = await getStorageItem<string>(
          `spaces/${spaceId}/daily/${note.id}.typ`,
          "",
        );

        if (!item) return;
      }

      return note;
    }),
  );

  const newNotes = maybeNotes.filter((note) => note) as DailyNote[];

  if (addToday) {
    const id = ulid();

    const date = new Date(decodeTime(id));
    const datetime: [number, number, number, number, number] = [
      date.getFullYear(),
      date.getMonth(),
      date.getDate(),
      date.getHours(),
      date.getMinutes(),
    ];

    newNotes.push({ id, datetime });
  }

  return newNotes;
}
