import { fromAbsolute, isToday, toCalendarDate } from "@internationalized/date";
import { decodeTime, ulid } from "ulid";

export interface DailyNote {
  id: string;
  // title?: string;
  datesReviewed?: string[];
}

export type NoteKind = "daily" | "sticky" | "prelude" | "task";

export async function useDailyNotes(spaceId: MaybeRefOrGetter<string>) {
  const notes = await useStorageMap<{ [id: string]: DailyNote }>(
    () => `spaces/${toValue(spaceId)}/daily/notes.json`,
  );

  return notes;
}

export async function loadDailyNotes(
  spaceId: string,
  notes: MapRef<{ [id: string]: DailyNote }>,
  archived?: boolean,
) {
  let addToday = true;

  const timeZone = useTimeZone();
  const noteEntries = await Promise.all(
    Object.entries(notes.value).map(async ([key, note]) => {
      const date = toCalendarDate(fromAbsolute(decodeTime(note.id), timeZone));

      if (isToday(date, timeZone) && !archived) addToday = false;
      else {
        const item = await getStorageItem<string>(`spaces/${spaceId}/daily/${note.id}.typ`);

        if (!item) return;
      }

      return [key, note];
    }),
  );

  const newNotes = Object.fromEntries(noteEntries.filter((note) => note) as [string, DailyNote][]);

  if (addToday) {
    const id = ulid();

    await notes.set(id, { id, datesReviewed: [] });
    newNotes[id] = { id, datesReviewed: [] };
  }

  return newNotes;
}
