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
import { Suspender } from "./Suspender";

createRoot(document.getElementById("root")!).render(<Suspender />);
