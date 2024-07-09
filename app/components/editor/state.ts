import init, { TypstState } from "mnemo-wasm";

// eslint-disable-next-line unicorn/prefer-top-level-await
const state = init().then(() => new TypstState());

export const useTypst = () => state;
