import type { Note } from "~/composables/spaces";
import { ulid, decodeTime } from "ulid";

export async function useSpaceNotes(spaceId: MaybeRefOrGetter<string>) {
  return await useRefStorageItem<Note[]>(
    computed(() => `spaces/${toValue(spaceId)}/daily/notes.json`),
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

export async function loadDailyNotes(notes: Ref<Note[]>) {
  const today = new Date();

  // return computedWithControl(notes, () => {
  let addToday = true;

  for (const note of notes.value) {
    const date = new Date(note.datetime[0], note.datetime[1], note.datetime[2]);
    if (
      date.getFullYear() === today.getFullYear() &&
      date.getMonth() === today.getMonth() &&
      date.getDate() === today.getDate()
    ) {
      addToday = false;

      break;
    }
  }

  if (addToday) {
    addDailyNote(notes);
  }

  return notes.value;
  // });
}
