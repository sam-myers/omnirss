import { defineConfig } from 'cypress'

export default defineConfig({
  projectId: "srh5kf",

  e2e: {
    baseUrl: "http://localhost:8888",
    supportFile: false,
  },
})
