import { createRoot } from "react-dom/client";
import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/code-highlight/styles.css";
import "@mantine/notifications/styles.css";
import "@mantine/spotlight/styles.css";
import "@mantine/carousel/styles.css";
import "@mantine/nprogress/styles.css";
import "mantine-contextmenu/styles.layer.css";

import "./style.scss";
import "./theme/style.css";
import { RouterProvider } from "react-router";
import { router } from "./routes";
import { Localization } from "./lang/provider";
import { MantineProvider } from "@mantine/core";
import { shadcnCssVariableResolver } from "./theme/cssVariableResolver";
import { shadcnTheme } from "./theme/theme";

createRoot(document.getElementById("root")!).render(
    <Localization>
        <MantineProvider
            forceColorScheme="dark"
            theme={shadcnTheme}
            cssVariablesResolver={shadcnCssVariableResolver}
        >
            <RouterProvider router={router} />
        </MantineProvider>
    </Localization>
);
