import { decodeTime, ulid } from "ulid";

export interface Note {
  id: string;
  // name?: string;
  datetime: [number, number, number, number, number];
}

export type NoteKind = "daily" | "sticky" | "prelude" | "task";

export async function useSpaceNotes(spaceId: MaybeRefOrGetter<string>) {
  return await useStorageList<Note[]>(
    () => `spaces/${toValue(spaceId)}/daily/notes.json`,
  );
}

export async function loadDailyNotes(
  spaceId: string,
  notes: ListRef<Note[]>,
  archived?: boolean,
) {
  const today = new Date();
  const year = today.getFullYear();
  const month = today.getMonth();
  const date = today.getDate();

  const rawNotes = notes.value;

  let addToday = true;

  // for (const note of rawNotes) {
  //   if (
  //     addToday &&
  //     note.datetime[2] === date &&
  //     note.datetime[1] === month &&
  //     note.datetime[0] === year &&
  //     !archived
  //   )
  //     addToday = false;
  // }

  const maybeNotes = await Promise.all(
    rawNotes.map(async (note, i) => {
      if (
        note.datetime[2] === date &&
        note.datetime[1] === month &&
        note.datetime[0] === year &&
        !archived
      )
        addToday = false;
      // else {
      //   const item = await getStorageItem<string>(
      //     `spaces/${spaceId}/daily/${note.id}.typ`,
      //     "",
      //   );
      //   console.log({ item });

      //   if (!item) return;
      // }

      return note;
    }),
  );
  // const filteredNotes = maybeNotes.filter((note) => note);

  // void (async () => {
  //   const end = rawNotes.length - 1;
  //   for (let i = end; i >= 0; i--) {
  //     const note = rawNotes[i]!;

  //     const item = await getStorageItem<string>(
  //       `spaces/${spaceId}/daily/${note.id}.typ`,
  //       "",
  //     );
  //     console.log({ item });
  //     if (!item) notes.delete(i, 1);
  //   }
  // })();

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

    notes.push({ id, datetime });
  }

  return notes.value;
}
