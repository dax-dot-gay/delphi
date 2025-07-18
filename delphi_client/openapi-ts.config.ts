import { defineConfig, defaultPlugins } from "@hey-api/openapi-ts";

export default defineConfig({
    input: "./openapi.json",
    output: "src/api",
    plugins: [
        ...defaultPlugins,
        {
            name: "@hey-api/client-axios",
            runtimeConfigPath: "./src/hey-api.ts",
        },
        {
            asClass: true,
            name: "@hey-api/sdk",
        },
    ],
});
