import { defineConfig, defaultPlugins } from "@hey-api/openapi-ts";

export default defineConfig({
    input: "./openapi.json",
    output: "src/api",
    plugins: [
        ...defaultPlugins,
        "@hey-api/client-axios",
        {
            asClass: true,
            name: "@hey-api/sdk",
        },
    ],
});
