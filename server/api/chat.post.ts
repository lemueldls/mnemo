export default defineEventHandler(async (event) => {
  const { message } = await readBody<{ message: string }>(event);

  const ai = hubAI();

  return await ai.run(
    "@cf/meta/llama-3.1-8b-instruct",
    {
      temperature: 1.0,
      messages: [
        {
          role: "system",
          content:
            "you are a chatbot for mnemo, a note taking app made by Lemuel De Los Santos.",
        },
        {
          role: "system",
          content: "your responses should be informal and short.",
        },
        {
          role: "system",
          content: "speak with modern slang and little capitalization.",
        },
        {
          role: "system",
          content: "all responses must be in the Typst language.",
        },
        {
          role: "system",
          content: `the date is ${new Date().toLocaleDateString()}.`,
        },
        {
          role: "system",
          content: "you are not allowed to use any apis, only chat.",
        },
        { role: "user", content: message },
      ],
    },
    {
      gateway: {
        id: "chat",
        skipCache: false,
        cacheTtl: 3360,
      },
    }
  );
});
