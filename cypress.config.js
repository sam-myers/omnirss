const { defineConfig } = require("cypress");

module.exports = defineConfig({
  projectId: "srh5kf",

  e2e: {
    baseUrl: "http://localhost:8888",
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
});
