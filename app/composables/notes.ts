import { decodeTime, ulid } from "ulid";

export interface DailyNote {
  id: string;
  // name?: string;
  datetime: [number, number, number, number, number];
}

export type NoteKind = "daily" | "sticky" | "prelude" | "task";

async function deduplicateNotes(notes: ListRef<DailyNote[]>) {
  const notesValue = notes.value;

  const deleteQueue: number[] = [];
  for (let i = 0; i < notesValue.length; i++) {
    const note = notesValue[i]!;

    for (let j = i + 1; j < notesValue.length; j++) {
      const otherNote = notesValue[j]!;

      if (otherNote.id === note.id && !deleteQueue.includes(j)) {
        deleteQueue.push(j);
      }
    }
  }

  // console.log({ deleteQueue }, notesValue.length);

  for (let i = deleteQueue.length - 1; i >= 0; i--) {
    void notes.delete(deleteQueue[i]!, 1);
  }
}

export async function useDailyNotes(spaceId: MaybeRefOrGetter<string>) {
  const notes = await useStorageList<DailyNote[]>(
    () => `spaces/${toValue(spaceId)}/daily/notes.json`,
  );
  // void deduplicateNotes(notes);

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
