import { fromAbsolute, isToday, toCalendarDate } from "@internationalized/date";
import { decodeTime, ulid } from "ulid";

export interface DailyNote {
  id: string;
  // title?: string;
  datesReviewed?: string[];
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
  let addToday = true;

  const timeZone = useTimeZone();
  const maybeNotes = await Promise.all(
    notes.map(async (note) => {
      const date = toCalendarDate(fromAbsolute(decodeTime(note.id), timeZone));

      if (isToday(date, timeZone) && !archived) addToday = false;
      else {
        const item = await getStorageItem<string>(
          `spaces/${spaceId}/daily/${note.id}.typ`,
        );

        if (!item) return;
      }

      return note;
    }),
  );

  const newNotes = maybeNotes.filter((note) => note) as DailyNote[];

  if (addToday) {
    const id = ulid();
    // const date = toCalendarDate(fromAbsolute(decodeTime(id), timeZone));

    newNotes.push({ id, datesReviewed: [] });
  }

  return newNotes;
}
