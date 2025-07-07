import { decodeTime, ulid } from "ulid";

export interface Note {
  id: string;
  // name?: string;
  datetime: [number, number, number, number, number];
}

export type NoteKind = "daily" | "sticky" | "prelude";

export async function useSpaceNotes(spaceId: MaybeRefOrGetter<string>) {
  return await useStorageItem<Note[]>(
    () => `spaces/${toValue(spaceId)}/daily/notes.json`,
    [],
  );
}

export function addDailyNote(notes: Ref<Note[]>) {
  const id = ulid();

  const date = new Date(decodeTime(id));
  const datetime: [number, number, number, number, number] = [
    date.getFullYear(),
    date.getMonth(),
    date.getDate(),
    date.getHours(),
    date.getMinutes(),
  ];

  notes.value.unshift({ id, datetime });
}

export async function loadDailyNotes(
  spaceId: string,
  notes: Ref<Note[]>,
  archived: boolean,
) {
  const today = new Date();
  const year = today.getFullYear();
  const month = today.getMonth();
  const date = today.getDate();

  let addToday = true;

  const end = notes.value.length - 1;
  for (let i = end; i >= 0; i--) {
    const note = notes.value[i]!;

    if (
      addToday &&
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
      if (!item) notes.value.splice(i, 1);
    }
  }

  if (addToday) {
    addDailyNote(notes);
  }

  return notes.value;
}
