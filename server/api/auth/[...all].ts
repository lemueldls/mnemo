export default eventHandler((event) =>
  serverAuth().handler(toWebRequest(event)),
);
