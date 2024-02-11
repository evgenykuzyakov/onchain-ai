const { OpenAI } = require("openai");

class AI {
  constructor(logger) {
    this.logger = logger;

    this.openai = new OpenAI({
      baseURL: process.env.OPENAI_BASE_URL || undefined,
      apiKey: process.env.OPENAI_API_KEY,
    });

    this.model = process.env.OPENAI_MODEL || "gpt-3.5-turbo";
    this.systemPrompt =
      process.env.OPENAI_SYSTEM_PROMPT || "You are a helpful assistant.";
    this.jsonSystemPrompt =
      process.env.OPENAI_SYSTEM_PROMPT_JSON ||
      "You are a helpful assistant that outputs in JSON.";
  }

  async complete(
    text,
    systemPrompt = undefined,
    temperature = undefined,
    jsonSchema = undefined
  ) {
    const completion = await this.openai.chat.completions.create({
      model: this.model,
      messages: [
        {
          role: "system",
          content:
            systemPrompt ||
            (jsonSchema ? this.jsonSystemPrompt : this.systemPrompt),
        },
        { role: "user", content: text },
      ],
      temperature: temperature ?? 0.7,
      response_format: jsonSchema
        ? {
            type: "json_object",
            schema: jsonSchema,
          }
        : undefined,
    });

    return completion.choices[0].message.content;
  }
}

module.exports = AI;
