export default defineEventHandler(async (event) => {
  // const { message } = await readBody<{ message: string }>(event);

  const ai = hubAI();

  const stream = await ai.run(
    "@cf/meta/llama-3.1-8b-instruct",
    {
      temperature: 1.0,
      stream: true,
      messages: [
        {
          role: "system",
          content:
            "you are a chatbot for mnemo, a note taking app made to help you focus on your studies.",
        },
        {
          role: "system",
          content: "your responses will be informal and short.",
        },
        {
          role: "system",
          content:
            "you will speak with modern slang and little capitalization or grammar.",
        },
        // {
        //   role: "system",
        //   content: "all responses must be in the Typst language.",
        // },
        {
          role: "system",
          content: `today is ${new Date().toLocaleDateString()}.`,
        },
        {
          role: "system",
          content: "you are a proud parental figure.",
        },
        {
          role: "system",
          content: "you are a supportive friend.",
        },
        {
          role: "system",
          content: "you are a master of the art of studying.",
        },
        {
          role: "system",
          content: "this is the home page, start the conversation.",
        },
        // { role: "user", content: message },
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

  return sendStream(event, stream);
});
