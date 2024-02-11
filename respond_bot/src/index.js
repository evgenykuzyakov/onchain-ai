require("dotenv").config();
const winston = require("winston");

const { loadJson, saveJson } = require("./utils");
const { initNear } = require("./near");
const AI = require("./ai");
const Bot = require("./bot");

const StateFilename = "state/" + (process.env.STATE_FILENAME || "state.json");

const DefaultState = {};

(async () => {
  const logger = winston.createLogger({
    level: "info",
    format: winston.format.combine(
      winston.format.errors({ stack: true }),
      winston.format.timestamp(),
      winston.format.json()
    ),
    transports: [
      new winston.transports.File({
        filename: `logs/${process.env.LOG_PREFIX}error.log`,
        level: "error",
      }),
      new winston.transports.File({
        filename: `logs/${process.env.LOG_PREFIX}combined.log`,
      }),
    ],
  });

  if (process.env.NODE_ENV !== "production") {
    logger.add(
      new winston.transports.Console({
        format: winston.format.simple(),
      })
    );
  }

  const state = Object.assign(DefaultState, loadJson(StateFilename, true));
  const near = await initNear(logger.child({ type: "near" }));
  const ai = new AI(logger.child({ type: "ai" }));
  const bot = new Bot({ logger, near, ai });

  try {
    await bot.run();
  } catch (error) {
    logger.error(error);
  } finally {
    saveJson(state, StateFilename);
  }
})();
