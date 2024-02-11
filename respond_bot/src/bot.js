class Bot {
  constructor({ logger, near, ai }) {
    this.logger = logger;
    this.near = near;
    this.ai = ai;
    this.sleepMs = parseFloat(process.env.SLEEP_MS) || 250;
    this.requests = new Map();
  }

  async processRequest(id, request) {
    this.logger.info("Processing request", { id, request });
    try {
      const result = await this.ai.complete(
        request.text,
        request.system,
        request.temperature,
        request.json_schema ? JSON.parse(request.json_schema) : undefined
      );
      this.logger.info("AI result", { id, result });
      await this.near.contract.respond({
        request_id: id,
        response: { ok: true, text: result },
      });
    } catch (e) {
      this.logger.error("Error processing request", { id, e });
      await this.near.contract.respond({
        request_id: id,
        response: { ok: false },
      });
    }
  }

  async processRequests() {
    const requests = await this.near.contract.get_all_requests();
    if (requests.length === 0) {
      return;
    }
    for (const [id, request] of requests) {
      if (this.requests.has(id)) {
        continue;
      }
      this.requests.set(id, request);
      this.processRequest(id, request);
    }
  }

  async run() {
    this.logger.info("Bot is running");
    while (true) {
      await this.processRequests();
      await new Promise((resolve) => setTimeout(resolve, this.sleepMs));
    }
  }
}

module.exports = Bot;
